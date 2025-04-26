use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::models::order::{Order, OrderStatus};

#[derive(Clone)]
pub struct CheckoutService {
    pub orders: Arc<Mutex<Vec<Order>>>,
}

impl CheckoutService {
    pub fn new() -> Self {
        CheckoutService {
            orders: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn create_order(&self, user_id: String, items: Vec<String>, total_amount: f64) -> Order {
        let order = Order {
            order_id: Uuid::new_v4().to_string(),
            user_id,
            items,
            total_amount,
            status: OrderStatus::PendingPayment,
        };

        let mut orders = self.orders.lock().unwrap();
        orders.push(order.clone());

        order
    }

    pub fn update_order_status(&self, order_id: String, new_status: OrderStatus) {
        let mut orders = self.orders.lock().unwrap();
        if let Some(order) = orders.iter_mut().find(|o| o.order_id == order_id) {
            order.status = new_status;
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, extract::Request, Extension, Router};
    use hyper::StatusCode;
    use serde_json::json;
    use tower::ServiceExt;

    use crate::{api::{cart::CartItem, checkout::checkout_routes, model::ProductService}, services::{cart_services::CartService, payment_service::PaymentService}, state::AppState};

    use super::CheckoutService;


    fn app() -> Router {
        let cart_service = CartService::new();
        let checkout_service = CheckoutService::new();
        let payment_service = PaymentService::new();
        let product_service = ProductService::new();

        let app_state = AppState {
            cart_service,
            checkout_service,
            payment_service,
            product_service,
        };

        Router::new()
            .merge(checkout_routes())
            .layer(Extension(app_state))
    }

    #[tokio::test]
    async fn test_checkout_empty_cart_should_fail() {
        let app = app();



        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/checkout")
                    .header("content-type", "application/json")
                    .body(Body::from(""))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_checkout_with_cart_should_succeed() {
        // Create AppState manually
        let cart_service = CartService::new();
        let checkout_service = CheckoutService::new();
        let payment_service = PaymentService::new();
        let product_service = ProductService::new();
    
        let app_state = AppState {
            cart_service: cart_service.clone(),
            checkout_service,
            payment_service,
            product_service,
        };
    
        // Build the app
        let app = Router::new()
            .merge(checkout_routes())
            .layer(Extension(app_state.clone()));
    
        // ✅ Insert product manually in the cart before making the checkout request
        cart_service.add_item(
            "user123".to_string(),
            CartItem {
                product_id: "2".to_string(), // Bamileke Stool (price = 15000)
                quantity: 2,
            },
        ).unwrap();
    
        // Now send checkout request
        let payload = json!({
            "payment_method": "MTN"
        });
    
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/checkout")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
    
        assert_eq!(response.status(), StatusCode::OK);
    }
    
    #[tokio::test]
    async fn test_payment_callback_success() {
        let app = app();

        let payload = json!({
            "order_id": "some-fake-order-id",
            "payment_status": "success"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/payment-callback")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
