use axum::{
    Router,
    routing::{delete, get},
};
use sqlx::PgPool;

use crate::handlers::favorites;

/// Роуты домена "избранное"
pub fn router() -> Router<PgPool> {
    Router::new()
        .route(
            "/favorites",
            get(favorites::get_favorites).post(favorites::add_favorite),
        )
        .route("/favorites/{product_id}", delete(favorites::remove_favorite))
}
