use axum_test_helper::TestClient;

#[tokio::test]
async fn health_check_words() {
    let client = spawn_app();

    let response = client.get("/health_check").send().await;

    assert!(response.status().is_success());
    let test_response = response.text().await;
    assert_eq!("", test_response);
}

fn spawn_app() -> TestClient {
    let app = zero_to_production::run();

    TestClient::new(app)
}
