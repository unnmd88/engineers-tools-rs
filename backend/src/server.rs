use axum::{
    Router, 
    routing::{get, post},
    response::{Html, IntoResponse},
    http::StatusCode,
};
use std::net::SocketAddr;
use rust_embed::RustEmbed;
use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::{health, info, generate, parse};
use crate::docs::ApiDoc;  // Этот импорт должен быть
use mime_guess;  

use std::env;
use dotenvy::from_filename;


#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
}


impl Config {
    pub fn from_env() -> Self {
        // Сначала проверяем, не указан ли конкретный .env файл
        if let Ok(env_file) = env::var("ENV_FILE") {
            println!("📁 Загружаем конфиг из: {}", env_file);
            from_filename(&env_file).ok();
        } else {
            // Иначе загружаем стандартный .env
            dotenv().ok();
        }
        
        // Читаем переменные с fallback на дефолты
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);
        
        // Выводим информацию о загруженном конфиге
        println!("📋 Текущие настройки:");
        println!("   HOST: {}", host);
        println!("   PORT: {}", port);
        if let Ok(db) = env::var("DATABASE_URL") {
            println!("   DATABASE_URL: {}", db);
        }
        
        Config { host, port }
    }
    
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}


// Встраиваем фронтенд
#[derive(RustEmbed)]
#[folder = "../frontend"]
struct Frontend;


async fn static_handler(uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    
    // Если корневой путь - отдаем index.html
    if path.is_empty() || path == "/" {
        return match Frontend::get("index.html") {
            Some(content) => {
                Html(String::from_utf8_lossy(&content.data).to_string()).into_response()
            }
            None => (StatusCode::NOT_FOUND, "index.html not found").into_response(),
        };
    }
    
    // Пытаемся найти запрошенный файл
    match Frontend::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            (
                [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
                content.data,
            )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, "404").into_response(),
    }
}






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
        // .route("/", get(root))
        .route("/health", get(health))
        .route("/info", get(info))
        .route("/generate", post(generate))
        .route("/parse", post(parse))
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
        .fallback(static_handler);

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