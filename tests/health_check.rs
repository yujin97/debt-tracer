use bigdecimal::ToPrimitive;
use debt_tracer::configuration::get_configuration;
use debt_tracer::configuration::DatabaseSettings;
use debt_tracer::startup::get_connection_pool;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

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

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub port: u16,
    pub test_creditor: TestUser,
    pub test_debtor: TestUser,
    pub api_client: reqwest::Client,
}

pub struct TestUser {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
            email: Uuid::new_v4().to_string(),
        }
    }

    async fn store(&self, pool: &PgPool) {
        sqlx::query!(
            "INSERT INTO users (user_id, username, password, email) VALUES ($1, $2, $3,$4)",
            self.user_id,
            self.username,
            self.password,
            self.email
        )
        .execute(pool)
        .await
        .expect("Failed to store test user.");
    }
}

async fn spawn_app() -> TestApp {
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };

    configure_database(&configuration.database).await;

    let test_creditor = TestUser::generate();
    let test_debtor = TestUser::generate();

    let application = debt_tracer::startup::Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();

    let _ = tokio::spawn(application.run_until_stopped());

    let address = format!("http://127.0.0.1:{}", application_port);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let test_app = TestApp {
        address,
        port: application_port,
        db_pool: get_connection_pool(&configuration.database),
        test_creditor,
        test_debtor,
        api_client: client,
    };

    test_app.test_creditor.store(&test_app.db_pool).await;
    test_app.test_debtor.store(&test_app.db_pool).await;

    test_app
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create Database.");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
