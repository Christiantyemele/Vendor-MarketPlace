use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub category: String,
    pub region: String,
    pub certified: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ProductError {
    #[error("Failed to lock the product storage")]
    LockError,
    #[error("Product not found")]
    ProductNotFound,
}

#[derive(Clone)]
pub struct ProductService {
    products: Arc<Mutex<HashMap<String, Product>>>,
}
impl ProductService {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        for p in mock_products() {
            map.insert(p.id.clone(), p);
        }

        ProductService {
            products: Arc::new(Mutex::new(map)),
        }
    }

    pub async fn get_product_by_id(&self, product_id: &str) -> Result<Product, ProductError> {
        let products = self.products.lock().await;
        products.get(product_id)
            .cloned()
            .ok_or(ProductError::ProductNotFound)
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ProductQuery {
    pub query: Option<String>,
    pub category: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub region: Option<String>,
    pub certified: Option<bool>,
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub page: usize,
    pub limit: usize,
    pub total: usize,
    pub products: Vec<T>,
}

// mock_product
pub fn mock_products() -> Vec<Product> {
    vec![
        Product {
            id: "2".to_string(),
            name: "Bamileke Stool".to_string(),
            price: 15000.0,
            category: "Furniture".to_string(),
            region: "Ouest".to_string(),
            certified: true,
        },
        Product {
            id: "3".to_string(),
            name: "Cameroon T-shirt".to_string(),
            price: 5000.0,
            category: "Clothing".to_string(),
            region: "Centre".to_string(),
            certified: false,
        },
    ]
}
