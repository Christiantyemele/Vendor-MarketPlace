use Vendor_MarketPlace::{
    api::{cart::cart_routes, handler::search_products},
    services::cart_services::CartService
};
use axum::{Router, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let cart_service = CartService::new();

    // Build app state

    
    let app = Router::new()
        .route("/api/products", get(search_products))
        .merge(cart_routes(cart_service));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("🚀 Server listening on {}", addr);

    // Bind the address to a TcpListener
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
