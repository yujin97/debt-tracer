use debt_tracer::startup::run;
use debt_tracer::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("debt-tracer".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    run().await
}
