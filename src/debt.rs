use actix_web::web;
use actix_web::HttpResponse;

#[allow(unused)]
#[derive(serde::Deserialize)]
pub struct JsonData {
    debtor: String,
    creditor: String,
    amount: f64,
    currency: String,
}

pub async fn create_debt(_body: web::Json<JsonData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
