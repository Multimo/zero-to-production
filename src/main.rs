use std::net::SocketAddr;

use zero_to_production::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = run();

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 8090));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
