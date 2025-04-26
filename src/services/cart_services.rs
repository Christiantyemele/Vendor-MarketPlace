// src/services/cart_service.rs
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::api::cart::CartItem;


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

    pub fn add_item(&self, user_id: String, item: CartItem) {
        let mut carts = self.carts.lock().unwrap();
        let cart = carts.entry(user_id).or_insert_with(Vec::new);

        if let Some(existing) = cart.iter_mut().find(|i| i.product_id == item.product_id) {
            existing.quantity += item.quantity;
        } else {
            cart.push(item);
        }
    }

    pub fn update_item(&self, user_id: String, item: CartItem) {
        let mut carts = self.carts.lock().unwrap();
        if let Some(cart) = carts.get_mut(&user_id) {
            if let Some(existing) = cart.iter_mut().find(|i| i.product_id == item.product_id) {
                existing.quantity = item.quantity;
            }
        }
    }

    pub fn remove_item(&self, user_id: String, product_id: String) {
        let mut carts = self.carts.lock().unwrap();
        if let Some(cart) = carts.get_mut(&user_id) {
            cart.retain(|i| i.product_id != product_id);
        }
    }

    pub fn get_cart(&self, user_id: String) -> Vec<CartItem> {
        let carts = self.carts.lock().unwrap();
        carts.get(&user_id).cloned().unwrap_or_else(Vec::new)
    }
}
