use std::net::TcpListener;

type LocalHttp = String;

fn create_local_address(port: u16) -> LocalHttp {
    format!("http://127.0.0.1:{}", port)
}

fn spawn_app() -> LocalHttp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Error: address or port may be wrong.");
    let port = listener.local_addr().unwrap().port();
    let server = rs_z2p::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    create_local_address(port)
}

#[tokio::test]
async fn test_health_check_work() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let endpoint = format!("{}/health_check", addr);
    let response = client
        .get(endpoint)
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
