use crate::models::branches::{Branch, BranchListItem};
use sqlx::PgPool;

pub async fn list_branches(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<BranchListItem>, sqlx::Error> {
    sqlx::query_as::<_, BranchListItem>(
        r#"
        SELECT id, name, is_active
        FROM branches
        ORDER BY id
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn get_branch_by_id(pool: &PgPool, id: i32) -> Result<Option<Branch>, sqlx::Error> {
    sqlx::query_as::<_, Branch>(
        r#"
        SELECT id, name, address, work_hours, map_link, phone, is_active
        FROM branches
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
