use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::catalogs;

/// Роуты домена "каталог" (категории)
pub fn router() -> Router<PgPool> {
    Router::new().route("/categories", get(catalogs::get_catalog))
}
