use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i32,
    #[allow(dead_code)] // Field read by SQLx mapping
    #[serde(skip_serializing)]
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
    pub category_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
