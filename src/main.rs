use rs_z2p::run;
use std::net::TcpListener;

const PORT: &str = "8080";

#[cfg(not(tarpaulin_include))]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_addr = format!("127.0.0.1:{PORT}");
    println!("Listening address: http://{}", &app_addr);
    let listener = TcpListener::bind(app_addr).expect("Failed to bind port");
    run(listener)?.await
}
