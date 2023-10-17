use std::sync::Arc;

use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use sqlx::PgConnection;

use crate::routes::{get_subscriptions, health_check_handler, subscribe};

pub fn run(db_connection: PgConnection) -> Router {
    let connection = Arc::new(db_connection);
    // build our application with a route
    Router::new()
        .route("/", get(root_handler))
        .route("/health_check", get(health_check_handler))
        .route("/subscribe", post(subscribe))
        .route("/subscribe", get(get_subscriptions))
        .with_state(connection)
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
