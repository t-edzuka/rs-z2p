use rs_z2p::run;
use std::net::TcpListener;

const PORT: &str = "8080";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).expect("Failed to bind port");

    run(listener)?.await
}
