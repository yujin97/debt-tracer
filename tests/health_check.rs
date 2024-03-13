use bigdecimal::ToPrimitive;
use debt_tracer::configuration::get_configuration;
use sqlx::{Connection, PgConnection};
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::test]
async fn health_check_works() {
    let app_address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app_address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn create_debt_returns_a_200_for_valid_json_data() {
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let client = reqwest::Client::new();

    let debtor_id = Uuid::new_v4().to_string();
    let creditor_id = Uuid::new_v4().to_string();

    let create_debt_request = CreateDebtRequest {
        debtor: debtor_id.clone(),
        creditor: creditor_id.clone(),
        amount: 3000.0,
        currency: "JPY".to_string(),
    };

    let response = client
        .post(&format!("{}/debt", &app_address))
        .json(&create_debt_request)
        .send()
        .await
        .expect("Failed to execute request");

    let saved = sqlx::query!("SELECT creditor_id, debtor_id, amount, currency FROM debts")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved debt.");

    assert_eq!(saved.debtor_id.to_string(), debtor_id);
    assert_eq!(saved.creditor_id.to_string(), creditor_id);
    assert_eq!(
        saved
            .amount
            .to_f64()
            .expect("Failed to cast BigDecimal to f64"),
        3000.0
    );
    assert_eq!(saved.currency, "JPY".to_string());

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn create_debt_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();

    let client = reqwest::Client::new();

    let mut create_debt_request = HashMap::new();

    create_debt_request.insert("debtor", "Yamada");
    create_debt_request.insert("creditor", "Yoshida");
    create_debt_request.insert("currency", "JPY");

    let response = client
        .post(&format!("{}/debt", &app_address))
        .json(&create_debt_request)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(400, response.status().as_u16());
}

#[derive(serde::Serialize)]
struct CreateDebtRequest {
    debtor: String,
    creditor: String,
    amount: f64,
    currency: String,
}

fn spawn_app() -> String {
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.application.port = 0;
        c
    };

    let application = debt_tracer::startup::Application::build(configuration)
        .expect("Failed to build application.");
    let application_port = application.port();

    let _ = tokio::spawn(application.run_until_stopped());

    format!("http://127.0.0.1:{}", application_port)
}
