use crate::authentication::{validate_credentials, AuthError, Credentials, UserInfo};
use crate::session_state::TypedSession;
use crate::utils::error_chain_fmt;
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::{HttpResponse, ResponseError};
use secrecy::Secret;
use sqlx::PgPool;

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(serde::Deserialize)]
pub struct JsonLoginData {
    username: String,
    password: Secret<String>,
}

#[tracing::instrument(
    skip(body, db_pool, session),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn login(
    body: web::Json<JsonLoginData>,
    db_pool: web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        username: body.username.clone(),
        password: body.password.clone(),
    };
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
    match validate_credentials(credentials, &db_pool).await {
        Ok(UserInfo { user_id, username }) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            tracing::Span::current().record("ussername", &tracing::field::display(&username));
            session.renew();
            session.insert_user_id(user_id).map_err(|e| {
                InternalError::from_response(
                    LoginError::UnexpectedError(e.into()),
                    HttpResponse::InternalServerError().finish(),
                )
            })?;
            session.insert_username(username).map_err(|e| {
                InternalError::from_response(
                    LoginError::UnexpectedError(e.into()),
                    HttpResponse::InternalServerError().finish(),
                )
            })?;
            Ok(HttpResponse::Ok().finish())
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            Err(InternalError::from_response(
                e,
                HttpResponse::InternalServerError().finish(),
            ))
        }
    }
}
