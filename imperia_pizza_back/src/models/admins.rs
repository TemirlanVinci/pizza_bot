use serde::{Deserialize, Serialize};
use validator::Validate;

// --- Request DTOs ---

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrderStatusRequest {
    #[validate(range(min = 1))]
    pub admin_tg_id: i64,
    #[validate(length(min = 1))]
    pub status: String, // confirmed, cooking, delivering, completed, cancelled
}

#[derive(Debug, Deserialize, Validate)]
pub struct BanUserRequest {
    #[validate(range(min = 1))]
    pub admin_tg_id: i64,
    #[validate(range(min = 1))]
    pub user_id: i64,
    pub phone_number: Option<String>,
    pub ban_reason: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AdminCheckRequest {
    #[validate(range(min = 1))]
    pub admin_tg_id: Option<i64>,
}

// --- Response DTOs ---

#[derive(Debug, Serialize)]
pub struct UpdateOrderStatusResponse {
    pub status: String,
    pub order_id: i32,
    pub new_status: String,
}

#[derive(Debug, Serialize)]
pub struct StatusSuccessResponse {
    pub status: String,
}

/// Позиция заказа. Декодируется из JSON-агрегата (json_agg/json_build_object),
/// который собирает Postgres, поэтому нужен Deserialize, а не FromRow.
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemResponse {
    pub product_id: Option<i32>, // order_items.product_id не NOT NULL в схеме
    pub name: String,
    pub quantity: i32,
    pub price_at_purchase: i32,
}

#[derive(Debug, Serialize)]
pub struct ActiveOrderResponse {
    pub order_id: i32,
    pub status: String,
    pub delivery_type: String,
    pub address: String,
    pub phone_number: String,
    pub user_id: i64,
    pub user_name: String,
    pub total_price: i32,
    pub created_at: String,
    pub items: Vec<OrderItemResponse>,
}

#[derive(Debug, Serialize)]
pub struct BroadcastUsersResponse {
    pub user_ids: Vec<i64>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AdminItem {
    pub telegram_id: i64,
    pub name: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct AdminListResponse {
    pub admins: Vec<AdminItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_update_order_status_request_validation() {
        let valid = UpdateOrderStatusRequest {
            admin_tg_id: 123,
            status: "cooking".to_string(),
        };
        assert!(valid.validate().is_ok());

        let invalid_admin = UpdateOrderStatusRequest {
            admin_tg_id: 0,
            status: "cooking".to_string(),
        };
        assert!(invalid_admin.validate().is_err());

        let empty_status = UpdateOrderStatusRequest {
            admin_tg_id: 123,
            status: "".to_string(),
        };
        assert!(empty_status.validate().is_err());
    }

    #[test]
    fn test_ban_user_request_validation() {
        let valid = BanUserRequest {
            admin_tg_id: 123,
            user_id: 456,
            phone_number: None,
            ban_reason: Some("Spam".to_string()),
        };
        assert!(valid.validate().is_ok());

        let invalid_user = BanUserRequest {
            admin_tg_id: 123,
            user_id: 0,
            phone_number: None,
            ban_reason: None,
        };
        assert!(invalid_user.validate().is_err());
    }

    #[test]
    fn test_admin_check_request_validation() {
        let valid_some = AdminCheckRequest {
            admin_tg_id: Some(100),
        };
        assert!(valid_some.validate().is_ok());

        let valid_none = AdminCheckRequest { admin_tg_id: None };
        assert!(valid_none.validate().is_ok());

        let invalid = AdminCheckRequest {
            admin_tg_id: Some(0),
        };
        assert!(invalid.validate().is_err());
    }
}
