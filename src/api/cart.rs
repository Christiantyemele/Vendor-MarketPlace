// src/models/cart.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: String,
    pub quantity: u32,
}

// src/api/cart.rs
use axum::{
    Router,
    extract::{Extension, Json},
    routing::{delete, get, post, put},
};

use crate::services::cart_services::CartService;

#[derive(Debug, Deserialize)]
pub struct CartRequest {
    pub product_id: String,
    pub quantity: Option<u32>,
}

pub fn cart_routes(cart_service: CartService) -> Router {
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
/// # Endpoint
/// POST `/api/cart/add`
///
/// # Request Body
/// JSON object containing:
/// - `product_id`: The ID of the product to add.
/// - `quantity` (optional): Quantity of the product (defaults to 1 if not provided).
///
/// # Response
/// Returns a JSON success message.
///
/// # Notes
/// - Currently uses a fake `user_id` ("user123") for demonstration purposes.
/// - If the product already exists in the cart, increments its quantity.
async fn add_to_cart(
    Extension(cart_service): Extension<CartService>,
    Json(payload): Json<CartRequest>,
) -> Json<&'static str> {
    let user_id = "user123".to_string();
    let item = CartItem {
        product_id: payload.product_id,
        quantity: payload.quantity.unwrap_or(1),
    };
    cart_service.add_item(user_id, item);
    Json("Item added to cart")
}

/// Handler to update the quantity of an item in the user's shopping cart.
///
/// # Endpoint
/// PUT `/api/cart/update`
///
/// # Request Body
/// JSON object containing:
/// - `product_id`: The ID of the product to update.
/// - `quantity`: The new quantity to set.
///
/// # Response
/// Returns a JSON success message.
///
/// # Notes
/// - If the product is not found in the cart, no action is taken.
/// - Assumes the `quantity` field is always provided when updating.
async fn update_cart(
    Extension(cart_service): Extension<CartService>,
    Json(payload): Json<CartRequest>,
) -> Json<&'static str> {
    let user_id = "user123".to_string();
    if let Some(quantity) = payload.quantity {
        let item = CartItem {
            product_id: payload.product_id,
            quantity,
        };
        cart_service.update_item(user_id, item);
    }
    Json("Item updated in cart")
}

/// Handler to remove an item from the user's shopping cart.
///
/// # Endpoint
/// DELETE `/api/cart/remove`
///
/// # Request Body
/// JSON object containing:
/// - `product_id`: The ID of the product to remove.
///
/// # Response
/// Returns a JSON success message.
///
/// # Notes
/// - If the product is not found in the cart, no action is taken.
async fn remove_from_cart(
    Extension(cart_service): Extension<CartService>,
    Json(payload): Json<CartRequest>,
) -> Json<&'static str> {
    let user_id = "user123".to_string();
    cart_service.remove_item(user_id, payload.product_id);
    Json("Item removed from cart")
}

/// Handler to retrieve the current contents of the user's shopping cart.
///
/// # Endpoint
/// GET `/api/cart`
///
/// # Response
/// Returns a JSON array of `CartItem` objects currently in the user's cart.
///
/// # Notes
/// - If the cart is empty, returns an empty array.
/// - Currently uses a fake `user_id` ("user123") for demonstration purposes.
async fn get_cart(Extension(cart_service): Extension<CartService>) -> Json<Vec<CartItem>> {
    let user_id = "user123".to_string();
    let cart = cart_service.get_cart(user_id);
    Json(cart)
}

#[cfg(test)]
mod tests {
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
        Router::new().merge(cart_routes(cart_service))
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
