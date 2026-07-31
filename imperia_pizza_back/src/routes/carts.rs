use axum::{
    Router,
    routing::{delete, get, post},
};
use sqlx::PgPool;

use crate::handlers::carts;

/// Роуты домена "корзина"
pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/cart", get(carts::get_cart))
        .route("/cart/add", post(carts::add_to_cart))
        .route("/cart/decrement", post(carts::decrement_cart))
        .route("/cart/item/{id}", delete(carts::remove_from_cart))
}
