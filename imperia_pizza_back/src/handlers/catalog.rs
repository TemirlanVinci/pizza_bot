use crate::models::catalog::{Catalog, Category};
use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;
use tracing::error;

pub async fn get_catalog(
    State(pool): State<PgPool>,
) -> Result<Json<Catalog>, (StatusCode, String)> {
    let categories = sqlx::query_as::<_, Category>("SELECT id, name FROM categories ORDER BY id")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Не удалось получить список категорий");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            )
        })?;

    Ok(Json(Catalog { categories }))
}
