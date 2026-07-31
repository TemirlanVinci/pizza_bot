use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;
use tracing::{error, info, instrument};

use crate::models::orders::{
    CreateOrderRequest, CreateOrderResponse, OrderDetailResponse, OrderItemResponse,
    UserOrderResponse,
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

    // Создаём заказ и сразу получаем дату создания
    let order_record = sqlx::query!(
        r#"
        INSERT INTO orders (user_id, user_name, phone_number, delivery_type, address, payment_method, status, total_price)
        VALUES ($1, $2, $3, $4, $5, $6, 'confirmed', $7)
        RETURNING id, COALESCE(to_char(created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"'), '') AS created_at
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
    let created_at = order_record.created_at.unwrap_or_default();

    // Вставляем товары заказа (итерируемся по ссылке, чтобы не уничтожить cart_items)
    for item in &cart_items {
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

    // Сохраняем телефон в историю пользователя, ТОЛЬКО ЕСЛИ его там еще нет
    sqlx::query!(
        r#"
        INSERT INTO user_phones (user_id, phone_number)
        SELECT $1::BIGINT, $2::VARCHAR
        WHERE NOT EXISTS (
            SELECT 1 FROM user_phones 
            WHERE user_id = $1 AND phone_number = $2
        )
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

    // Сохраняем адрес в историю пользователя, ТОЛЬКО ЕСЛИ его там еще нет
    sqlx::query!(
        r#"
        INSERT INTO user_addresses (user_id, address)
        SELECT $1::BIGINT, $2::TEXT
        WHERE NOT EXISTS (
            SELECT 1 FROM user_addresses 
            WHERE user_id = $1 AND address = $2
        )
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

    // Фиксируем транзакцию
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

    // Достаем список админов для уведомлений
    let admin_records = sqlx::query!("SELECT telegram_id FROM admins WHERE is_active = true")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let admin_tg_ids: Vec<i64> = admin_records.into_iter().map(|a| a.telegram_id).collect();

    // Формируем список товаров для ответа из того, что вытащили из корзины ранее
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
