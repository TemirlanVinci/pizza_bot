use crate::models::product::{Product, ProductFilter};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use sqlx::{PgPool, Postgres, QueryBuilder};
use tracing::error;

pub async fn get_products(
    State(pool): State<PgPool>,
    Query(filter): Query<ProductFilter>,
) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    // Используем QueryBuilder для безопасного динамического SQL
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT id, name, price, category_id FROM products WHERE 1=1");

    if let Some(category_id) = filter.category_id {
        query_builder.push(" AND category_id = ");
        query_builder.push_bind(category_id);
    }

    if let Some(limit) = filter.limit {
        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit);
    }

    if let Some(offset) = filter.offset {
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);
    }

    let products = query_builder
        .build_query_as::<Product>()
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            error!(
                category_id = ?filter.category_id,
                limit = ?filter.limit,
                offset = ?filter.offset,
                error = %e,
                "Не удалось получить список товаров"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            )
        })?;

    Ok(Json(products))
}
