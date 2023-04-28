use axum::{response::Html, routing::get, Router};

pub fn run() -> Router {
    // build our application with a route
    Router::new()
        .route("/", get(root_handler))
        .route("/health_check", get(health_check_handler))
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// just returns a empty 200 ok
async fn health_check_handler() {}
