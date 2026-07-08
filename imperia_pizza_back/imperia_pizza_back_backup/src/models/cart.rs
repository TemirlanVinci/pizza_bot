use serde::{Deserialize, Serialize};

// --- Входящие запросы ---

#[derive(Deserialize)]
pub struct CartQuery {
    pub user_id: i64,
}

#[derive(Deserialize)]
pub struct CartActionRequest {
    pub user_id: i64,
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