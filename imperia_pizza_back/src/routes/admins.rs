use axum::{
    Router,
    routing::{get, patch, post},
};
use sqlx::PgPool;

use crate::handlers::admins;

/// Роуты домена "администратор"
pub fn router() -> Router<PgPool> {
    Router::new()
        .route(
            "/admin/orders/{order_id}/status",
            patch(admins::update_order_status),
        )
        .route("/admin/users/ban", post(admins::ban_user))
        .route("/admin/orders/active", post(admins::get_active_orders))
        .route("/admin/broadcast/users", post(admins::get_broadcast_users))
        .route("/admin/list", get(admins::list_admins))
}
