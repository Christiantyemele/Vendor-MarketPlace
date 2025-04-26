use Vendor_MarketPlace::api::handler::search_products;
use axum::{Router, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/products", get(search_products));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("🚀 Server listening on {}", addr);

    // Bind the address to a TcpListener
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
