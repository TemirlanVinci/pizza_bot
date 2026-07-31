use sqlx::PgPool;

pub struct OrderCartItem {
    pub product_id: Option<i32>,
    pub name: String,
    pub price: i32,
    pub quantity: i32,
}

pub struct CreatedOrderHeader {
    pub id: i32,
    pub created_at: String,
}

pub struct CreateOrderHeaderParams<'a> {
    pub user_id: i64,
    pub user_name: &'a str,
    pub phone_number: &'a str,
    pub delivery_type: &'a str,
    pub address: &'a str,
    pub payment_method: &'a str,
    pub total_price: i32,
}

pub struct UserOrderRecord {
    pub order_id: i32,
    pub status: String,
    pub total_price: i32,
    pub created_at: String,
}

pub struct OrderDetailHeader {
    pub order_id: i32,
    pub status: String,
    pub delivery_type: String,
    pub address: String,
    pub user_name: String,
    pub phone_number: String,
    pub total_price: i32,
    pub created_at: String,
}

pub struct OrderItemRecord {
    pub product_id: Option<i32>,
    pub name: String,
    pub quantity: i32,
    pub price_at_purchase: i32,
}

pub async fn get_cart_items_for_order(
    pool: &PgPool,
    user_id: i64,
) -> Result<Vec<OrderCartItem>, sqlx::Error> {
    let records = sqlx::query!(
        r#"
        SELECT c.product_id, p.name, p.price, c.quantity
        FROM cart_items c
        JOIN products p ON c.product_id = p.id
        WHERE c.user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(records
        .into_iter()
        .map(|r| OrderCartItem {
            product_id: r.product_id,
            name: r.name,
            price: r.price,
            quantity: r.quantity,
        })
        .collect())
}

pub async fn ensure_user_exists_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (telegram_id)
        VALUES ($1)
        ON CONFLICT (telegram_id) DO NOTHING
        "#,
        user_id
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn create_order_header_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    params: CreateOrderHeaderParams<'_>,
) -> Result<CreatedOrderHeader, sqlx::Error> {
    let record = sqlx::query!(
        r#"
        INSERT INTO orders (user_id, user_name, phone_number, delivery_type, address, payment_method, status, total_price)
        VALUES ($1, $2, $3, $4, $5, $6, 'confirmed', $7)
        RETURNING id, COALESCE(to_char(created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"'), '') AS created_at
        "#,
        params.user_id,
        params.user_name,
        params.phone_number,
        params.delivery_type,
        params.address,
        params.payment_method,
        params.total_price
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(CreatedOrderHeader {
        id: record.id,
        created_at: record.created_at.unwrap_or_default(),
    })
}

pub async fn insert_order_item_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    order_id: i32,
    product_id: Option<i32>,
    name: &str,
    quantity: i32,
    price_at_purchase: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO order_items (order_id, product_id, name, quantity, price_at_purchase)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        order_id,
        product_id,
        name,
        quantity,
        price_at_purchase
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn save_user_phone_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    phone_number: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO user_phones (user_id, phone_number)
        SELECT $1::BIGINT, $2::VARCHAR
        WHERE NOT EXISTS (
            SELECT 1 FROM user_phones 
            WHERE user_id = $1 AND phone_number = $2
        )
        "#,
        user_id,
        phone_number
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn save_user_address_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    address: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO user_addresses (user_id, address)
        SELECT $1::BIGINT, $2::TEXT
        WHERE NOT EXISTS (
            SELECT 1 FROM user_addresses 
            WHERE user_id = $1 AND address = $2
        )
        "#,
        user_id,
        address
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn clear_cart_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM cart_items
        WHERE user_id = $1
        "#,
        user_id
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn get_active_admin_ids(pool: &PgPool) -> Result<Vec<i64>, sqlx::Error> {
    let records = sqlx::query!("SELECT telegram_id FROM admins WHERE is_active = true")
        .fetch_all(pool)
        .await?;

    Ok(records.into_iter().map(|a| a.telegram_id).collect())
}

pub async fn get_user_orders(
    pool: &PgPool,
    user_id: i64,
) -> Result<Vec<UserOrderRecord>, sqlx::Error> {
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
    .fetch_all(pool)
    .await?;

    Ok(records
        .into_iter()
        .map(|r| UserOrderRecord {
            order_id: r.order_id,
            status: r.status,
            total_price: r.total_price,
            created_at: r.created_at.unwrap_or_default(),
        })
        .collect())
}

pub async fn get_order_detail_header(
    pool: &PgPool,
    order_id: i32,
) -> Result<Option<OrderDetailHeader>, sqlx::Error> {
    let record = sqlx::query!(
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
    .fetch_optional(pool)
    .await?;

    Ok(record.map(|r| OrderDetailHeader {
        order_id: r.order_id,
        status: r.status,
        delivery_type: r.delivery_type,
        address: r.address,
        user_name: r.user_name,
        phone_number: r.phone_number,
        total_price: r.total_price,
        created_at: r.created_at.unwrap_or_default(),
    }))
}

pub async fn get_order_items(
    pool: &PgPool,
    order_id: i32,
) -> Result<Vec<OrderItemRecord>, sqlx::Error> {
    let records = sqlx::query!(
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
    .fetch_all(pool)
    .await?;

    Ok(records
        .into_iter()
        .map(|item| OrderItemRecord {
            product_id: item.product_id,
            name: item.name,
            quantity: item.quantity,
            price_at_purchase: item.price_at_purchase,
        })
        .collect())
}
