use actix_web::web;
use actix_web::HttpResponse;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use chrono::Utc;
use sqlx::PgConnection;
use uuid::Uuid;

#[allow(unused)]
#[derive(serde::Deserialize)]
pub struct JsonData {
    debtor: String,
    creditor: String,
    amount: f64,
    currency: String,
}

pub async fn create_debt(
    body: web::Json<JsonData>,
    connection: web::Data<PgConnection>,
) -> HttpResponse {
    let creditor = Uuid::parse_str(&body.creditor);

    if let Err(_) = creditor {
        return HttpResponse::InternalServerError().finish();
    }

    let debtor = Uuid::parse_str(&body.debtor);

    if let Err(_) = debtor {
        return HttpResponse::InternalServerError().finish();
    }

    let amount = BigDecimal::from_f64(body.amount);

    if let None = amount {
        return HttpResponse::InternalServerError().finish();
    }

    sqlx::query!(
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
    .execute(connection.get_ref());

    HttpResponse::Ok().finish()
}
