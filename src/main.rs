use pidgey::configuration::get_configuration;
use pidgey::startup::Application;
use pidgey::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("pidgey".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("failed to read config");
    let application = Application::build(config).await?;
    application.run_until_stopped().await?;

    Ok(())
}
