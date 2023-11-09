use reqwest::StatusCode;

use crate::helpers::async_spawn_app;
mod helpers;

#[tokio::test]
async fn subscribe_happy_path() {
    let test_app = async_spawn_app().await;

    let response = test_app
        .client
        .post("/subscribe")
        .header("content-type", "application/json")
        .json(&serde_json::json!({
            "email": "fake@email.com",
            "name": "some name"
        }))
        .send()
        .await;

    let test_response = response.text().await;
    assert_eq!("{\"status\":\"success\"}", test_response);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    println!("found saved {:?}", saved);
    assert_eq!(saved.email, "fake@email.com");
    assert_eq!(saved.name, "some name");
}

#[tokio::test]
async fn subscribe_bad_json_400() {
    let test_app = async_spawn_app().await;

    let response = test_app
        .client
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
    let test_app = async_spawn_app().await;

    let response = test_app
        .client
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
    let test_app = async_spawn_app().await;

    let response = test_app
        .client
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
