use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

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
#[derive(Debug, Deserialize, Validate)]
pub struct BranchListParams {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<i64>,
    #[validate(range(min = 0))]
    pub offset: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_branch_list_params_valid() {
        let params = BranchListParams {
            limit: Some(20),
            offset: Some(0),
        };
        assert!(params.validate().is_ok());

        let none_params = BranchListParams {
            limit: None,
            offset: None,
        };
        assert!(none_params.validate().is_ok());
    }

    #[test]
    fn test_branch_list_params_invalid() {
        let invalid_limit = BranchListParams {
            limit: Some(0),
            offset: Some(0),
        };
        assert!(invalid_limit.validate().is_err());

        let invalid_offset = BranchListParams {
            limit: Some(10),
            offset: Some(-1),
        };
        assert!(invalid_offset.validate().is_err());
    }
}
