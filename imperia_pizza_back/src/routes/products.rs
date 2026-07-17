use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::{product, product_full};

/// Роуты домена "товары"
pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/products", get(product::get_products))
        .route("/products/{id}", get(product_full::get_product_by_id))
}
