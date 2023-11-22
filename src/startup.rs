use axum::{
    extract::MatchedPath,
    http::Request,
    response::Html,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use sqlx::{Pool, Postgres};

use crate::routes::{get_subscriptions, health_check_handler, subscribe};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

pub fn run(db_connection: Pool<Postgres>) -> Router {
    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
    //             // axum logs rejections from built-in extractors with the `axum::rejection`
    //             // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
    //             "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
    //         }),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    // Init subscriber for tracing / logs
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let connection = db_connection;
    let state = AppState { db: connection };
    // build our application with a route
    Router::new()
        .route("/", get(root_handler))
        .route("/health_check", get(health_check_handler))
        .route("/subscribe", post(subscribe))
        .route("/subscribe", get(get_subscriptions))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
