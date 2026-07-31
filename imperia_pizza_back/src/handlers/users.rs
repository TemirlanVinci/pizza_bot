use crate::db;
use crate::error::AppError;
use crate::models::users::{RegisterUserRequest, UserStatusResponse};
use axum::{Json, extract::State};
use sqlx::PgPool;
use tracing::info;

/// POST /api/v1/users/register
pub async fn register_user(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<UserStatusResponse>, AppError> {
    db::users::register_user(&pool, payload.telegram_id).await?;

    info!(
        telegram_id = payload.telegram_id,
        "Пользователь зарегистрирован/уже существует"
    );

    Ok(Json(UserStatusResponse {
        status: "success".to_string(),
    }))
}
