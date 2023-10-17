use std::net::{SocketAddr, TcpListener};

use axum::Server;
use sqlx::{Connection, PgConnection};
use zero_to_production::{
    configuration::{connect_to_db, get_configuration},
    startup::run,
};

async fn async_spawn_app() -> SocketAddr {
    let connection = connect_to_db().await;
    let app = run(connection);

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

#[tokio::test]
async fn database_connection_successful() {
    let config = get_configuration().expect("Failed to read configuration");

    let mut connection = PgConnection::connect(&config.database.connection_string())
        .await
        .expect("cannot connect to db");

    connection.ping().await.unwrap();
    // assert_eq!(ping_result, ());

    connection.close().await.unwrap();
    // assert_eq!(close_result, ());
}
