use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;
use tracing::{error, info, instrument};

use crate::models::admin::{
    ActiveOrderResponse, AdminCheckRequest, AdminItem, AdminListResponse, BanUserRequest,
    BroadcastUsersResponse, StatusSuccessResponse, UpdateOrderStatusRequest,
    UpdateOrderStatusResponse,
};

/// PATCH /api/v1/admin/orders/{order_id}/status
#[instrument(skip(pool))]
pub async fn update_order_status(
    State(pool): State<PgPool>,
    Path(order_id): Path<i32>,
    Json(payload): Json<UpdateOrderStatusRequest>,
) -> Result<Json<UpdateOrderStatusResponse>, (StatusCode, String)> {
    let valid_statuses = [
        "confirmed",
        "cooking",
        "delivering",
        "completed",
        "cancelled",
    ];
    if !valid_statuses.contains(&payload.status.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "Некорректный статус заказа: '{}'. Допустимые: confirmed, cooking, delivering, completed, cancelled",
                payload.status
            ),
        ));
    }

    // Проверка прав администратора
    let admin = sqlx::query!(
        r#"
        SELECT telegram_id
        FROM admins
        WHERE telegram_id = $1 AND is_active = true
        "#,
        payload.admin_tg_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error!(
            admin_tg_id = payload.admin_tg_id,
            error = %e,
            "Ошибка проверки прав администратора"
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Внутренняя ошибка сервера".to_string(),
        )
    })?;

    if admin.is_none() {
        return Err((
            StatusCode::FORBIDDEN,
            "Доступ запрещен: администратор не найден или не активен".to_string(),
        ));
    }

    // Обновление статуса заказа
    let updated = sqlx::query!(
        r#"
        UPDATE orders
        SET status = $1
        WHERE id = $2
        RETURNING id
        "#,
        payload.status,
        order_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error!(
            order_id = order_id,
            error = %e,
            "Не удалось обновить статус заказа"
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Внутренняя ошибка сервера".to_string(),
        )
    })?;

    if updated.is_none() {
        return Err((StatusCode::NOT_FOUND, "Заказ не найден".to_string()));
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
) -> Result<Json<StatusSuccessResponse>, (StatusCode, String)> {
    // Проверка прав администратора
    let admin = sqlx::query!(
        r#"
        SELECT telegram_id
        FROM admins
        WHERE telegram_id = $1 AND is_active = true
        "#,
        payload.admin_tg_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error!(
            admin_tg_id = payload.admin_tg_id,
            error = %e,
            "Ошибка проверки прав администратора"
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Внутренняя ошибка сервера".to_string(),
        )
    })?;

    if admin.is_none() {
        return Err((
            StatusCode::FORBIDDEN,
            "Доступ запрещен: администратор не найден или не активен".to_string(),
        ));
    }

    sqlx::query!(
        r#"
        INSERT INTO banned_users (telegram_id, phone_number, ban_reason, banned_by)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (telegram_id) DO UPDATE
        SET phone_number = EXCLUDED.phone_number,
            ban_reason = EXCLUDED.ban_reason,
            banned_by = EXCLUDED.banned_by,
            banned_at = NOW()
        "#,
        payload.user_id,
        payload.phone_number,
        payload.ban_reason,
        payload.admin_tg_id
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        error!(
            user_id = payload.user_id,
            error = %e,
            "Не удалось заблокировать пользователя"
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Внутренняя ошибка сервера".to_string(),
        )
    })?;

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
) -> Result<Json<Vec<ActiveOrderResponse>>, (StatusCode, String)> {
    if let Some(Json(AdminCheckRequest {
        admin_tg_id: Some(admin_tg_id),
    })) = payload
    {
        let admin = sqlx::query!(
            r#"
            SELECT telegram_id
            FROM admins
            WHERE telegram_id = $1 AND is_active = true
            "#,
            admin_tg_id
        )
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error!(admin_tg_id = admin_tg_id, error = %e, "Ошибка проверки прав администратора");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Внутренняя ошибка сервера".to_string(),
            )
        })?;

        if admin.is_none() {
            return Err((
                StatusCode::FORBIDDEN,
                "Доступ запрещен: администратор не найден или не активен".to_string(),
            ));
        }
    }

    let records = sqlx::query!(
        r#"
        SELECT 
            id AS order_id,
            status,
            delivery_type,
            address,
            phone_number,
            total_price,
            COALESCE(to_char(created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"'), '') AS created_at
        FROM orders
        WHERE status NOT IN ('completed', 'cancelled')
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!(error = %e, "Не удалось получить список активных заказов");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Внутренняя ошибка сервера".to_string(),
        )
    })?;

    let orders: Vec<ActiveOrderResponse> = records
        .into_iter()
        .map(|rec| ActiveOrderResponse {
            order_id: rec.order_id,
            status: rec.status,
            delivery_type: rec.delivery_type,
            address: rec.address,
            phone_number: rec.phone_number,
            total_price: rec.total_price,
            created_at: rec.created_at.unwrap_or_default(),
        })
        .collect();

    Ok(Json(orders))
}

/// POST /api/v1/admin/broadcast/users
#[instrument(skip(pool))]
pub async fn get_broadcast_users(
    State(pool): State<PgPool>,
    payload: Option<Json<AdminCheckRequest>>,
) -> Result<Json<BroadcastUsersResponse>, (StatusCode, String)> {
    if let Some(Json(AdminCheckRequest {
        admin_tg_id: Some(admin_tg_id),
    })) = payload
    {
        let admin = sqlx::query!(
            r#"
            SELECT telegram_id
            FROM admins
            WHERE telegram_id = $1 AND is_active = true
            "#,
            admin_tg_id
        )
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error!(admin_tg_id = admin_tg_id, error = %e, "Ошибка проверки прав администратора");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Внутренняя ошибка сервера".to_string(),
            )
        })?;

        if admin.is_none() {
            return Err((
                StatusCode::FORBIDDEN,
                "Доступ запрещен: администратор не найден или не активен".to_string(),
            ));
        }
    }

    let records = sqlx::query!(
        r#"
        SELECT telegram_id
        FROM users
        ORDER BY id ASC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!(error = %e, "Не удалось получить список пользователей для рассылки");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Внутренняя ошибка сервера".to_string(),
        )
    })?;

    let user_ids: Vec<i64> = records.into_iter().map(|rec| rec.telegram_id).collect();

    info!(
        count = user_ids.len(),
        "Список пользователей для рассылки получен"
    );

    Ok(Json(BroadcastUsersResponse { user_ids }))
}

/// GET /api/v1/admin/list
#[instrument(skip(pool))]
pub async fn list_admins(
    State(pool): State<PgPool>,
) -> Result<Json<AdminListResponse>, (StatusCode, String)> {
    let records = sqlx::query!(
        r#"
        SELECT telegram_id, name, is_active
        FROM admins
        ORDER BY telegram_id ASC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!(error = %e, "Не удалось получить список администраторов");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Внутренняя ошибка сервера".to_string(),
        )
    })?;

    let admins: Vec<AdminItem> = records
        .into_iter()
        .map(|rec| AdminItem {
            telegram_id: rec.telegram_id,
            name: rec.name,
            is_active: rec.is_active,
        })
        .collect();

    info!(
        count = admins.len(),
        "Список администраторов успешно получен"
    );

    Ok(Json(AdminListResponse { admins }))
}
