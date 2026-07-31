use serde::{Deserialize, Serialize};
use validator::Validate;

// --- Responses ---

#[derive(Serialize, sqlx::FromRow)]
pub struct FavoriteProduct {
    pub id: i32,
    pub name: String,
    pub price: i32,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: &'static str,
}

// --- Requests ---

#[derive(Debug, Deserialize, Validate)]
pub struct GetFavoritesQuery {
    #[validate(range(min = 1))]
    pub user_id: i64,
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<i64>,
    #[validate(range(min = 0))]
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddFavoriteRequest {
    #[validate(range(min = 1))]
    pub user_id: i64,
    #[validate(range(min = 1))]
    pub product_id: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct DeleteFavoriteQuery {
    #[validate(range(min = 1))]
    pub user_id: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_get_favorites_query_validation() {
        let valid = GetFavoritesQuery {
            user_id: 10,
            limit: Some(10),
            offset: Some(0),
        };
        assert!(valid.validate().is_ok());

        let invalid_user = GetFavoritesQuery {
            user_id: 0,
            limit: Some(10),
            offset: Some(0),
        };
        assert!(invalid_user.validate().is_err());
    }

    #[test]
    fn test_add_favorite_request_validation() {
        let valid = AddFavoriteRequest {
            user_id: 10,
            product_id: 2,
        };
        assert!(valid.validate().is_ok());

        let invalid_product = AddFavoriteRequest {
            user_id: 10,
            product_id: 0,
        };
        assert!(invalid_product.validate().is_err());
    }

    #[test]
    fn test_delete_favorite_query_validation() {
        let valid = DeleteFavoriteQuery { user_id: 10 };
        assert!(valid.validate().is_ok());

        let invalid = DeleteFavoriteQuery { user_id: -5 };
        assert!(invalid.validate().is_err());
    }
}
