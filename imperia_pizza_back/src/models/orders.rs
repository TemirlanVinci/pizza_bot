use serde::{Deserialize, Serialize};
use validator::Validate;

// --- Requests ---

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrderRequest {
    #[validate(range(min = 1))]
    pub user_id: i64,
    #[validate(length(min = 1))]
    pub user_name: String,
    #[validate(length(min = 1))]
    pub phone_number: String,
    #[validate(length(min = 1))]
    pub delivery_type: String, // "delivery" или "pickup"
    #[validate(length(min = 1))]
    pub address: String,
    #[validate(length(min = 1))]
    pub payment_method: String, // "cash" или "visa_courier"
}

// --- Responses ---

#[derive(Debug, Serialize)]
pub struct CreateOrderResponse {
    pub status: String,
    pub order_id: i32,
    pub total_price: i32,

    // Добавленные поля для пайтон-бота, чтобы он сразу раскидал уведомления
    pub delivery_type: String,
    pub address: String,
    pub user_name: String,
    pub phone_number: String,
    pub payment_method: String,
    pub created_at: String,
    pub admin_tg_ids: Vec<i64>,
    pub items: Vec<OrderItemResponse>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserOrderResponse {
    pub order_id: i32,
    pub status: String,
    pub total_price: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct OrderItemResponse {
    pub product_id: Option<i32>,
    pub name: String,
    pub quantity: i32,
    pub price_at_purchase: i32,
}

#[derive(Debug, Serialize)]
pub struct OrderDetailResponse {
    pub order_id: i32,
    pub status: String,
    pub delivery_type: String,
    pub address: String,
    pub user_name: String,
    pub phone_number: String,
    pub total_price: i32,
    pub created_at: String,
    pub items: Vec<OrderItemResponse>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    fn valid_order_request() -> CreateOrderRequest {
        CreateOrderRequest {
            user_id: 12345,
            user_name: "John Doe".to_string(),
            phone_number: "+77001234567".to_string(),
            delivery_type: "delivery".to_string(),
            address: "Main St. 10".to_string(),
            payment_method: "cash".to_string(),
        }
    }

    #[test]
    fn test_create_order_request_valid() {
        let req = valid_order_request();
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_create_order_request_invalid_user_id() {
        let mut req = valid_order_request();
        req.user_id = 0;
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_create_order_request_empty_fields() {
        let mut req = valid_order_request();
        req.user_name = "".to_string();
        assert!(req.validate().is_err());

        let mut req = valid_order_request();
        req.phone_number = "".to_string();
        assert!(req.validate().is_err());

        let mut req = valid_order_request();
        req.address = "".to_string();
        assert!(req.validate().is_err());
    }
}
