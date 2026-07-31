use crate::db;
use crate::error::AppError;
use crate::models::catalogs::Catalog;
use axum::{Json, extract::State};
use sqlx::PgPool;

pub async fn get_catalog(State(pool): State<PgPool>) -> Result<Json<Catalog>, AppError> {
    let categories = db::catalogs::get_categories(&pool).await?;
    Ok(Json(Catalog { categories }))
}
