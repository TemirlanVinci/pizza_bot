use crate::models::catalogs::Category;
use sqlx::PgPool;

pub async fn get_categories(pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
    sqlx::query_as::<_, Category>("SELECT id, name FROM categories ORDER BY id")
        .fetch_all(pool)
        .await
}
