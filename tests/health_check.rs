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

    let create_debt_request = CreateDebtRequest {
        debtor: "Yamada".to_string(),
        creditor: "Yoshida".to_string(),
        amount: 3000.0,
        currency: "JPY".to_string(),
    };

    let response = client
        .post(&format!("{}/debt", &app_address))
        .json(&create_debt_request)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
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
