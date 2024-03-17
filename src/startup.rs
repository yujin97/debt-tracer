use crate::configuration::Settings;
use crate::debt::create_debt;
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let connection = PgPool::connect(&configuration.database.connection_string())
            .await
            .expect("Failed to connect to Postgres");

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(listener, connection)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/debt", web::post().to(create_debt))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
