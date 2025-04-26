use Vendor_MarketPlace::{
    api::{
        cart::cart_routes, checkout::checkout_routes, handler::search_products,
        model::ProductService,
    },
    services::{
        cart_services::CartService, checkout_service::CheckoutService,
        payment_service::PaymentService,
    },
    state::AppState,
};
use axum::{Extension, Router, routing::get};
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    let cart_service = CartService::new();
    let checkout_service = CheckoutService::new();
    let payment_service = PaymentService::new();
    let product_service = ProductService::new();

    let app_state = Arc::new(AppState {
        cart_service,
        checkout_service,
        payment_service,
        product_service,
    });

    let app = Router::new()
        .route("/api/products", get(search_products))
        .merge(cart_routes(app_state.clone()))
        .merge(checkout_routes())
        .layer(Extension(app_state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("🚀 Server listening on {}", addr);

    // Bind the address to a TcpListener
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
