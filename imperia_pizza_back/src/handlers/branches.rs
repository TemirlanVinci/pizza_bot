use axum::{
    Json,
    extract::{Path, Query, State},
};
use sqlx::PgPool;
use tracing::{info, instrument};

use crate::db;
use crate::error::AppError;
use crate::models::branches::{Branch, BranchListItem, BranchListParams};

const DEFAULT_LIMIT: i64 = 20;
const MAX_LIMIT: i64 = 100;

/// GET /api/v1/branch?limit=&offset=
#[instrument(skip(pool))]
pub async fn list_branches(
    State(pool): State<PgPool>,
    Query(params): Query<BranchListParams>,
) -> Result<Json<Vec<BranchListItem>>, AppError> {
    let limit = params.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let offset = params.offset.unwrap_or(0).max(0);

    info!(limit, offset, "запрос списка филиалов");

    let branches = db::branches::list_branches(&pool, limit, offset).await?;

    info!(count = branches.len(), "список филиалов успешно получен");

    Ok(Json(branches))
}

/// GET /api/v1/branch/:id
#[instrument(skip(pool))]
pub async fn get_branch(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Branch>, AppError> {
    info!(branch_id = id, "запрос филиала по id");

    let branch = db::branches::get_branch_by_id(&pool, id).await?;

    match branch {
        Some(branch) => {
            info!(branch_id = id, "филиал найден");
            Ok(Json(branch))
        }
        None => {
            info!(branch_id = id, "филиал не найден");
            Err(AppError::NotFound)
        }
    }
}
