use actix_web::web;
use actix_web::HttpResponse;
use sqlx::PgConnection;

#[allow(unused)]
#[derive(serde::Deserialize)]
pub struct JsonData {
    debtor: String,
    creditor: String,
    amount: f64,
    currency: String,
}

pub async fn create_debt(
    _body: web::Json<JsonData>,
    _connection: web::Data<PgConnection>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}
