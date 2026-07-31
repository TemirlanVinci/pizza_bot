use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;
use tracing::{info, instrument};
use validator::Validate;

use crate::db;
use crate::error::AppError;
use crate::models::admins::{
    ActiveOrderResponse, AdminCheckRequest, AdminListResponse, BanUserRequest,
    BroadcastUsersResponse, StatusSuccessResponse, UpdateOrderStatusRequest,
    UpdateOrderStatusResponse,
};

/// PATCH /api/v1/admin/orders/{order_id}/status
#[instrument(skip(pool))]
pub async fn update_order_status(
    State(pool): State<PgPool>,
    Path(order_id): Path<i32>,
    Json(payload): Json<UpdateOrderStatusRequest>,
) -> Result<Json<UpdateOrderStatusResponse>, AppError> {
    payload.validate()?;

    let valid_statuses = [
        "confirmed",
        "cooking",
        "delivering",
        "completed",
        "cancelled",
    ];
    if !valid_statuses.contains(&payload.status.as_str()) {
        return Err(AppError::Validation(format!(
            "Некорректный статус заказа: '{}'. Допустимые: confirmed, cooking, delivering, completed, cancelled",
            payload.status
        )));
    }

    // Проверка прав администратора
    let is_admin = db::admins::check_admin_active(&pool, payload.admin_tg_id).await?;
    if !is_admin {
        return Err(AppError::Unauthorized);
    }

    // Обновление статуса заказа
    let updated_id = db::admins::update_order_status(&pool, order_id, &payload.status).await?;
    if updated_id.is_none() {
        return Err(AppError::NotFound);
    }

    info!(
        order_id = order_id,
        new_status = payload.status,
        admin_tg_id = payload.admin_tg_id,
        "Статус заказа успешно обновлен"
    );

    Ok(Json(UpdateOrderStatusResponse {
        status: "success".to_string(),
        order_id,
        new_status: payload.status,
    }))
}

/// POST /api/v1/admin/users/ban
#[instrument(skip(pool))]
pub async fn ban_user(
    State(pool): State<PgPool>,
    Json(payload): Json<BanUserRequest>,
) -> Result<Json<StatusSuccessResponse>, AppError> {
    payload.validate()?;

    // Проверка прав администратора
    let is_admin = db::admins::check_admin_active(&pool, payload.admin_tg_id).await?;
    if !is_admin {
        return Err(AppError::Unauthorized);
    }

    db::admins::ban_user(
        &pool,
        payload.user_id,
        payload.phone_number.as_deref(),
        payload.ban_reason.as_deref(),
        payload.admin_tg_id,
    )
    .await?;

    info!(
        user_id = payload.user_id,
        banned_by = payload.admin_tg_id,
        "Пользователь успешно заблокирован"
    );

    Ok(Json(StatusSuccessResponse {
        status: "success".to_string(),
    }))
}

/// POST /api/v1/admin/orders/active
#[instrument(skip(pool))]
pub async fn get_active_orders(
    State(pool): State<PgPool>,
    payload: Option<Json<AdminCheckRequest>>,
) -> Result<Json<Vec<ActiveOrderResponse>>, AppError> {
    if let Some(Json(ref req)) = payload {
        req.validate()?;
    }

    if let Some(Json(AdminCheckRequest {
        admin_tg_id: Some(admin_tg_id),
    })) = payload
    {
        let is_admin = db::admins::check_admin_active(&pool, admin_tg_id).await?;
        if !is_admin {
            return Err(AppError::Unauthorized);
        }
    }

    let orders = db::admins::get_active_orders(&pool).await?;

    Ok(Json(orders))
}

/// POST /api/v1/admin/broadcast/users
#[instrument(skip(pool))]
pub async fn get_broadcast_users(
    State(pool): State<PgPool>,
    payload: Option<Json<AdminCheckRequest>>,
) -> Result<Json<BroadcastUsersResponse>, AppError> {
    if let Some(Json(ref req)) = payload {
        req.validate()?;
    }

    if let Some(Json(AdminCheckRequest {
        admin_tg_id: Some(admin_tg_id),
    })) = payload
    {
        let is_admin = db::admins::check_admin_active(&pool, admin_tg_id).await?;
        if !is_admin {
            return Err(AppError::Unauthorized);
        }
    }

    let user_ids = db::admins::get_broadcast_user_ids(&pool).await?;

    info!(
        count = user_ids.len(),
        "Список пользователей для рассылки получен"
    );

    Ok(Json(BroadcastUsersResponse { user_ids }))
}

/// GET /api/v1/admin/list
#[instrument(skip(pool))]
pub async fn list_admins(State(pool): State<PgPool>) -> Result<Json<AdminListResponse>, AppError> {
    let admins = db::admins::list_admins(&pool).await?;

    info!(
        count = admins.len(),
        "Список администраторов успешно получен"
    );

    Ok(Json(AdminListResponse { admins }))
}
