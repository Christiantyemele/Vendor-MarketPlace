use axum::{Json, extract::Query, http::StatusCode, response::IntoResponse};

use super::model::{PaginatedResponse, Product, ProductQuery, mock_products};

/// Handles the GET `/api/products` endpoint.
///
/// Allows buyers to search and filter available products with the following optional query parameters:
/// - `query`: Search keyword for product name (case-insensitive partial match)
/// - `category`: Filter by product category
/// - `min_price`: Minimum price filter
/// - `max_price`: Maximum price filter
/// - `region`: Filter by vendor's region
/// - `certified`: Filter by "Made in Cameroon" certified status
/// - `page`: Pagination page number (default = 1)
/// - `limit`: Number of products per page (default = 10)
///
/// Returns a paginated JSON response containing the list of matching products.
///
/// # Example
///
/// ```
/// GET /api/products?query=stool&min_price=5000&region=Ouest&page=1&limit=5
/// ```
///
/// # Response
/// - `200 OK` with `PaginatedResponse<Product>` body
///
/// # Errors
/// - Currently none; always returns 200 even if no products found (empty list)
///
/// # Notes
/// - Currently uses mock products; will later integrate with a database.
pub async fn search_products(Query(params): Query<ProductQuery>) -> impl IntoResponse {
    // Todo: implement real database
    let all_products = mock_products();

    let filtered: Vec<Product> = all_products
        .into_iter()
        .filter(|p| {
            if let Some(ref query) = params.query {
                if !p.name.to_lowercase().contains(&query.to_lowercase()) {
                    return false;
                }
            }
            if let Some(ref category) = params.category {
                if p.category.to_lowercase() != category.to_lowercase() {
                    return false;
                }
            }
            if let Some(min_price) = params.min_price {
                if p.price < min_price {
                    return false;
                }
            }
            if let Some(max_price) = params.max_price {
                if p.price > max_price {
                    return false;
                }
            }
            if let Some(ref region) = params.region {
                if p.region.to_lowercase() != region.to_lowercase() {
                    return false;
                }
            }
            if let Some(certified) = params.certified {
                if p.certified != certified {
                    return false;
                }
            }
            true
        })
        .collect();

    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let start = (page - 1) * limit;

    let paginated_products = filtered.iter().skip(start).take(limit).cloned().collect();

    let response = PaginatedResponse {
        page,
        limit,
        total: filtered.len(),
        products: paginated_products,
    };

    (StatusCode::OK, Json(response))
}

#[cfg(test)]
mod test {
    use axum::body::to_bytes;
    use axum::{Router, body::Body, extract::Request, routing::get};
    use hyper::StatusCode;
    use tower::util::ServiceExt;

    use crate::api::model::{PaginatedResponse, Product};

    use super::search_products;

    fn app() -> Router {
        Router::new().route("/api/products", get(search_products))
    }

    #[tokio::test]
    async fn test_search_products_empty_query() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/products")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();

        let parsed: PaginatedResponse<Product> = serde_json::from_slice(&body).unwrap();

        // Check if we received at least one mocked product
        assert_eq!(parsed.total, 2);
    }

    #[tokio::test]
    async fn test_search_products_with_query_stool() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/api/products?query=stool").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 1024 *1024).await.unwrap();
        let parsed: PaginatedResponse<Product> = serde_json::from_slice(&body).unwrap();

        // Only the "Bamileke Stool" should match
        assert_eq!(parsed.total, 1);
        assert_eq!(parsed.products[0].name, "Bamileke Stool");
    }

    #[tokio::test]
    async fn test_search_products_with_category_clothing() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/api/products?category=Clothing").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let parsed: PaginatedResponse<Product> = serde_json::from_slice(&body).unwrap();

        // Only the "Cameroon T-shirt" should match
        assert_eq!(parsed.total, 1);
        assert_eq!(parsed.products[0].category, "Clothing");
    }

    #[tokio::test]
    async fn test_search_products_certified_true() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/api/products?certified=true").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let parsed: PaginatedResponse<Product> = serde_json::from_slice(&body).unwrap();

        // Only "Bamileke Stool" is certified
        assert_eq!(parsed.total, 1);
        assert_eq!(parsed.products[0].certified, true);
    }
}
