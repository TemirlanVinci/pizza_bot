use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use tracing::{error, info};
use crate::models::favorite::{
    AddFavoriteRequest, DeleteFavoriteQuery, FavoriteProduct, GetFavoritesQuery, StatusResponse,
};

/// GET /api/v1/favorites
pub async fn get_favorites(
    State(pool): State<PgPool>,
    Query(query): Query<GetFavoritesQuery>,
) -> Result<Json<Vec<FavoriteProduct>>, (StatusCode, String)> {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    let favorites = sqlx::query_as::<_, FavoriteProduct>(
        r#"
        SELECT p.id, p.name, p.price
        FROM favorites f
        JOIN products p ON f.product_id = p.id
        WHERE f.user_id = $1
        ORDER BY f.id DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(query.user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!(
            user_id = query.user_id,
            limit, offset,
            error = %e,
            "Не удалось получить список избранного"
        );
        // Клиенту — общий текст, детали только в логах сервера
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
    })?;

    Ok(Json(favorites))
}

/// POST /api/v1/favorites
pub async fn add_favorite(
    State(pool): State<PgPool>,
    Json(payload): Json<AddFavoriteRequest>,
) -> Result<Json<StatusResponse>, (StatusCode, String)> {
    sqlx::query(
        r#"
        INSERT INTO favorites (user_id, product_id)
        VALUES ($1, $2)
        ON CONFLICT (user_id, product_id) DO NOTHING
        "#,
    )
    .bind(payload.user_id)
    .bind(payload.product_id)
    .execute(&pool)
    .await
    .map_err(|e| {
        error!(
            user_id = payload.user_id,
            product_id = payload.product_id,
            error = %e,
            "Не удалось добавить товар в избранное"
        );
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
    })?;

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
) -> Result<Json<StatusResponse>, (StatusCode, String)> {
    sqlx::query(
        r#"
        DELETE FROM favorites
        WHERE user_id = $1 AND product_id = $2
        "#,
    )
    .bind(query.user_id)
    .bind(product_id)
    .execute(&pool)
    .await
    .map_err(|e| {
        error!(
            user_id = query.user_id,
            product_id = product_id,
            error = %e,
            "Не удалось удалить товар из избранного"
        );
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
    })?;

    info!(
        user_id = query.user_id,
        product_id = product_id,
        "Товар удалён из избранного"
    );

    Ok(Json(StatusResponse { status: "success" }))
}