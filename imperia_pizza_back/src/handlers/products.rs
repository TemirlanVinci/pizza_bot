use crate::db;
use crate::error::AppError;
use crate::models::products::{Product, ProductFilter, ProductFull};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use sqlx::PgPool;
use tracing::warn;

pub async fn get_products(
    State(pool): State<PgPool>,
    Query(filter): Query<ProductFilter>,
) -> Result<Json<Vec<Product>>, AppError> {
    let products = db::products::get_products(&pool, &filter).await?;
    Ok(Json(products))
}

pub async fn get_product_by_id(
    State(pool): State<PgPool>,
    Path(product_id): Path<i32>,
) -> Result<Json<ProductFull>, AppError> {
    let product = db::products::get_product_by_id(&pool, product_id).await?;

    match product {
        Some(product) => Ok(Json(product)),
        None => {
            warn!(product_id, "Запрошен несуществующий товар");
            Err(AppError::NotFound)
        }
    }
}
