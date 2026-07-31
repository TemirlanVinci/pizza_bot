use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::products;

/// Роуты домена "товары"
pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/products", get(products::get_products))
        .route("/products/{id}", get(products::get_product_by_id))
}
