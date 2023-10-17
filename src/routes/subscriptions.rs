use std::str::FromStr;

use axum::{extract::rejection::JsonRejection, Json};
use email_address::*;
use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::{Connection, PgConnection};

use crate::configuration::get_configuration;

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

    let _response: EmailAddress = match EmailAddress::from_str(&payload.email) {
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

    // send back ok response
    (
        StatusCode::OK,
        Json(serde_json::json!({"status": "success"})),
    )
}
