use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Краткая карточка филиала — под GET /api/v1/branch (список)
#[derive(Debug, Serialize, FromRow)]
pub struct BranchListItem {
    pub id: i32,
    pub name: String,
    pub is_active: bool,
}

/// Полная карточка филиала — под GET /api/v1/branch/:id
#[derive(Debug, Serialize, FromRow)]
pub struct Branch {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub work_hours: String,
    pub map_link: Option<String>,
    pub phone: Option<String>,
    pub is_active: bool,
}

/// Query-параметры пагинации для списка филиалов
#[derive(Debug, Deserialize)]
pub struct BranchListParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
