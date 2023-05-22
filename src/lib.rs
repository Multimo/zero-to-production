use std::str::FromStr;

use axum::{
    extract::rejection::JsonRejection,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use email_address::*;
use reqwest::StatusCode;
use serde::Deserialize;

pub fn run() -> Router {
    // build our application with a route
    Router::new()
        .route("/", get(root_handler))
        .route("/health_check", get(health_check_handler))
        .route("/subscribe", post(subscribe))
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// just returns a empty 200 ok
async fn health_check_handler() {}

#[derive(Deserialize)]
struct Subscribe {
    email: String,
    name: String,
}

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

async fn subscribe(
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

    let response = match EmailAddress::from_str(&payload.email) {
        Ok(email) => email,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "invalid email"
                })),
            )
        }
    };

    // store email and name in newletter db

    //
    (
        StatusCode::OK,
        Json(serde_json::json!({"status": "success"})),
    )
}
