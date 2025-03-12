mod api;
mod models;
mod routes;
mod views;

use axum::{Router, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        Router::new().route("/", get(routes::hello_world)),
    )
    .await
    .unwrap();
}
