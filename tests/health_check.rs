use std::net::{SocketAddr, TcpListener};

use axum::Server;
use zero_to_production::startup::run;

async fn async_spawn_app() -> SocketAddr {
    let app = run();

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

#[tokio::test]
async fn health_check_works() {
    let addr = async_spawn_app().await;

    println!("Found addr {:?}", addr);

    let address = format!("http://{}", addr);
    let url: String = format!("{}/health_check", address);

    println!("Found url {:?}", url);

    let response = reqwest::get(url).await.expect("Could not get response");

    assert!(response.status().is_success());
    let test_response = response.text().await.expect("Could not get text content");
    assert_eq!("", test_response);
}
