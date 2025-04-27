use axum::{
    extract::{Path, Extension},
    routing::{get, post},
    Router,
    Json,
};
use crate::state::AppState;
use crate::models::order::Order;
use crate::services::checkout_service::CheckoutError;
use axum::http::StatusCode;

pub fn order_routes() -> Router {
    Router::new()
        .route("/api/orders", get(list_orders))
        .route("/api/orders/:order_id", get(view_order))
        .route("/api/orders/:order_id/cancel", post(cancel_order))
}

async fn list_orders(
    Extension(state): Extension<AppState>,
) -> Result<Json<Vec<Order>>, StatusCode> {
    let user_id = "user123".to_string(); // dummy for now

    let orders = state.checkout_service.get_user_orders(&user_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(orders))
}

async fn view_order(
    Extension(state): Extension<AppState>,
    Path(order_id): Path<String>,
) -> Result<Json<Order>, StatusCode> {
    let order = state.checkout_service.get_order_by_id(&order_id)
        .map_err(|err| match err {
            CheckoutError::OrderNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(order))
}

async fn cancel_order(
    Extension(state): Extension<AppState>,
    Path(order_id): Path<String>,
) -> Result<Json<&'static str>, StatusCode> {
    state.checkout_service.cancel_order(&order_id)
        .map_err(|err| match err {
            CheckoutError::OrderNotFound => StatusCode::NOT_FOUND,
            CheckoutError::CannotCancelOrder => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json("Order cancelled"))
}
