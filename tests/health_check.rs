use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=bob%20jones&email=bobjones%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=bob%20jones", "missing email"),
        ("email=bob_jones%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", address))
            .header("Content-Type", "application/x-www-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with 400 Bad Request when payload was {}",
            error_message
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = pidgey::run(listener).expect("failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
