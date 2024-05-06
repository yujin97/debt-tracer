use actix_web::web;
use actix_web::HttpResponse;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct JsonData {
    debtor: String,
    creditor: String,
    amount: f64,
    currency: String,
}

#[derive(serde::Deserialize)]
pub struct QueryData {
    user_id: String,
}

pub async fn create_debt(body: web::Json<JsonData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let creditor = Uuid::parse_str(&body.creditor);

    if creditor.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let debtor = Uuid::parse_str(&body.debtor);

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
) -> HttpResponse {
    let user_id = Uuid::parse_str(&query_string.user_id).expect("Failed to parse UUID");
    let pool = db_pool.as_ref();

    let _ = sqlx::query!(
        "SELECT * FROM debts \
        WHERE creditor_id = $1 OR debtor_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await;

    HttpResponse::Ok().finish()
}
