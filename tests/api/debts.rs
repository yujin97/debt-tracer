use super::helpers::{spawn_app, TestApp};
use debt_tracer::debt::DebtJSONResponse;
use rust_decimal::prelude::ToPrimitive;
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

    let response = test_app.post_debt(3000.0, "JPY").await;

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
            .expect("Failed to cast Decimal to f64"),
        3000.0
    );
    assert_eq!(saved.currency, "JPY".to_string());
}

#[tokio::test]
async fn create_debt_returns_a_400_when_data_is_missing() {
    let test_app = spawn_app().await;

    let mut create_debt_request = HashMap::new();

    create_debt_request.insert("debtor", "Yamada");
    create_debt_request.insert("creditor", "Yoshida");
    create_debt_request.insert("currency", "JPY");

    let response = test_app
        .api_client
        .post(&format!("{}/debt", &test_app.address))
        .json(&create_debt_request)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn get_debts_returns_a_200_for_valid_query_string() {
    let test_app = spawn_app().await;

    let response = test_app
        .get_debts_by_user_id(&test_app.test_creditor.user_id)
        .await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn get_debts_returns_a_list_of_debts() {
    let test_app = spawn_app().await;

    let test_cases = vec![(3000.12, "USD"), (69.420, "JPY")];

    for (amount, currency) in &test_cases {
        test_app.post_debt(*amount, currency).await;
    }

    let response = test_app
        .get_debts_by_user_id(&test_app.test_creditor.user_id)
        .await;

    assert_eq!(200, response.status().as_u16());

    let json_result = response.json::<Vec<DebtJSONResponse>>().await;

    assert!(json_result.is_ok());

    let debts = json_result.unwrap();

    for (i, debt) in debts.iter().enumerate() {
        let (amount, currency) = test_cases[i];
        assert_eq!(debt.creditor_id, test_app.test_creditor.user_id.to_string());
        assert_eq!(
            debt.creditor_name,
            test_app.test_creditor.username.to_string()
        );
        assert_eq!(debt.debtor_id, test_app.test_debtor.user_id.to_string());
        assert_eq!(debt.debtor_name, test_app.test_debtor.username.to_string());
        assert_eq!(debt.amount, amount);
        assert_eq!(debt.currency, currency.to_owned());
    }
}
