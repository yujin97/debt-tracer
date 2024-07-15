use crate::authentication::UserId;
use crate::domain::{DebtAmount, DebtCurrency, DebtDescription, DebtStatus, DebtUserId, NewDebt};
use crate::utils::error_chain_fmt;
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::ResponseError;
use anyhow::Context;
use chrono::Utc;
use rust_decimal::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::utils::e500;

#[derive(serde::Deserialize)]
pub struct JsonData {
    debtor_id: String,
    creditor_id: String,
    amount: f64,
    currency: String,
    description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateDebtJSONResponse {
    debt_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GetDebtJSONResponse {
    pub debt_id: String,
    pub creditor_id: String,
    pub creditor_name: String,
    pub debtor_id: String,
    pub debtor_name: String,
    pub amount: f64,
    pub currency: String,
    pub description: String,
    pub status: String,
    pub created_at: String,
}

impl TryFrom<JsonData> for NewDebt {
    type Error = String;

    fn try_from(json_data: JsonData) -> Result<Self, Self::Error> {
        let debtor_id = DebtUserId::parse(&json_data.debtor_id)?;
        let creditor_id = DebtUserId::parse(&json_data.creditor_id)?;
        let amount = DebtAmount::parse(json_data.amount)?;
        let currency = DebtCurrency::parse(json_data.currency)?;
        let description = DebtDescription::parse(json_data.description)?;

        Ok(Self {
            debtor_id,
            creditor_id,
            amount,
            currency,
            description,
            status: DebtStatus::Pending,
        })
    }
}

#[tracing::instrument(
    name= "Creating a debt",
    skip(body, db_pool),
    fields(
        creditor_id = %body.creditor_id,
        debtor_id =%body.debtor_id,
        amount = %body.amount,
        currency = %body.currency,
    )
)]
pub async fn create_debt(
    body: web::Json<JsonData>,
    db_pool: web::Data<PgPool>,
) -> Result<web::Json<CreateDebtJSONResponse>, CreateDebtError> {
    let new_debt: NewDebt = body
        .0
        .try_into()
        .map_err(CreateDebtError::ValidationError)?;

    let debt_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO debts (debt_id, creditor_id, debtor_id, amount, currency, description, status, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        debt_id.clone(),
        new_debt.creditor_id.as_ref(),
        new_debt.debtor_id.as_ref(),
        new_debt.amount.as_ref(),
        new_debt.currency.to_string(),
        new_debt.description.as_ref(),
        new_debt.status.to_string(),
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    .context("Failed to insert new debt into the database.")?;

    let res = CreateDebtJSONResponse {
        debt_id: debt_id.to_string(),
    };
    Ok(web::Json(res))
}

#[tracing::instrument(name = "Getting list of debts by User ID", skip(db_pool))]
pub async fn get_debts_by_user_id(
    user_id: web::ReqData<UserId>,
    db_pool: web::Data<PgPool>,
) -> Result<web::Json<Vec<GetDebtJSONResponse>>, actix_web::Error> {
    let user_id = *user_id.into_inner();
    let pool = db_pool.as_ref();

    let result = sqlx::query!(
        "SELECT debt_id, users_1.user_id as creditor_id, users_1.username as creditor_name, \
        users_2.user_id as debtor_id, users_2.username as debtor_name, amount, currency, description, status, created_at \
        FROM debts JOIN users users_1 ON debts.creditor_id =  users_1.user_id \
        JOIN users users_2 ON debts.debtor_id = users_2.user_id \
        WHERE creditor_id = $1 OR debtor_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch debts from the database.")
    .context("Internal Server Error");

    match result {
        Ok(result) => {
            let result = result
                .into_iter()
                .map(|row| GetDebtJSONResponse {
                    debt_id: row.debt_id.to_string(),
                    creditor_id: row.creditor_id.to_string(),
                    creditor_name: row.creditor_name,
                    debtor_id: row.debtor_id.to_string(),
                    debtor_name: row.debtor_name,
                    // temporarily unwrap this value
                    amount: row.amount.to_f64().expect("Failed to convert big decimal"),
                    description: row.description,
                    currency: row.currency,
                    status: row.status,
                    created_at: row.created_at.to_string(),
                })
                .collect::<Vec<_>>();
            Ok(web::Json(result))
        }
        Err(e) => Err(e500(e)),
    }
}

#[derive(thiserror::Error)]
pub enum CreateDebtError {
    #[error("{0}")]
    ValidationError(String),
    #[error("Internal Server Error")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for CreateDebtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CreateDebtError {
    fn status_code(&self) -> StatusCode {
        match self {
            CreateDebtError::ValidationError(_) => StatusCode::BAD_REQUEST,
            CreateDebtError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
