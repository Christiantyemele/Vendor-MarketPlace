use crate::{api::model::ProductError, services::cart_services::CartError};

impl From<ProductError> for CartError {
    fn from(err: ProductError) -> CartError {
        match err {
            ProductError::LockError => CartError::LockError,
            ProductError::ProductNotFound => {
                CartError::GenericError("Product not found".to_string())
            }
        }
    }
}

impl Into<ProductError> for CartError {
    fn into(self) -> ProductError {
        match self {
            CartError::LockError => ProductError::LockError,
            CartError::CartNotFound => ProductError::ProductNotFound,
            CartError::GenericError(_) => ProductError::ProductNotFound,
        }
    }
}