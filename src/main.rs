use std::net::TcpListener;

use pidgey::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to bind address");
    run(listener)?.await
}
