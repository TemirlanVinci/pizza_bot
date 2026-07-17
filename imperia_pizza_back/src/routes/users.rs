use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::handlers::users;

/// Роуты домена "пользователи"
pub fn router() -> Router<PgPool> {
    Router::new().route("/users/register", post(users::register_user))
}
