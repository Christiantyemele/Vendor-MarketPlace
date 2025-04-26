use core::error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::response::IntoResponse;

use crate::api::cart::CartItem;

/// Represents possible errors from CartService.
#[derive(Debug, thiserror::Error)]
pub enum CartError {
    #[error("Failed to lock the cart storage")]
    LockError,
    #[error("Cart not found for user")]
    CartNotFound,
    #[error("Item not found in cart: {0}")]
    GenericError(String),
}
impl IntoResponse for CartError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            CartError::LockError => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            CartError::CartNotFound => axum::http::StatusCode::NOT_FOUND,
            CartError::GenericError(_) => axum::http::StatusCode::BAD_REQUEST,
        };
        (status, self.to_string()).into_response()
    }
}
#[derive(Clone)]
pub struct CartService {
    carts: Arc<Mutex<HashMap<String, Vec<CartItem>>>>,
}

impl CartService {
    pub fn new() -> Self {
        CartService {
            carts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Adds an item to the user's cart. If the item exists, increments the quantity.
    pub fn add_item(&self, user_id: String, item: CartItem) -> Result<(), CartError> {
        let mut carts = self.carts.lock().map_err(|_| CartError::LockError)?;
        let cart = carts.entry(user_id).or_insert_with(Vec::new);

        if let Some(existing) = cart.iter_mut().find(|i| i.product_id == item.product_id) {
            existing.quantity += item.quantity;
        } else {
            cart.push(item);
        }

        Ok(())
    }

    /// Updates an item's quantity in the user's cart.
    pub fn update_item(&self, user_id: String, item: CartItem) -> Result<(), CartError> {
        let mut carts = self.carts.lock().map_err(|_| CartError::LockError)?;
        if let Some(cart) = carts.get_mut(&user_id) {
            if let Some(existing) = cart.iter_mut().find(|i| i.product_id == item.product_id) {
                existing.quantity = item.quantity;
            }
        }
        Ok(())
    }

    /// Removes an item from the user's cart.
    pub fn remove_item(&self, user_id: String, product_id: String) -> Result<(), CartError> {
        let mut carts = self.carts.lock().map_err(|_| CartError::LockError)?;
        if let Some(cart) = carts.get_mut(&user_id) {
            cart.retain(|i| i.product_id != product_id);
        }
        Ok(())
    }

    /// Retrieves the user's cart.
    pub fn get_cart(&self, user_id: String) -> Result<Vec<CartItem>, CartError> {
        let carts = self.carts.lock().map_err(|_| CartError::LockError)?;
        Ok(carts.get(&user_id).cloned().unwrap_or_else(Vec::new))
    }
}
