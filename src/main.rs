use std::net::TcpListener;

use pidgey::configuration::get_configuration;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("failed to read config");
    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("failed to connect to postgres");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    pidgey::startup::run(listener, connection)?.await
}
