use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub telegram_id: i64,
}

#[derive(Serialize)]
pub struct UserStatusResponse {
    pub status: String,
}
