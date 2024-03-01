use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use debt_tracer::telemetry::{get_subscriber, init_subscriber};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("debt-tracer".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
