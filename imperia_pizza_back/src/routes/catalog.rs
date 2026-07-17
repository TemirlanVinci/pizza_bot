use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::catalog;

/// Роуты домена "каталог" (категории)
pub fn router() -> Router<PgPool> {
    Router::new().route("/categories", get(catalog::get_catalog))
}
