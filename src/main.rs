use std::net::SocketAddr;

use zero_to_production::{
    configuration::{connect_to_db, get_configuration},
    startup::run,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read config");

    let db_connection = connect_to_db().await;

    let app = run(db_connection);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
