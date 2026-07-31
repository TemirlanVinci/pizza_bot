use crate::db;
use crate::error::AppError;
use crate::models::favorites::{
    AddFavoriteRequest, DeleteFavoriteQuery, FavoriteProduct, GetFavoritesQuery, StatusResponse,
};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use sqlx::PgPool;
use tracing::info;
use validator::Validate;

/// GET /api/v1/favorites
pub async fn get_favorites(
    State(pool): State<PgPool>,
    Query(query): Query<GetFavoritesQuery>,
) -> Result<Json<Vec<FavoriteProduct>>, AppError> {
    query.validate()?;

    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    let favorites = db::favorites::get_favorites(&pool, query.user_id, limit, offset).await?;

    Ok(Json(favorites))
}

/// POST /api/v1/favorites
pub async fn add_favorite(
    State(pool): State<PgPool>,
    Json(payload): Json<AddFavoriteRequest>,
) -> Result<Json<StatusResponse>, AppError> {
    payload.validate()?;

    db::favorites::add_favorite(&pool, payload.user_id, payload.product_id).await?;

    info!(
        user_id = payload.user_id,
        product_id = payload.product_id,
        "Товар добавлен в избранное"
    );

    Ok(Json(StatusResponse { status: "success" }))
}

/// DELETE /api/v1/favorites/:product_id
pub async fn remove_favorite(
    State(pool): State<PgPool>,
    Path(product_id): Path<i32>,
    Query(query): Query<DeleteFavoriteQuery>,
) -> Result<Json<StatusResponse>, AppError> {
    query.validate()?;

    db::favorites::remove_favorite(&pool, query.user_id, product_id).await?;

    info!(
        user_id = query.user_id,
        product_id = product_id,
        "Товар удалён из избранного"
    );

    Ok(Json(StatusResponse { status: "success" }))
}
