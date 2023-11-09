use sqlx::{Connection, PgConnection};
use zero_to_production::configuration::get_configuration;

use crate::helpers::async_spawn_app;
mod helpers;

#[tokio::test]
async fn health_check_works() {
    let test_app = async_spawn_app().await;

    let response = test_app.client.get("/health_check").send().await;

    assert!(response.status().is_success());
    let test_response = response.text().await;
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
