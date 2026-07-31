use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, FromRow, Deserialize, Serialize, Validate)]
pub struct Product {
    pub id: i32,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(range(min = 0))]
    pub price: i32,
    #[allow(dead_code)] // Field read by SQLx mapping
    #[serde(default, skip_serializing)]
    pub category_id: i32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct ProductFull {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
    pub weight: i32,
    pub image_url: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ProductFilter {
    #[validate(range(min = 1))]
    pub category_id: Option<i32>,
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<i64>,
    #[validate(range(min = 0))]
    pub offset: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_product_validation() {
        let valid_product = Product {
            id: 1,
            name: "Pepperoni".to_string(),
            price: 500,
            category_id: 1,
        };
        assert!(valid_product.validate().is_ok());

        let invalid_name_product = Product {
            id: 1,
            name: "".to_string(),
            price: 500,
            category_id: 1,
        };
        assert!(invalid_name_product.validate().is_err());

        let invalid_price_product = Product {
            id: 1,
            name: "Pepperoni".to_string(),
            price: -10,
            category_id: 1,
        };
        assert!(invalid_price_product.validate().is_err());
    }

    #[test]
    fn test_product_validation_success() {
        let product = Product {
            id: 1,
            name: "Pepperoni".to_string(),
            price: 500,
            category_id: 1,
        };
        assert!(product.validate().is_ok());
    }

    #[test]
    fn test_product_validation_empty_name() {
        let product = Product {
            id: 1,
            name: "".to_string(),
            price: 500,
            category_id: 1,
        };
        assert!(product.validate().is_err());
    }

    #[test]
    fn test_product_validation_negative_price() {
        let product = Product {
            id: 1,
            name: "Pepperoni".to_string(),
            price: -10,
            category_id: 1,
        };
        assert!(product.validate().is_err());
    }

    #[test]
    fn test_product_filter_validation() {
        let valid_filter = ProductFilter {
            category_id: Some(2),
            limit: Some(10),
            offset: Some(0),
        };
        assert!(valid_filter.validate().is_ok());

        let invalid_limit = ProductFilter {
            category_id: Some(2),
            limit: Some(0),
            offset: Some(0),
        };
        assert!(invalid_limit.validate().is_err());

        let invalid_offset = ProductFilter {
            category_id: Some(2),
            limit: Some(10),
            offset: Some(-5),
        };
        assert!(invalid_offset.validate().is_err());
    }
}
