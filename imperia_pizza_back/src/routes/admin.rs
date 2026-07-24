use axum::{
    Router,
    routing::{get, patch, post},
};
use sqlx::PgPool;

use crate::handlers::admin;

/// Роуты домена "администратор"
pub fn router() -> Router<PgPool> {
    Router::new()
        .route(
            "/admin/orders/{order_id}/status",
            patch(admin::update_order_status),
        )
        .route("/admin/users/ban", post(admin::ban_user))
        .route("/admin/orders/active", post(admin::get_active_orders))
        .route("/admin/broadcast/users", post(admin::get_broadcast_users))
        .route("/admin/list", get(admin::list_admins))
}
