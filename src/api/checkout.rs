use crate::models::order::OrderStatus;
use crate::{services::cart_services::CartError, state::AppState};
use axum::{
    Router,
    extract::{Extension, Json},
    routing::post,
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CheckoutRequest {
    pub payment_method: String, // "MTN" or "Orange"
}

#[derive(Debug, Deserialize)]
pub struct PaymentCallback {
    pub order_id: String,
    pub payment_status: String, // "success" or "failure"
}

pub fn checkout_routes() -> Router {
    Router::new()
        .route("/api/checkout", post(checkout))
        .route("/api/payment-callback", post(payment_callback))
}

async fn checkout(Extension(state): Extension<AppState>) -> Result<Json<&'static str>, CartError> {
    let user_id = "user123".to_string();

    let cart_items = state
        .cart_service
        .get_cart(user_id.clone())
        .map_err(|e| e)?;
    if cart_items.is_empty() {
        return Err(CartError::GenericError("Cart is empty".to_string())); // can't checkout with empty cart
    };

    let mut total_amount = 0.0;
    for item in &cart_items {
        let product = state
            .product_service
            .get_product_by_id(&item.product_id)
            .await
            .map_err(|_| CartError::GenericError("product not found".to_owned()))?;
        total_amount += item.quantity as f64 * product.price;
    }

    let product_ids: Vec<String> = cart_items.into_iter().map(|item| item.product_id).collect();
    let order = state
        .checkout_service
        .create_order(user_id, product_ids, total_amount)
        .map_err(|_| CartError::GenericError("Failed to create order".to_string()))?;

    state.payment_service.initiate_payment(&order);

    Ok(Json("Checkout started, awaiting payment..."))
}

async fn payment_callback(
    Extension(state): Extension<AppState>,
    Json(payload): Json<PaymentCallback>,
) -> Result<Json<&'static str>, CartError> {
    let status = match payload.payment_status.as_str() {
        "success" => OrderStatus::Paid,
        "failure" => OrderStatus::Failed,
        _ => OrderStatus::Failed,
    };

    state
        .checkout_service
        .update_order_status(payload.order_id, status)
        .map_err(|_| CartError::GenericError("Failed to update order status".to_string()))?;
    Ok(Json("Payment status updated"))
}
