use axum::Router;
use sqlx::PgPool;

mod admin;
mod branch;
mod cart;
mod catalog;
mod favorites;
mod orders;
mod products;
mod users;

/// Собирает роутеры всех доменов под общим префиксом /api/v1.
/// Чтобы добавить новый домен: создать routes/<domain>.rs с fn router() -> Router<PgPool>,
/// объявить `mod <domain>;` выше и добавить `.merge(<domain>::router())` ниже.
pub fn build_router() -> Router<PgPool> {
    Router::new().nest(
        "/api/v1",
        Router::new()
            .merge(users::router())
            .merge(catalog::router())
            .merge(products::router())
            .merge(favorites::router())
            .merge(cart::router())
            .merge(branch::router())
            .merge(orders::router())
            .merge(admin::router()),
    )
}
