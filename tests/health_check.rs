use std::net::{SocketAddr, TcpListener};

use axum::Server;
use axum_test_helper::TestClient;

#[tokio::test]
async fn health_check_works() {
    let port = async_spawn_app().await;

    println!("Found port {:?}", port);

    let address = format!("http://{}", port);
    let url: String = format!("{}/health_check", address);

    println!("Found url {:?}", url);

    let response = reqwest::get(url).await.expect("Could not get response");

    assert!(response.status().is_success());
    let test_response = response.text().await.expect("Could not get text content");
    assert_eq!("", test_response);
}

fn spawn_app() -> TestClient {
    let app = zero_to_production::run();

    TestClient::new(app)
}

async fn async_spawn_app() -> SocketAddr {
    let app = zero_to_production::run();

    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind ephemeral socket");
    let addr: std::net::SocketAddr = listener.local_addr().unwrap();
    println!("Listening on {}", addr);

    tokio::spawn(async move {
        let server = Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service());
        server.await.expect("server error");
    });

    addr
}
