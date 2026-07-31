use axum::{
    Json,
    extract::{Path, Query, State},
};
use sqlx::PgPool;
use tracing::info;
use validator::Validate;

use crate::db;
use crate::error::AppError;
use crate::models::carts::{
    CartActionRequest, CartActionResponse, CartItemResponse, CartQuery, CartResponse,
    StatusResponse,
};

// GET /api/v1/cart?user_id=123456789
pub async fn get_cart(
    State(pool): State<PgPool>,
    Query(query): Query<CartQuery>,
) -> Result<Json<CartResponse>, AppError> {
    query.validate()?;

    let records = db::carts::get_cart(&pool, query.user_id).await?;

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
) -> Result<Json<CartActionResponse>, AppError> {
    payload.validate()?;

    let new_quantity = db::carts::add_to_cart(&pool, payload.user_id, payload.product_id).await?;

    info!(
        user_id = payload.user_id,
        product_id = payload.product_id,
        new_quantity = new_quantity,
        "Товар добавлен в корзину"
    );

    Ok(Json(CartActionResponse {
        status: "success".to_string(),
        product_id: payload.product_id,
        current_quantity: new_quantity,
    }))
}

// POST /api/v1/cart/decrement
pub async fn decrement_cart(
    State(pool): State<PgPool>,
    Json(payload): Json<CartActionRequest>,
) -> Result<Json<CartActionResponse>, AppError> {
    payload.validate()?;

    let record = db::carts::decrement_cart(&pool, payload.user_id, payload.product_id).await?;

    let current_quantity = record.unwrap_or(0);

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
) -> Result<Json<StatusResponse>, AppError> {
    query.validate()?;

    db::carts::remove_from_cart(&pool, query.user_id, product_id).await?;

    info!(
        user_id = query.user_id,
        product_id = product_id,
        "Товар удалён из корзины"
    );

    Ok(Json(StatusResponse {
        status: "success".to_string(),
    }))
}
