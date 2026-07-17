use axum::{
    Router,
    routing::{delete, get, post},
};
use sqlx::PgPool;

use crate::handlers::cart;

/// Роуты домена "корзина"
pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/cart", get(cart::get_cart))
        .route("/cart/add", post(cart::add_to_cart))
        .route("/cart/decrement", post(cart::decrement_cart))
        .route("/cart/item/{id}", delete(cart::remove_from_cart))
}
