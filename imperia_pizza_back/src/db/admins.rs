use crate::models::admins::{ActiveOrderResponse, AdminItem, OrderItemResponse};
use sqlx::{PgPool, postgres::PgQueryResult};

pub async fn check_admin_active(pool: &PgPool, admin_tg_id: i64) -> Result<bool, sqlx::Error> {
    let admin = sqlx::query!(
        r#"
        SELECT telegram_id
        FROM admins
        WHERE telegram_id = $1 AND is_active = true
"#,
        admin_tg_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(admin.is_some())
}

pub async fn update_order_status(
    pool: &PgPool,
    order_id: i32,
    status: &str,
) -> Result<Option<i32>, sqlx::Error> {
    let updated = sqlx::query!(
        r#"
        UPDATE orders
        SET status = $1
        WHERE id = $2
        RETURNING id
"#,
        status,
        order_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(updated.map(|r| r.id))
}

pub async fn ban_user(
    pool: &PgPool,
    user_id: i64,
    phone_number: Option<&str>,
    ban_reason: Option<&str>,
    admin_tg_id: i64,
) -> Result<PgQueryResult, sqlx::Error> {
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
        user_id,
        phone_number,
        ban_reason,
        admin_tg_id
    )
    .execute(pool)
    .await
}

/// Активные заказы + корзина (order_items) каждого заказа.
/// items собирается через json_agg на стороне Postgres (LEFT JOIN + GROUP BY),
/// чтобы не плодить дубликаты строк по заказу и не агрегировать вручную в Rust.
/// FILTER (WHERE oi.id IS NOT NULL) нужен, чтобы у заказа без позиций
/// items был пустым массивом, а не массивом с одним "пустым" объектом
/// (LEFT JOIN даёт NULL-строку oi, если позиций нет).
pub async fn get_active_orders(pool: &PgPool) -> Result<Vec<ActiveOrderResponse>, sqlx::Error> {
    let records = sqlx::query!(
        r#"
        SELECT
            o.id AS order_id,
            o.status,
            o.delivery_type,
            o.address,
            o.phone_number,
            o.user_id,
            o.user_name,
            o.total_price,
            COALESCE(to_char(o.created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"'), '') AS created_at,
            COALESCE(
                json_agg(
                    json_build_object(
                        'product_id', oi.product_id,
                        'name', oi.name,
                        'quantity', oi.quantity,
                        'price_at_purchase', oi.price_at_purchase
                    ) ORDER BY oi.id
                ) FILTER (WHERE oi.id IS NOT NULL),
                '[]'
            ) AS "items!: sqlx::types::Json<Vec<OrderItemResponse>>"
        FROM orders o
        LEFT JOIN order_items oi ON oi.order_id = o.id
        WHERE o.status NOT IN ('completed', 'cancelled')
        GROUP BY o.id
        ORDER BY o.created_at DESC
"#
    )
    .fetch_all(pool)
    .await?;

    Ok(records
        .into_iter()
        .map(|rec| ActiveOrderResponse {
            order_id: rec.order_id,
            status: rec.status,
            delivery_type: rec.delivery_type,
            address: rec.address,
            phone_number: rec.phone_number,
            user_id: rec.user_id,
            user_name: rec.user_name,
            total_price: rec.total_price,
            created_at: rec.created_at.unwrap_or_default(),
            items: rec.items.0,
        })
        .collect())
}

pub async fn get_broadcast_user_ids(pool: &PgPool) -> Result<Vec<i64>, sqlx::Error> {
    let records = sqlx::query!(
        r#"
        SELECT telegram_id
        FROM users
        ORDER BY id ASC
"#
    )
    .fetch_all(pool)
    .await?;

    Ok(records.into_iter().map(|rec| rec.telegram_id).collect())
}

pub async fn list_admins(pool: &PgPool) -> Result<Vec<AdminItem>, sqlx::Error> {
    let records = sqlx::query!(
        r#"
        SELECT telegram_id, name, is_active
        FROM admins
        ORDER BY telegram_id ASC
"#
    )
    .fetch_all(pool)
    .await?;

    Ok(records
        .into_iter()
        .map(|rec| AdminItem {
            telegram_id: rec.telegram_id,
            name: rec.name,
            is_active: rec.is_active,
        })
        .collect())
}
