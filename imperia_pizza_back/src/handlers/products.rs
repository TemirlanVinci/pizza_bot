use crate::models::products::{Product, ProductFilter, ProductFull};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use sqlx::{PgPool, Postgres, QueryBuilder};
use tracing::{error, warn};

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

pub async fn get_product_by_id(
    State(pool): State<PgPool>,
    Path(product_id): Path<i32>,
) -> Result<Json<ProductFull>, (StatusCode, String)> {
    let product = sqlx::query_as::<_, ProductFull>(
        r#"
        SELECT 
            id, 
            category_id, 
            name, 
            description, 
            price, 
            weight, 
            image_url 
        FROM products 
        WHERE id = $1
        "#,
    )
    .bind(product_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => {
            warn!(product_id, "Запрошен несуществующий товар");
            (
                StatusCode::NOT_FOUND,
                format!("Product with id {} not found", product_id),
            )
        }
        _ => {
            error!(product_id, error = %e, "Ошибка БД при получении товара");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            )
        }
    })?;

    Ok(Json(product))
}
