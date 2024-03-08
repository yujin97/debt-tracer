#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
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
