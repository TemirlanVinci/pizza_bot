use serde::{Deserialize, Serialize};

// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub admin_tg_id: i64,
    pub status: String, // confirmed, cooking, delivering, completed, cancelled
}

#[derive(Debug, Deserialize)]
pub struct BanUserRequest {
    pub admin_tg_id: i64,
    pub user_id: i64,
    pub phone_number: Option<String>,
    pub ban_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminCheckRequest {
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

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ActiveOrderResponse {
    pub order_id: i32,
    pub status: String,
    pub delivery_type: String,
    pub address: String,
    pub phone_number: String,
    pub total_price: i32,
    pub created_at: String,
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
