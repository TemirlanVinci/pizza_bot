use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;
use tracing::{error, info, instrument};

use crate::models::orders::{
    CreateOrderRequest, CreateOrderResponse, NewOrderNotificationRequest,
    NewOrderNotificationResponse, OrderDetailResponse, OrderItemResponse, UserOrderResponse,
};

/// POST /api/v1/orders
#[instrument(skip(pool))]
pub async fn create_order(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<CreateOrderResponse>, (StatusCode, String)> {
    if payload.delivery_type != "delivery" && payload.delivery_type != "pickup" {
        return Err((
            StatusCode::BAD_REQUEST,
            "Некорректный тип доставки (ожидается 'delivery' или 'pickup')".to_string(),
        ));
    }

    if payload.payment_method != "cash" && payload.payment_method != "visa_courier" {
        return Err((
            StatusCode::BAD_REQUEST,
            "Некорректный способ оплаты (ожидается 'cash' или 'visa_courier')".to_string(),
        ));
    }

    if payload.user_name.trim().is_empty()
        || payload.phone_number.trim().is_empty()
        || payload.address.trim().is_empty()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "Заполните все обязательные поля (user_name, phone_number, address)".to_string(),
        ));
    }

    // Получаем товары из корзины пользователя
    let cart_items = sqlx::query!(
        r#"
        SELECT c.product_id, p.name, p.price, c.quantity
        FROM cart_items c
        JOIN products p ON c.product_id = p.id
        WHERE c.user_id = $1
        "#,
        payload.user_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!(user_id = payload.user_id, error = %e, "Не удалось получить корзину при создании заказа");
        (StatusCode::INTERNAL_SERVER_ERROR, "Внутренняя ошибка сервера".to_string())
    })?;

    if cart_items.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Корзина пуста".to_string()));
    }

    let total_price: i32 = cart_items
        .iter()
        .map(|item| item.price * item.quantity)
        .sum();

    // Транзакционное создание заказа
    let mut tx = pool.begin().await.map_err(|e| {
        error!(error = %e, "Не удалось начать транзакцию");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Внутренняя ошибка сервера".to_string(),
        )
    })?;

    // Убедимся, что пользователь существует в таблице users
    sqlx::query!(
        r#"
        INSERT INTO users (telegram_id)
        VALUES ($1)
        ON CONFLICT (telegram_id) DO NOTHING
        "#,
        payload.user_id
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        error!(user_id = payload.user_id, error = %e, "Не удалось зарегистрировать пользователя при заказе");
        (StatusCode::INTERNAL_SERVER_ERROR, "Внутренняя ошибка сервера".to_string())
    })?;

    // Создаём заказ
    let order_record = sqlx::query!(
        r#"
        INSERT INTO orders (user_id, user_name, phone_number, delivery_type, address, payment_method, status, total_price)
        VALUES ($1, $2, $3, $4, $5, $6, 'confirmed', $7)
        RETURNING id
        "#,
        payload.user_id,
        payload.user_name,
        payload.phone_number,
        payload.delivery_type,
        payload.address,
        payload.payment_method,
        total_price
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        error!(user_id = payload.user_id, error = %e, "Не удалось создать заказ в БД");
        (StatusCode::INTERNAL_SERVER_ERROR, "Внутренняя ошибка сервера".to_string())
    })?;

    let order_id = order_record.id;

    // Вставляем товары заказа
    for item in cart_items {
        sqlx::query!(
            r#"
            INSERT INTO order_items (order_id, product_id, name, quantity, price_at_purchase)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            order_id,
            item.product_id,
            item.name,
            item.quantity,
            item.price
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!(order_id, error = %e, "Не удалось сохранить позицию заказа");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Внутренняя ошибка сервера".to_string(),
            )
        })?;
    }

    // Сохраняем телефон и адрес в историй пользователя
    sqlx::query!(
        r#"
        INSERT INTO user_phones (user_id, phone_number)
        VALUES ($1, $2)
        "#,
        payload.user_id,
        payload.phone_number
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        error!(user_id = payload.user_id, error = %e, "Не удалось сохранить телефон в user_phones");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Внутренняя ошибка сервера".to_string(),
        )
    })?;

    sqlx::query!(
        r#"
        INSERT INTO user_addresses (user_id, address)
        VALUES ($1, $2)
        "#,
        payload.user_id,
        payload.address
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        error!(user_id = payload.user_id, error = %e, "Не удалось сохранить адрес в user_addresses");
        (StatusCode::INTERNAL_SERVER_ERROR, "Внутренняя ошибка сервера".to_string())
    })?;

    // Очищаем корзину пользователя
    sqlx::query!(
        r#"
        DELETE FROM cart_items
        WHERE user_id = $1
        "#,
        payload.user_id
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        error!(user_id = payload.user_id, error = %e, "Не удалось очистить корзину");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Внутренняя ошибка сервера".to_string(),
        )
    })?;

    tx.commit().await.map_err(|e| {
        error!(error = %e, "Не удалось зафиксировать транзакцию создания заказа");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Внутренняя ошибка сервера".to_string(),
        )
    })?;

    info!(
        order_id,
        user_id = payload.user_id,
        total_price,
        "Заказ успешно создан"
    );

    Ok(Json(CreateOrderResponse {
        status: "success".to_string(),
        order_id,
        total_price,
    }))
}

/// GET /api/v1/orders/user/{user_id}
#[instrument(skip(pool))]
pub async fn get_user_orders(
    State(pool): State<PgPool>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<UserOrderResponse>>, (StatusCode, String)> {
    let records = sqlx::query!(
        r#"
        SELECT 
            id AS order_id, 
            status, 
            total_price, 
            COALESCE(to_char(created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"'), '') AS created_at
        FROM orders
        WHERE user_id = $1
        ORDER BY id DESC
        "#,
        user_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!(user_id, error = %e, "Не удалось получить список заказов пользователя");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Внутренняя ошибка сервера".to_string(),
        )
    })?;

    let orders = records
        .into_iter()
        .map(|r| UserOrderResponse {
            order_id: r.order_id,
            status: r.status,
            total_price: r.total_price,
            created_at: r.created_at.unwrap_or_default(),
        })
        .collect();

    Ok(Json(orders))
}

/// GET /api/v1/orders/detail/{order_id}
#[instrument(skip(pool))]
pub async fn get_order_detail(
    State(pool): State<PgPool>,
    Path(order_id): Path<i32>,
) -> Result<Json<OrderDetailResponse>, (StatusCode, String)> {
    let order_record = sqlx::query!(
        r#"
        SELECT 
            id AS order_id, 
            status, 
            delivery_type, 
            address, 
            user_name, 
            phone_number, 
            total_price, 
            COALESCE(to_char(created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"'), '') AS created_at
        FROM orders
        WHERE id = $1
        "#,
        order_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error!(order_id, error = %e, "Не удалось получить детали заказа");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Внутренняя ошибка сервера".to_string(),
        )
    })?;

    let order = match order_record {
        Some(rec) => rec,
        None => return Err((StatusCode::NOT_FOUND, "Заказ не найден".to_string())),
    };

    let items_records = sqlx::query!(
        r#"
        SELECT 
            product_id, 
            name, 
            quantity, 
            price_at_purchase
        FROM order_items
        WHERE order_id = $1
        ORDER BY id ASC
        "#,
        order_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!(order_id, error = %e, "Не удалось получить позиции заказа");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Внутренняя ошибка сервера".to_string(),
        )
    })?;

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
        created_at: order.created_at.unwrap_or_default(),
        items,
    }))
}

/// POST /api/v1/internal/new-order-notification
#[instrument(skip(pool))]
pub async fn internal_new_order_notification(
    State(pool): State<PgPool>,
    Json(payload): Json<NewOrderNotificationRequest>,
) -> Result<Json<NewOrderNotificationResponse>, (StatusCode, String)> {
    let order_record = sqlx::query!(
        r#"
        SELECT 
            id AS order_id, 
            delivery_type, 
            address, 
            user_name, 
            phone_number, 
            payment_method, 
            total_price, 
            COALESCE(to_char(created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"'), '') AS created_at
        FROM orders
        WHERE id = $1
        "#,
        payload.order_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error!(order_id = payload.order_id, error = %e, "Не удалось получить данные заказа для уведомления");
        (StatusCode::INTERNAL_SERVER_ERROR, "Внутренняя ошибка сервера".to_string())
    })?;

    let order = match order_record {
        Some(rec) => rec,
        None => return Err((StatusCode::NOT_FOUND, "Заказ не найден".to_string())),
    };

    let admin_records = sqlx::query!(
        r#"
        SELECT telegram_id
        FROM admins
        WHERE is_active = true
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

    let admin_tg_ids = admin_records.into_iter().map(|a| a.telegram_id).collect();

    let items_records = sqlx::query!(
        r#"
        SELECT 
            product_id, 
            name, 
            quantity, 
            price_at_purchase
        FROM order_items
        WHERE order_id = $1
        ORDER BY id ASC
        "#,
        payload.order_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!(order_id = payload.order_id, error = %e, "Не удалось получить позиции заказа для уведомления");
        (StatusCode::INTERNAL_SERVER_ERROR, "Внутренняя ошибка сервера".to_string())
    })?;

    let items = items_records
        .into_iter()
        .map(|item| OrderItemResponse {
            product_id: item.product_id,
            name: item.name,
            quantity: item.quantity,
            price_at_purchase: item.price_at_purchase,
        })
        .collect();

    Ok(Json(NewOrderNotificationResponse {
        order_id: order.order_id,
        delivery_type: order.delivery_type,
        address: order.address,
        user_name: order.user_name,
        phone_number: order.phone_number,
        payment_method: order.payment_method,
        total_price: order.total_price,
        created_at: order.created_at.unwrap_or_default(),
        admin_tg_ids,
        items,
    }))
}
