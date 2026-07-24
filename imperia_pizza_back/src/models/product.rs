use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i32,
    #[allow(dead_code)] // Field read by SQLx mapping
    #[serde(skip_serializing)]
    pub category_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct ProductFilter {
    pub category_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
