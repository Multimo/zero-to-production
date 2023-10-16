use axum_test_helper::TestClient;
use reqwest::StatusCode;
use zero_to_production::startup::run;

fn spawn_app() -> TestClient {
    let app = run();

    TestClient::new(app)
}

#[tokio::test]
async fn subscribe_happy_path() {
    let client = spawn_app();

    let response = client
        .post("/subscribe")
        .header("content-type", "application/json")
        .json(&serde_json::json!({
            "email": "fake@email.com",
            "name": "some name"
        }))
        .send()
        .await;

    assert!(response.status().is_success());
    let test_response = response.text().await;
    assert_eq!("{\"status\":\"success\"}", test_response);
}

#[tokio::test]
async fn subscribe_bad_json_400() {
    let client = spawn_app();

    let response = client
        .post("/subscribe")
        .json(&serde_json::json!({
            "email": "fakeemail.com",
            "name": "some name"
        }))
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let test_response = &response.text().await;
    assert_eq!("{\"error\":\"invalid email\"}", test_response);
}

#[tokio::test]
async fn subscribe_bad_json_400_empty_name() {
    let client = spawn_app();

    let response = client
        .post("/subscribe")
        .json(&serde_json::json!({
            "email": "fakeemail.com",
            "name": ""
        }))
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let test_response = response.text().await;
    assert_eq!("{\"error\":\"missing name\"}", test_response);
}

#[tokio::test]
async fn subscribe_bad_json_400_missing_name() {
    let client = spawn_app();

    let response = client
        .post("/subscribe")
        .json(&serde_json::json!({
            "email": "fakeemail.com",
        }))
        .send()
        .await;

    let test_response = response.text().await;
    assert_eq!("{\"error\":\"invalid json\"}", test_response);
    // assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
