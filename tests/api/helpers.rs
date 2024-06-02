use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use debt_tracer::configuration::get_configuration;
use debt_tracer::configuration::DatabaseSettings;
use debt_tracer::startup::get_connection_pool;
use debt_tracer::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

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

impl TestApp {
    pub async fn post_debt(
        &self,
        amount: f64,
        currency: &str,
        description: &str,
    ) -> reqwest::Response {
        let debtor_id = self.test_debtor.user_id.to_string();
        let creditor_id = self.test_creditor.user_id.to_string();

        let create_debt_request = CreateDebtRequest {
            debtor_id: debtor_id.clone(),
            creditor_id: creditor_id.clone(),
            amount,
            currency: currency.to_string(),
            description: description.to_string(),
        };

        self.api_client
            .post(&format!("{}/debt", &self.address))
            .json(&create_debt_request)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_debts_by_user_id(&self, user_id: &Uuid) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/debts", &self.address))
            .query(&[("user_id", user_id)])
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn login_as_test_creditor(&self) -> reqwest::Response {
        let login_request_body = serde_json::json!({
            "username" : &self.test_creditor.username,
            "password" : &self.test_creditor.password,
        });

        self.api_client
            .post(&format!("{}/login", &self.address))
            .json(&login_request_body)
            .send()
            .await
            .expect("Failed to execute request")
    }
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
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(self.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
        sqlx::query!(
            "INSERT INTO users (user_id, username, password_hash, email) VALUES ($1, $2, $3,$4)",
            self.user_id,
            self.username,
            password_hash,
            self.email
        )
        .execute(pool)
        .await
        .expect("Failed to store test user.");
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

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

#[derive(serde::Serialize)]
struct CreateDebtRequest {
    debtor_id: String,
    creditor_id: String,
    amount: f64,
    currency: String,
    description: String,
}
