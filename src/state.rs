use crate::{api::model::ProductService, services::{cart_services::CartService, checkout_service::CheckoutService, payment_service::PaymentService}};

#[derive(Clone)]
pub struct AppState {
    pub checkout_service: CheckoutService,
    pub cart_service:CartService,
    pub product_service: ProductService,
    pub payment_service: PaymentService,
    
}
