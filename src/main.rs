use std::net::TcpListener;

use pidgey::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("failed to read config");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    pidgey::startup::run(listener)?.await
}
