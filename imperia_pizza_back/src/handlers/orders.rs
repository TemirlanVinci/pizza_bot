use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;
use tracing::{info, instrument};

use crate::db;
use crate::error::AppError;
use crate::models::orders::{
    CreateOrderRequest, CreateOrderResponse, OrderDetailResponse, OrderItemResponse,
    UserOrderResponse,
};

/// POST /api/v1/orders
#[instrument(skip(pool))]
pub async fn create_order(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<CreateOrderResponse>, AppError> {
    if payload.delivery_type != "delivery" && payload.delivery_type != "pickup" {
        return Err(AppError::Validation(
            "Некорректный тип доставки (ожидается 'delivery' или 'pickup')".to_string(),
        ));
    }

    if payload.payment_method != "cash" && payload.payment_method != "visa_courier" {
        return Err(AppError::Validation(
            "Некорректный способ оплаты (ожидается 'cash' или 'visa_courier')".to_string(),
        ));
    }

    if payload.user_name.trim().is_empty()
        || payload.phone_number.trim().is_empty()
        || payload.address.trim().is_empty()
    {
        return Err(AppError::Validation(
            "Заполните все обязательные поля (user_name, phone_number, address)".to_string(),
        ));
    }

    // Получаем товары из корзины пользователя
    let cart_items = db::orders::get_cart_items_for_order(&pool, payload.user_id).await?;

    if cart_items.is_empty() {
        return Err(AppError::Validation("Корзина пуста".to_string()));
    }

    let total_price: i32 = cart_items
        .iter()
        .map(|item| item.price * item.quantity)
        .sum();

    // Транзакционное создание заказа
    let mut tx = pool.begin().await?;

    db::orders::ensure_user_exists_tx(&mut tx, payload.user_id).await?;

    let order_header = db::orders::create_order_header_tx(
        &mut tx,
        db::orders::CreateOrderHeaderParams {
            user_id: payload.user_id,
            user_name: &payload.user_name,
            phone_number: &payload.phone_number,
            delivery_type: &payload.delivery_type,
            address: &payload.address,
            payment_method: &payload.payment_method,
            total_price,
        },
    )
    .await?;

    let order_id = order_header.id;
    let created_at = order_header.created_at;

    for item in &cart_items {
        db::orders::insert_order_item_tx(
            &mut tx,
            order_id,
            item.product_id,
            &item.name,
            item.quantity,
            item.price,
        )
        .await?;
    }

    db::orders::save_user_phone_tx(&mut tx, payload.user_id, &payload.phone_number).await?;
    db::orders::save_user_address_tx(&mut tx, payload.user_id, &payload.address).await?;
    db::orders::clear_cart_tx(&mut tx, payload.user_id).await?;

    tx.commit().await?;

    info!(
        order_id,
        user_id = payload.user_id,
        total_price,
        "Заказ успешно создан"
    );

    let admin_tg_ids = db::orders::get_active_admin_ids(&pool)
        .await
        .unwrap_or_default();

    let response_items: Vec<OrderItemResponse> = cart_items
        .into_iter()
        .map(|item| OrderItemResponse {
            product_id: item.product_id,
            name: item.name,
            quantity: item.quantity,
            price_at_purchase: item.price,
        })
        .collect();

    Ok(Json(CreateOrderResponse {
        status: "success".to_string(),
        order_id,
        total_price,
        delivery_type: payload.delivery_type,
        address: payload.address,
        user_name: payload.user_name,
        phone_number: payload.phone_number,
        payment_method: payload.payment_method,
        created_at,
        admin_tg_ids,
        items: response_items,
    }))
}

/// GET /api/v1/orders/user/{user_id}
#[instrument(skip(pool))]
pub async fn get_user_orders(
    State(pool): State<PgPool>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<UserOrderResponse>>, AppError> {
    let records = db::orders::get_user_orders(&pool, user_id).await?;

    let orders = records
        .into_iter()
        .map(|r| UserOrderResponse {
            order_id: r.order_id,
            status: r.status,
            total_price: r.total_price,
            created_at: r.created_at,
        })
        .collect();

    Ok(Json(orders))
}

/// GET /api/v1/orders/detail/{order_id}
#[instrument(skip(pool))]
pub async fn get_order_detail(
    State(pool): State<PgPool>,
    Path(order_id): Path<i32>,
) -> Result<Json<OrderDetailResponse>, AppError> {
    let order_header = db::orders::get_order_detail_header(&pool, order_id).await?;

    let order = match order_header {
        Some(header) => header,
        None => return Err(AppError::NotFound),
    };

    let items_records = db::orders::get_order_items(&pool, order_id).await?;

    let items = items_records
        .into_iter()
        .map(|item| OrderItemResponse {
            product_id: item.product_id,
            name: item.name,
            quantity: item.quantity,
            price_at_purchase: item.price_at_purchase,
        })
        .collect();

    Ok(Json(OrderDetailResponse {
        order_id: order.order_id,
        status: order.status,
        delivery_type: order.delivery_type,
        address: order.address,
        user_name: order.user_name,
        phone_number: order.phone_number,
        total_price: order.total_price,
        created_at: order.created_at,
        items,
    }))
}
