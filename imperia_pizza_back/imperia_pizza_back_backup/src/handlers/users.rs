use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use tracing::{error, info};
use crate::models::users::{RegisterUserRequest, UserStatusResponse};

/// POST /api/v1/users/register
pub async fn register_user(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<UserStatusResponse>, StatusCode> {
    // Используем ON CONFLICT DO NOTHING, чтобы повторные вызовы /start
    // не вызывали ошибок уникальности (UNIQUE constraint violation).
    sqlx::query!(
        r#"
        INSERT INTO users (telegram_id)
        VALUES ($1)
        ON CONFLICT (telegram_id) DO NOTHING
        "#,
        payload.telegram_id
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        error!(
            telegram_id = payload.telegram_id,
            error = %e,
            "Не удалось зарегистрировать пользователя в БД"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!(telegram_id = payload.telegram_id, "Пользователь зарегистрирован/уже существует");

    Ok(Json(UserStatusResponse {
        status: "success".to_string(),
    }))
}