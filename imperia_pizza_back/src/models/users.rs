use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserRequest {
    #[validate(range(min = 1))]
    pub telegram_id: i64,
}

#[derive(Serialize)]
pub struct UserStatusResponse {
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_register_user_request_valid() {
        let req = RegisterUserRequest {
            telegram_id: 123456789,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_register_user_request_invalid_telegram_id() {
        let req_zero = RegisterUserRequest { telegram_id: 0 };
        assert!(req_zero.validate().is_err());

        let req_negative = RegisterUserRequest { telegram_id: -100 };
        assert!(req_negative.validate().is_err());
    }
}
