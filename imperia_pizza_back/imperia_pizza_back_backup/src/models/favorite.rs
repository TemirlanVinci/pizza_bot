use serde::{Deserialize, Serialize};

// --- Responses ---

#[derive(Serialize, sqlx::FromRow)]
pub struct FavoriteProduct {
    pub id: i32,
    pub name: String,
    pub price: i32,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: &'static str,
}

// --- Requests ---

#[derive(Deserialize)]
pub struct GetFavoritesQuery {
    pub user_id: i64,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct AddFavoriteRequest {
    pub user_id: i64,
    pub product_id: i32,
}

#[derive(Deserialize)]
pub struct DeleteFavoriteQuery {
    pub user_id: i64,
}