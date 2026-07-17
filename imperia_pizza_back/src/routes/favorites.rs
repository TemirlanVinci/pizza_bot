use axum::{
    Router,
    routing::{delete, get},
};
use sqlx::PgPool;

use crate::handlers::favorite;

/// Роуты домена "избранное"
pub fn router() -> Router<PgPool> {
    Router::new()
        .route(
            "/favorites",
            get(favorite::get_favorites).post(favorite::add_favorite),
        )
        .route("/favorites/{product_id}", delete(favorite::remove_favorite))
}
