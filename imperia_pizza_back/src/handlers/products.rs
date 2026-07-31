use crate::db;
use crate::error::AppError;
use crate::models::products::{Product, ProductFilter, ProductFull};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use sqlx::PgPool;
use tracing::warn;
use validator::Validate;

pub async fn get_products(
    State(pool): State<PgPool>,
    Query(filter): Query<ProductFilter>,
) -> Result<Json<Vec<Product>>, AppError> {
    filter.validate()?;

    let products = db::products::get_products(&pool, &filter).await?;
    Ok(Json(products))
}

pub async fn get_product_by_id(
    State(pool): State<PgPool>,
    Path(product_id): Path<i32>,
) -> Result<Json<ProductFull>, AppError> {
    let product = db::products::get_product_by_id(&pool, product_id)
        .await?
        .ok_or_else(|| {
            warn!(product_id, "Запрошен несуществующий товар");
            AppError::NotFound
        })?;

    Ok(Json(product))
}
