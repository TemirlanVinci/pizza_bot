use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize)]
pub struct Catalog {
    pub categories: Vec<Category>,
}