use axum_test_helper::TestClient;
use reqwest::StatusCode;

use sqlx::PgConnection;
use zero_to_production::{configuration::connect_to_db, startup::run};

fn spawn_app(connection: PgConnection) -> TestClient {
    let app = run(connection);

    TestClient::new(app)
}

#[tokio::test]
async fn subscribe_happy_path() {
    let connection = connect_to_db().await;
    let client = spawn_app(connection);

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

    let mut connection = connect_to_db().await;
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "fake@email.com");
    assert_eq!(saved.name, "some name");
}

#[tokio::test]
async fn subscribe_bad_json_400() {
    let connection = connect_to_db().await;
    let client = spawn_app(connection);

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
    let connection = connect_to_db().await;
    let client = spawn_app(connection);

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
    let connection = connect_to_db().await;
    let client = spawn_app(connection);

    let response = client
        .post("/subscribe")
        .json(&serde_json::json!({
            "email": "fakeemail.com",
        }))
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let test_response = response.text().await;
    assert_eq!("{\"error\":\"invalid json\"}", test_response);
}
