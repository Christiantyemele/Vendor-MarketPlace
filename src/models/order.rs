// src/models/order.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    PendingPayment,
    Paid,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub order_id: String,
    pub user_id: String,
    pub items: Vec<String>, 
    pub total_amount: f64,
    pub status: OrderStatus,
}
