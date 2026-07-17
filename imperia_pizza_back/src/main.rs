use axum::http::{Method, header::CONTENT_TYPE};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer; // Автоматическое логирование HTTP-запросов
use tracing::info;
use tracing_subscriber::EnvFilter;

// Объявление модулей (без дублей)
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    // Инициализация переменных окружения
    dotenv().ok();

    // Инициализация логирования — ОДИН РАЗ, в самом начале main
    // RUST_LOG=info по умолчанию, если переменная не задана
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let database_url = env::var("DATABASE_URL").expect("Переменная DATABASE_URL не найдена в .env");

    // Пул соединений с БД
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Не удалось подключиться к базе данных");

    info!("✅ Успешное подключение к PostgreSQL!");

    // Настраиваем CORS для беспроблемной работы с фронтендом/TWA
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers([CONTENT_TYPE])
        .allow_origin(Any);

    // Сборка роутера — все домены собраны в routes::build_router()
    let app = routes::build_router()
        .with_state(pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Запуск сервера
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("🚀 Запуск сервера на http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
