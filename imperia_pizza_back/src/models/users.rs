use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterUserRequest {
    pub telegram_id: i64,
}

#[derive(Serialize)]
pub struct UserStatusResponse {
    pub status: String,
}
