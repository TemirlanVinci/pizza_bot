use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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

#[derive(Debug, Deserialize)]
pub struct ProductFilter {
    pub category_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}