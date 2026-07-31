use axum_test::TestServer;
use imperia_pizza_back::{
    create_app,
    models::products::{Product, ProductFull},
};
use sqlx::PgPool;
use std::env;

const BOT_SECRET_ENV: &str = "BOT_SECRET";
const TEST_SECRET: &str = "test_bot_secret";

/// Helper to ensure BOT_SECRET env variable is present during test execution.
fn ensure_bot_secret_env() -> String {
    match env::var(BOT_SECRET_ENV) {
        Ok(secret) => secret,
        Err(_) => {
            unsafe {
                env::set_var(BOT_SECRET_ENV, TEST_SECRET);
            }
            TEST_SECRET.to_string()
        }
    }
}

/// Helper to create a test server wrapping the Axum router injected with test PgPool.
fn setup_test_server(pool: PgPool) -> TestServer {
    ensure_bot_secret_env();
    let app = create_app(pool);
    TestServer::new(app).expect("Failed to create Axum TestServer")
}

#[sqlx::test]
async fn test_unauthorized_access(pool: PgPool) {
    ensure_bot_secret_env();
    let server = setup_test_server(pool);

    let response = server.get("/api/v1/products").await;

    response.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_get_products_empty(pool: PgPool) {
    let secret = ensure_bot_secret_env();
    let server = setup_test_server(pool);

    let response = server
        .get("/api/v1/products")
        .add_header("X-Bot-Secret", secret)
        .await;

    response.assert_status_ok();
    let _products: Vec<Product> = response.json();
}

#[sqlx::test]
async fn test_create_and_get_product(pool: PgPool) {
    let secret = ensure_bot_secret_env();
    let server = setup_test_server(pool.clone());

    // Insert parent category first to satisfy foreign key constraint
    sqlx::query!(
        "INSERT INTO categories (id, name) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name",
        99999,
        "Pizzas"
    )
    .execute(&pool)
    .await
    .expect("Failed to insert mock category");

    // Insert mock product directly using sqlx::query!
    sqlx::query!(
        "INSERT INTO products (id, category_id, name, description, price, weight, image_url) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, price = EXCLUDED.price",
        99999,
        99999,
        "Margarita",
        "Classic Margherita Pizza",
        450,
        400,
        "http://example.com/margarita.jpg"
    )
    .execute(&pool)
    .await
    .expect("Failed to insert mock product");

    // GET /api/v1/products with auth header
    let response = server
        .get("/api/v1/products")
        .add_header("X-Bot-Secret", secret.clone())
        .await;

    response.assert_status_ok();
    let products: Vec<Product> = response.json();
    let created_product = products.iter().find(|p| p.id == 99999).expect("Created product with ID 99999 not found");
    assert_eq!(created_product.name, "Margarita");
    assert_eq!(created_product.price, 450);

    // GET /api/v1/products/99999 with auth header
    let single_response = server
        .get("/api/v1/products/99999")
        .add_header("X-Bot-Secret", secret)
        .await;

    single_response.assert_status_ok();
    let product_full: ProductFull = single_response.json();
    assert_eq!(product_full.id, 99999);
    assert_eq!(product_full.category_id, 99999);
    assert_eq!(product_full.name, "Margarita");
    assert_eq!(
        product_full.description.as_deref(),
        Some("Classic Margherita Pizza")
    );
    assert_eq!(product_full.price, 450);
    assert_eq!(product_full.weight, 400);
    assert_eq!(product_full.image_url, "http://example.com/margarita.jpg");
}
