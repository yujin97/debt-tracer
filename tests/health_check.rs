use std::collections::HashMap;

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
async fn create_debt_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();

    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("debtor", "Yamada");
    map.insert("creditor", "Yoshida");
    map.insert("amount", "3000");
    map.insert("currency", "JPY");

    let response = client
        .post(&format!("{}/debt", &app_address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

fn spawn_app() -> String {
    let configuration = {
        let mut c =
            debt_tracer::configuration::get_configuration().expect("Failed to read configuration.");
        c.application.port = 0;
        c
    };

    let application = debt_tracer::startup::Application::build(configuration)
        .expect("Failed to build application.");
    let application_port = application.port();

    let _ = tokio::spawn(application.run_until_stopped());

    format!("http://127.0.0.1:{}", application_port)
}
