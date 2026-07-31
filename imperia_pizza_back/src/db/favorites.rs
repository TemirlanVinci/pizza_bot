use crate::models::favorites::FavoriteProduct;
use sqlx::{PgPool, postgres::PgQueryResult};

pub async fn get_favorites(
    pool: &PgPool,
    user_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<FavoriteProduct>, sqlx::Error> {
    sqlx::query_as::<_, FavoriteProduct>(
        r#"
        SELECT p.id, p.name, p.price
        FROM favorites f
        JOIN products p ON f.product_id = p.id
        WHERE f.user_id = $1
        ORDER BY f.id DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn add_favorite(
    pool: &PgPool,
    user_id: i64,
    product_id: i32,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO favorites (user_id, product_id)
        VALUES ($1, $2)
        ON CONFLICT (user_id, product_id) DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(product_id)
    .execute(pool)
    .await
}

pub async fn remove_favorite(
    pool: &PgPool,
    user_id: i64,
    product_id: i32,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM favorites
        WHERE user_id = $1 AND product_id = $2
        "#,
    )
    .bind(user_id)
    .bind(product_id)
    .execute(pool)
    .await
}
