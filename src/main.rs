//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health-check", get(health_check_handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 8090));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn health_check_handler() -> Html<&'static str> {
    Html("ok")
}
