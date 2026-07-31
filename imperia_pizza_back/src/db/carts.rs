use sqlx::{PgPool, postgres::PgQueryResult};

pub struct CartItemRow {
    pub product_id: i32,
    pub name: String,
    pub price: i32,
    pub quantity: i32,
}

pub async fn get_cart(pool: &PgPool, user_id: i64) -> Result<Vec<CartItemRow>, sqlx::Error> {
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
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(records
        .into_iter()
        .map(|r| CartItemRow {
            product_id: r.product_id,
            name: r.name,
            price: r.price,
            quantity: r.quantity,
        })
        .collect())
}

pub async fn add_to_cart(pool: &PgPool, user_id: i64, product_id: i32) -> Result<i32, sqlx::Error> {
    let record = sqlx::query!(
        r#"
        INSERT INTO cart_items (user_id, product_id, quantity)
        VALUES ($1, $2, 1)
        ON CONFLICT (user_id, product_id)
        DO UPDATE SET quantity = cart_items.quantity + 1
        RETURNING quantity
        "#,
        user_id,
        product_id
    )
    .fetch_one(pool)
    .await?;

    Ok(record.quantity)
}

pub async fn decrement_cart(
    pool: &PgPool,
    user_id: i64,
    product_id: i32,
) -> Result<Option<i32>, sqlx::Error> {
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
        user_id,
        product_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(record.map(|r| r.quantity))
}

pub async fn remove_from_cart(
    pool: &PgPool,
    user_id: i64,
    product_id: i32,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM cart_items
        WHERE user_id = $1 AND product_id = $2
        "#,
        user_id,
        product_id
    )
    .execute(pool)
    .await
}
