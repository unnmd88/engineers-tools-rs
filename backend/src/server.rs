use axum::{Router, routing::{get, post}};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::{root, health, info, generate, parse};
use crate::docs::ApiDoc;  // Этот импорт должен быть

#[derive(Clone)]
pub struct AppState {
    pub server_running: Arc<Mutex<bool>>,
}

/// Сигнал для graceful shutdown
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Ошибка при установке обработчика Ctrl+C");
    println!("\n🛑 Получен сигнал завершения, останавливаем сервер...");
}

/// Запуск сервера
pub async fn run_server() {
    let state = AppState {
        server_running: Arc::new(Mutex::new(true)),
    };
    
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/info", get(info))
        .route("/generate", post(generate))
        .route("/parse", post(parse))
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    println!("🚀 Сервер запущен на http://{}", addr);
    println!("📚 Документация: http://{}/swagger-ui", addr);
    println!("💡 Для проверки: http://{}/health", addr);
    println!("{}", "─".repeat(50));
    
    let listener = TcpListener::bind(addr).await.unwrap();
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    
    println!("✅ Сервер остановлен");
}