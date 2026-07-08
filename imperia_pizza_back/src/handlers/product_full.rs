use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use tracing::{error, warn};
use crate::models::product_full::ProductFull;

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
        "#
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
            (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
        }
    })?;

    Ok(Json(product))
}