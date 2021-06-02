use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_test() {
    let addr = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/status", addr))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let server = newsletter::run(listener).expect("Failed to bind to address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
