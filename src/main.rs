use pidgey::configuration::get_configuration;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("pidgey".into(), std::io::stdout);
    let apm_layer = tracing_elastic_apm::new_layer(
        "pidgey-apm".to_string(),
        tracing_elastic_apm::config::Config::new("http://localhost:8200".to_string()),
    )
    .expect("failed to initialize apm layer");

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(apm_layer);

    set_global_default(subscriber).expect("failed to set subscriber");

    let config = get_configuration().expect("failed to read config");
    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("failed to connect to postgres");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    pidgey::startup::run(listener, connection)?.await
}
