use debt_tracer::configuration::get_configuration;
use debt_tracer::startup::Application;
use debt_tracer::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("debt-tracer".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration).expect("Failed to build application.");

    application.run_until_stopped().await
}
