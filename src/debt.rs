use actix_web::web;
use actix_web::HttpResponse;
use bigdecimal::FromPrimitive;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::Utc;
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
}

#[derive(serde::Deserialize)]
pub struct QueryData {
    user_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DebtJSONResponse {
    pub debt_id: String,
    pub creditor_id: String,
    pub creditor_name: String,
    pub debtor_id: String,
    pub debtor_name: String,
    pub amount: f64,
    pub currency: String,
}

pub async fn create_debt(body: web::Json<JsonData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let creditor = Uuid::parse_str(&body.creditor_id);

    if creditor.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let debtor = Uuid::parse_str(&body.debtor_id);

    if debtor.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let amount = BigDecimal::from_f64(body.amount);

    if amount.is_none() {
        return HttpResponse::InternalServerError().finish();
    }

    let _ = sqlx::query!(
        r#"
        INSERT INTO debts (debt_id, creditor_id, debtor_id, amount, currency, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        creditor.unwrap(),
        debtor.unwrap(),
        amount.unwrap(),
        body.currency,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await;

    HttpResponse::Ok().finish()
}

pub async fn get_debts_by_user_id(
    query_string: web::Query<QueryData>,
    db_pool: web::Data<PgPool>,
) -> Result<web::Json<Vec<DebtJSONResponse>>, actix_web::Error> {
    let user_id = Uuid::parse_str(&query_string.user_id).expect("Failed to parse UUID");
    let pool = db_pool.as_ref();

    let result = sqlx::query!(
        "SELECT debt_id, users_1.user_id as creditor_id, users_1.username as creditor_name, \
        users_2.user_id as debtor_id, users_2.username as debtor_name, amount, currency
        FROM debts JOIN users users_1 ON debts.creditor_id =  users_1.user_id
        JOIN users users_2 ON debts.debtor_id = users_2.user_id
        WHERE creditor_id = $1 OR debtor_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await;

    let response = match result {
        Ok(result) => {
            let result = result
                .into_iter()
                .map(|row| DebtJSONResponse {
                    debt_id: row.debt_id.to_string(),
                    creditor_id: row.creditor_id.to_string(),
                    creditor_name: row.creditor_name,
                    debtor_id: row.debtor_id.to_string(),
                    debtor_name: row.debtor_name,
                    // temporarily unwrap this value
                    amount: row.amount.to_f64().expect("Failed to convert big decimal"),
                    currency: row.currency,
                })
                .collect::<Vec<_>>();
            Ok(web::Json(result))
        }
        Err(e) => Err(e500(e)),
    };

    response
}
