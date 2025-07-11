mod api;
mod models;
mod routes;
mod views;

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        axum::Router::new()
            .route("/", axum::routing::get(routes::stations))
            .route("/station/:code", axum::routing::get(routes::station)),
    )
    .await
    .unwrap();
}
