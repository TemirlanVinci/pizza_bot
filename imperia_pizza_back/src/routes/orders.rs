use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::handlers::orders;

/// Роуты домена "заказы"
pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/orders", post(orders::create_order))
        .route("/orders/user/{user_id}", get(orders::get_user_orders))
        .route("/orders/detail/{order_id}", get(orders::get_order_detail))
        .route(
            "/internal/new-order-notification",
            post(orders::internal_new_order_notification),
        )
}
