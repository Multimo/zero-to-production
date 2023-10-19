use axum::{
    extract::{rejection::JsonRejection, State},
    Json,
};
use chrono::Utc;
use email_address::*;
use reqwest::StatusCode;
use serde::Deserialize;
use std::str::FromStr;
use uuid::Uuid;

use crate::startup::AppState;

#[derive(Deserialize)]
pub struct Subscribe {
    email: String,
    name: String,
}

// not sure what i was going to do here?
// fn deserialize_subscribe_json<T>(payload: Json<T>) -> Result<Json<T>, JsonRejection> {
//     match payload {
//         Ok(payload) => {
//             payload
//             // We got a valid JSON payload
//         }
//         Err(JsonRejection::MissingJsonContentType(_)) => {
//             // Request didn't have `Content-Type: application/json`
//             // header
//         }
//         Err(JsonRejection::JsonDataError(_)) => {
//             // Couldn't deserialize the body into the target type
//         }
//         Err(JsonRejection::JsonSyntaxError(_)) => {
//             // Syntax error in the body
//         }
//         Err(JsonRejection::BytesRejection(_)) => {
//             // Failed to extract the request body
//         }
//         Err(_) => {
//             // `JsonRejection` is marked `#[non_exhaustive]` so match must
//             // include a catch-all case.
//         }
//     }
// }

pub async fn get_subscriptions() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({"status": "success"})),
    )
}

pub async fn subscribe(
    State(state): State<AppState>,
    payload: Result<Json<Subscribe>, JsonRejection>,
) -> (StatusCode, Json<serde_json::Value>) {
    let Ok(payload) = payload else  {
        // We got a invalid JSON payload
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "invalid json"
            })),
        );
    };

    println!("{:?}", payload.email);

    if payload.name.is_empty() {
        let error = Json(serde_json::json!({
            "error": "missing name",
        }));
        return (StatusCode::BAD_REQUEST, error);
    }

    if !EmailAddress::is_valid(&payload.email) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "invalid email"
            })),
        );
    }

    let email: EmailAddress = match EmailAddress::from_str(&payload.email) {
        Ok(email) => email,
        Err(_err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "invalid email"
                })),
            )
        }
    };

    // store email and name in newsletter db
    let query_result = sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        email.as_str(),
        payload.name,
        Utc::now()
    )
    .execute(&state.db)
    .await;

    if query_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Insert into db failed"
            })),
        );
    }

    // send back ok response
    (
        StatusCode::OK,
        Json(serde_json::json!({"status": "success"})),
    )
}
