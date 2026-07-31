use serde::{Deserialize, Serialize};
use validator::Validate;

// --- Входящие запросы ---

#[derive(Debug, Deserialize, Validate)]
pub struct CartQuery {
    #[validate(range(min = 1))]
    pub user_id: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CartActionRequest {
    #[validate(range(min = 1))]
    pub user_id: i64,
    #[validate(range(min = 1))]
    pub product_id: i32,
}

// --- Исходящие ответы ---

#[derive(Serialize)]
pub struct CartItemResponse {
    pub product_id: i32,
    pub name: String,
    pub price: i32,
    pub quantity: i32,
    pub total_item_price: i32,
}

#[derive(Serialize)]
pub struct CartResponse {
    pub items: Vec<CartItemResponse>,
    pub total_quantity: i32,
    pub final_price: i32,
}

#[derive(Serialize)]
pub struct CartActionResponse {
    pub status: String,
    pub product_id: i32,
    pub current_quantity: i32,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_cart_query_validation() {
        let valid = CartQuery { user_id: 100 };
        assert!(valid.validate().is_ok());

        let invalid = CartQuery { user_id: 0 };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_cart_action_request_validation() {
        let valid = CartActionRequest {
            user_id: 100,
            product_id: 5,
        };
        assert!(valid.validate().is_ok());

        let invalid_user = CartActionRequest {
            user_id: -1,
            product_id: 5,
        };
        assert!(invalid_user.validate().is_err());

        let invalid_product = CartActionRequest {
            user_id: 100,
            product_id: 0,
        };
        assert!(invalid_product.validate().is_err());
    }
}
