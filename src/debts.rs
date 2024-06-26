use crate::authentication::UserId;
use actix_web::web;
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
) -> Result<web::Json<CreateDebtJSONResponse>, actix_web::Error> {
    let creditor = Uuid::parse_str(&body.creditor_id);

    if creditor.is_err() {
        return Err(actix_web::error::ErrorInternalServerError(
            "Invalid creditor ID",
        ));
    }

    let debtor = Uuid::parse_str(&body.debtor_id);

    if debtor.is_err() {
        return Err(actix_web::error::ErrorInternalServerError(
            "Invalid debtor ID",
        ));
    }

    let amount = Decimal::from_f64(body.amount);

    if amount.is_none() {
        return Err(actix_web::error::ErrorInternalServerError("Invalid amount"));
    }

    let debt_id = Uuid::new_v4();

    let response = sqlx::query!(
        r#"
        INSERT INTO debts (debt_id, creditor_id, debtor_id, amount, currency, description, status, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        debt_id.clone(),
        creditor.unwrap(),
        debtor.unwrap(),
        amount.unwrap(),
        body.currency,
        body.description,
        "pending",
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await;

    match response {
        Ok(_) => {
            let res = CreateDebtJSONResponse {
                debt_id: debt_id.to_string(),
            };
            Ok(web::Json(res))
        }
        Err(e) => Err(e500(e)),
    }
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
    .await;

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
