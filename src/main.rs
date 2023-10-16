use std::net::SocketAddr;

use zero_to_production::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read config");

    let app = run();

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_post));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
