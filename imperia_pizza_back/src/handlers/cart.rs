use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use sqlx::PgPool;
use tracing::{error, info};

use crate::models::cart::{
    CartActionRequest, CartActionResponse, CartItemResponse, CartQuery, CartResponse,
    StatusResponse,
};

// GET /api/v1/cart?user_id=123456789
pub async fn get_cart(
    State(pool): State<PgPool>,
    Query(query): Query<CartQuery>,
) -> Result<Json<CartResponse>, StatusCode> {
    // Делаем JOIN с products, чтобы получить цену и название
    let records = sqlx::query!(
        r#"
        SELECT 
            p.id as product_id, 
            p.name, 
            p.price, 
            c.quantity 
        FROM cart_items c
        JOIN products p ON c.product_id = p.id
        WHERE c.user_id = $1
        "#,
        query.user_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!(user_id = query.user_id, error = %e, "Не удалось получить корзину из БД");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut items = Vec::with_capacity(records.len());
    let mut total_quantity = 0;
    let mut final_price = 0;

    for rec in records {
        let total_item_price = rec.price * rec.quantity;
        total_quantity += rec.quantity;
        final_price += total_item_price;

        items.push(CartItemResponse {
            product_id: rec.product_id,
            name: rec.name,
            price: rec.price,
            quantity: rec.quantity,
            total_item_price,
        });
    }

    Ok(Json(CartResponse {
        items,
        total_quantity,
        final_price,
    }))
}

// POST /api/v1/cart/add
pub async fn add_to_cart(
    State(pool): State<PgPool>,
    Json(payload): Json<CartActionRequest>,
) -> Result<Json<CartActionResponse>, StatusCode> {
    let record = sqlx::query!(
        r#"
        INSERT INTO cart_items (user_id, product_id, quantity)
        VALUES ($1, $2, 1)
        ON CONFLICT (user_id, product_id)
        DO UPDATE SET quantity = cart_items.quantity + 1
        RETURNING quantity
        "#,
        payload.user_id,
        payload.product_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!(
            user_id = payload.user_id,
            product_id = payload.product_id,
            error = %e,
            "Не удалось добавить товар в корзину"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!(
        user_id = payload.user_id,
        product_id = payload.product_id,
        new_quantity = record.quantity,
        "Товар добавлен в корзину"
    );

    Ok(Json(CartActionResponse {
        status: "success".to_string(),
        product_id: payload.product_id,
        current_quantity: record.quantity,
    }))
}

// POST /api/v1/cart/decrement
pub async fn decrement_cart(
    State(pool): State<PgPool>,
    Json(payload): Json<CartActionRequest>,
) -> Result<Json<CartActionResponse>, StatusCode> {
    let record = sqlx::query!(
        r#"
        WITH updated AS (
            UPDATE cart_items
            SET quantity = quantity - 1
            WHERE user_id = $1 AND product_id = $2 AND quantity > 0
            RETURNING quantity
        ),
        deleted AS (
            DELETE FROM cart_items
            WHERE user_id = $1 AND product_id = $2 AND quantity = 0
        )
        SELECT quantity FROM updated
        "#,
        payload.user_id,
        payload.product_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error!(
            user_id = payload.user_id,
            product_id = payload.product_id,
            error = %e,
            "Не удалось уменьшить количество товара в корзине"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let current_quantity = record.map(|r| r.quantity).unwrap_or(0);

    info!(
        user_id = payload.user_id,
        product_id = payload.product_id,
        new_quantity = current_quantity,
        "Количество товара в корзине уменьшено"
    );

    Ok(Json(CartActionResponse {
        status: "success".to_string(),
        product_id: payload.product_id,
        current_quantity,
    }))
}

// DELETE /api/v1/cart/item/14?user_id=123456789
pub async fn remove_from_cart(
    State(pool): State<PgPool>,
    Path(product_id): Path<i32>,
    Query(query): Query<CartQuery>,
) -> Result<Json<StatusResponse>, StatusCode> {
    sqlx::query!(
        r#"
        DELETE FROM cart_items
        WHERE user_id = $1 AND product_id = $2
        "#,
        query.user_id,
        product_id
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        error!(
            user_id = query.user_id,
            product_id = product_id,
            error = %e,
            "Не удалось удалить товар из корзины"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!(
        user_id = query.user_id,
        product_id = product_id,
        "Товар удалён из корзины"
    );

    Ok(Json(StatusResponse {
        status: "success".to_string(),
    }))
}
