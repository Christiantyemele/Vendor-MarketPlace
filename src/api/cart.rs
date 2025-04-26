use std::sync::Arc;

// src/models/cart.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: String,
    pub quantity: u32,
}

use axum::{
    Router,
    extract::{Extension, Json},
    response::AppendHeaders,
    routing::{delete, get, post, put},
};

use crate::{
    services::cart_services::{CartError, CartService},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct CartRequest {
    pub product_id: String,
    pub quantity: Option<u32>,
}

pub fn cart_routes(appstate: Arc<AppState>) -> Router {
    let cart_service = appstate.cart_service.clone();
    Router::new()
        .nest(
            "/api/cart",
            Router::new()
                .route("/", get(get_cart))
                .route("/add", post(add_to_cart))
                .route("/update", put(update_cart))
                .route("/remove", delete(remove_from_cart)),
        )
        .layer(Extension(cart_service))
}

/// Handler to add an item to the user's shopping cart.
///
/// POST `/api/cart/add`
async fn add_to_cart(
    Extension(cart_service): Extension<CartService>,
    Json(payload): Json<CartRequest>,
) -> Result<Json<&'static str>, CartError> {
    let user_id = "user123".to_string();
    let item = CartItem {
        product_id: payload.product_id,
        quantity: payload.quantity.unwrap_or(1),
    };

    cart_service.add_item(user_id, item)?;
    Ok(Json("Item added to cart"))
}

/// Handler to update the quantity of an item in the user's shopping cart.
///
/// PUT `/api/cart/update`
async fn update_cart(
    Extension(cart_service): Extension<CartService>,
    Json(payload): Json<CartRequest>,
) -> Result<Json<&'static str>, CartError> {
    let user_id = "user123".to_string();
    if let Some(quantity) = payload.quantity {
        let item = CartItem {
            product_id: payload.product_id,
            quantity,
        };
        cart_service.update_item(user_id, item)?;
    }
    Ok(Json("Item updated in cart"))
}

/// Handler to remove an item from the user's shopping cart.
///
/// DELETE `/api/cart/remove`
async fn remove_from_cart(
    Extension(cart_service): Extension<CartService>,
    Json(payload): Json<CartRequest>,
) -> Result<Json<&'static str>, CartError> {
    let user_id = "user123".to_string();
    cart_service.remove_item(user_id, payload.product_id)?;
    Ok(Json("Item removed from cart"))
}

/// Handler to retrieve the current contents of the user's shopping cart.
///
/// GET `/api/cart`
async fn get_cart(
    Extension(cart_service): Extension<CartService>,
) -> Result<Json<Vec<CartItem>>, CartError> {
    let user_id = "user123".to_string();
    let cart = cart_service.get_cart(user_id)?;
    Ok(Json(cart))
}

#[cfg(test)]
mod tests {
    use crate::{
        api::model::ProductService,
        services::{checkout_service::CheckoutService, payment_service::PaymentService},
    };

    use super::*;
    use axum::{
        Router,
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use serde_json::json;
    use tower::ServiceExt; // for `oneshot`

    fn app() -> Router {
        let cart_service = CartService::new();
        let appstate = AppState {
            cart_service: cart_service.clone(),
            checkout_service: CheckoutService::new(),
            product_service: ProductService::new(),
            payment_service: PaymentService::new(),
        };
        Router::new().merge(cart_routes(Arc::new(appstate)))
    }

    #[tokio::test]
    async fn test_add_item_to_cart() {
        let app = app();

        let payload = json!({
            "product_id": "2",
            "quantity": 2
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/cart/add")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_cart_after_adding_item() {
        let app = app();

        // Add an item first
        let payload = json!({
            "product_id": "3",
            "quantity": 1
        });

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/cart/add")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Now fetch the cart
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/cart")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let cart: Vec<CartItem> = serde_json::from_slice(&body).unwrap();

        assert_eq!(cart.len(), 1);
        assert_eq!(cart[0].product_id, "3");
        assert_eq!(cart[0].quantity, 1);
    }

    #[tokio::test]
    async fn test_update_item_quantity() {
        let app = app();

        // First add item
        let payload = json!({
            "product_id": "2",
            "quantity": 2
        });
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/cart/add")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Then update quantity
        let update_payload = json!({
            "product_id": "2",
            "quantity": 5
        });

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/api/cart/update")
                    .header("content-type", "application/json")
                    .body(Body::from(update_payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Fetch cart and verify quantity updated
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/cart")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let cart: Vec<CartItem> = serde_json::from_slice(&body).unwrap();

        assert_eq!(cart[0].quantity, 5);
    }

    #[tokio::test]
    async fn test_remove_item_from_cart() {
        let app = app();

        // Add item first
        let payload = json!({
            "product_id": "2",
            "quantity": 2
        });
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/cart/add")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Remove the item
        let remove_payload = json!({
            "product_id": "2"
        });

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/cart/remove")
                    .header("content-type", "application/json")
                    .body(Body::from(remove_payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Fetch cart and verify it's empty
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/cart")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let cart: Vec<CartItem> = serde_json::from_slice(&body).unwrap();

        assert!(cart.is_empty());
    }
}
