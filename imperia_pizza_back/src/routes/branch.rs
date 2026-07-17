use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::branch;

/// Роуты домена "филиалы"
pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/branch", get(branch::list_branches))
        .route("/branch/{id}", get(branch::get_branch))
}
