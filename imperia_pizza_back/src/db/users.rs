use sqlx::{PgPool, postgres::PgQueryResult};

pub async fn register_user(pool: &PgPool, telegram_id: i64) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (telegram_id)
        VALUES ($1)
        ON CONFLICT (telegram_id) DO NOTHING
        "#,
        telegram_id
    )
    .execute(pool)
    .await
}
