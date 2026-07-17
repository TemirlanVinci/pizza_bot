use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use sqlx::PgPool;
use tracing::{error, info, instrument};

use crate::models::branch::{Branch, BranchListItem, BranchListParams};

const DEFAULT_LIMIT: i64 = 20;
const MAX_LIMIT: i64 = 100;

/// GET /api/v1/branch?limit=&offset=
#[instrument(skip(pool))]
pub async fn list_branches(
    State(pool): State<PgPool>,
    Query(params): Query<BranchListParams>,
) -> Result<Json<Vec<BranchListItem>>, (StatusCode, String)> {
    // защищаемся от неадекватных значений в query
    let limit = params.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let offset = params.offset.unwrap_or(0).max(0);

    info!(limit, offset, "запрос списка филиалов");

    let branches = sqlx::query_as::<_, BranchListItem>(
        r#"
        SELECT id, name, is_active
        FROM branches
        WHERE is_active = true
        ORDER BY id
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await
    .map_err(|err| {
        error!(%err, "не удалось получить список филиалов из БД");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "внутренняя ошибка сервера".to_string(),
        )
    })?;

    info!(count = branches.len(), "список филиалов успешно получен");

    Ok(Json(branches))
}

/// GET /api/v1/branch/:id
#[instrument(skip(pool))]
pub async fn get_branch(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Branch>, (StatusCode, String)> {
    info!(branch_id = id, "запрос филиала по id");

    let branch = sqlx::query_as::<_, Branch>(
        r#"
        SELECT id, name, address, work_hours, map_link, phone, is_active
        FROM branches
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        error!(%err, branch_id = id, "ошибка запроса к БД при получении филиала");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "внутренняя ошибка сервера".to_string(),
        )
    })?;

    match branch {
        Some(branch) => {
            info!(branch_id = id, "филиал найден");
            Ok(Json(branch))
        }
        None => {
            info!(branch_id = id, "филиал не найден");
            Err((StatusCode::NOT_FOUND, "филиал не найден".to_string()))
        }
    }
}
