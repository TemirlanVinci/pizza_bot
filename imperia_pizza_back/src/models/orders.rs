use serde::{Deserialize, Serialize};

// --- Requests ---

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub user_id: i64,
    pub user_name: String,
    pub phone_number: String,
    pub delivery_type: String, // "delivery" или "pickup"
    pub address: String,
    pub payment_method: String, // "cash" или "visa_courier"
}

#[derive(Debug, Deserialize)]
pub struct NewOrderNotificationRequest {
    pub order_id: i32,
}

// --- Responses ---

#[derive(Debug, Serialize)]
pub struct CreateOrderResponse {
    pub status: String,
    pub order_id: i32,
    pub total_price: i32,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserOrderResponse {
    pub order_id: i32,
    pub status: String,
    pub total_price: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
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

#[derive(Debug, Serialize)]
pub struct NewOrderNotificationResponse {
    pub order_id: i32,
    pub delivery_type: String,
    pub address: String,
    pub user_name: String,
    pub phone_number: String,
    pub payment_method: String,
    pub total_price: i32,
    pub created_at: String,
    pub admin_tg_ids: Vec<i64>,
    pub items: Vec<OrderItemResponse>,
}
