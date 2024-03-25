use super::helpers::{spawn_app, TestApp};
use bigdecimal::ToPrimitive;
use std::collections::HashMap;

#[tokio::test]
async fn create_debt_returns_a_200_for_valid_json_data() {
    let test_app = spawn_app().await;

    let TestApp {
        test_creditor,
        test_debtor,
        ..
    } = &test_app;

    let debtor_id = test_debtor.user_id.to_string();
    let creditor_id = test_creditor.user_id.to_string();

    let create_debt_request = CreateDebtRequest {
        debtor: debtor_id.clone(),
        creditor: creditor_id.clone(),
        amount: 3000.0,
        currency: "JPY".to_string(),
    };

    let response = test_app
        .api_client
        .post(&format!("{}/debt", &test_app.address))
        .json(&create_debt_request)
        .send()
        .await
        .expect("Failed to execute request");

    let saved = sqlx::query!("SELECT creditor_id, debtor_id, amount, currency FROM debts")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved debt.");

    assert_eq!(200, response.status().as_u16());

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
}

#[tokio::test]
async fn create_debt_returns_a_400_when_data_is_missing() {
    let test_app = spawn_app().await;

    let client = reqwest::Client::new();

    let mut create_debt_request = HashMap::new();

    create_debt_request.insert("debtor", "Yamada");
    create_debt_request.insert("creditor", "Yoshida");
    create_debt_request.insert("currency", "JPY");

    let response = client
        .post(&format!("{}/debt", &test_app.address))
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
