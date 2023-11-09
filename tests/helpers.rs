use axum_test_helper::TestClient;
use std::net::{SocketAddr, TcpListener};

use sqlx::PgPool;
use zero_to_production::{configuration::connect_to_random_db, startup::run};

pub struct TestApp {
    pub addr: SocketAddr,
    pub db_pool: PgPool,
    pub client: TestClient,
}

pub async fn async_spawn_app() -> TestApp {
    let pool: sqlx::Pool<sqlx::Postgres> = connect_to_random_db().await;
    let app = run(pool.clone());

    // :0 will assign to any socket
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind ephemeral socket");
    let addr: std::net::SocketAddr = listener.local_addr().unwrap();

    println!("async_spawn_app: Listening on {}", addr);

    // tokio::spawn(async move {
    //     let server = Server::from_tcp(listener)
    //         .unwrap()
    //         .serve(app.into_make_service());
    //     server.await.expect("server error");
    // });

    let test_client: TestClient = TestClient::new(app);

    TestApp {
        addr,
        db_pool: pool,
        client: test_client,
    }
}
