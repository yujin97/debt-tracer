use crate::domain::NewUser;
use crate::utils::error_chain_fmt;
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use anyhow::Context;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct SignUpJsonRequestBody {
    username: String,
    password: String,
    email: String,
}

impl TryFrom<SignUpJsonRequestBody> for NewUser {
    type Error = anyhow::Error;

    fn try_from(json_data: SignUpJsonRequestBody) -> Result<Self, Self::Error> {
        Ok(NewUser::new(
            json_data.username,
            json_data.password,
            json_data.email,
        ))
    }
}

#[derive(thiserror::Error)]
pub enum SignUpError {
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for SignUpError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl std::fmt::Debug for SignUpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[tracing::instrument(
    name = "Signing up for user",
    skip(body, db_pool),
    fields(
        username = %body.username,
        email = %body.email
    )
)]
pub async fn sign_up(
    body: web::Json<SignUpJsonRequestBody>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, SignUpError> {
    let new_user: NewUser = body.0.try_into().map_err(SignUpError::UnexpectedError)?;

    sqlx::query!(
        r#"
        INSERT INTO users (user_id, username, password_hash, email)
        VALUES ($1, $2, $3, $4)
        "#,
        new_user.user_id,
        new_user.username,
        new_user.password_hash,
        new_user.email
    )
    .execute(db_pool.get_ref())
    .await
    .context("Failed to insert new user into the database")?;

    Ok(HttpResponse::Ok().finish())
}
