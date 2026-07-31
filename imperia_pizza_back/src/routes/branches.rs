use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::branches;

/// Роуты домена "филиалы"
pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/branch", get(branches::list_branches))
        .route("/branch/{id}", get(branches::get_branch))
}
