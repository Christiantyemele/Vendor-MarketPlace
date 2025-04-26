// src/services/payment_service.rs
use crate::models::order::Order;

#[derive(Clone)]
pub struct PaymentService;

impl PaymentService {
    pub fn new() -> Self {
        PaymentService
    }

    pub fn initiate_payment(&self, order: &Order) {
        println!(
            "Initiating mobile money payment for order {} (amount: {})",
            order.order_id, order.total_amount
        );
        // In real life, call MTN/Orange API here
    }
}
