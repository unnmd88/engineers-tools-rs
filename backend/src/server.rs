use axum::{
    Router, 
    routing::{get, post},
    response::{Html, IntoResponse},
    http::StatusCode,
};
use std::net::TcpListener as StdTcpListener;  // ВАЖНО: для проверки порта
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

use dotenvy::{dotenv, from_filename};  // ВАЖНО: правильный импорт dotenv
use std::env;


// Структура конфигурации
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
}



impl Config {
    pub fn from_env() -> Self {
        // Загружаем .env
        if let Ok(env_file) = env::var("ENV_FILE") {
            println!("📁 Загружаем конфиг из: {}", env_file);
            from_filename(&env_file).ok();
        } else {
            dotenv().ok();
        }

        // ОТЛАДКА: смотрим что в переменных окружения
        println!("🔍 DEBUG: env::var(\"PORT\") = {:?}", env::var("PORT"));
        println!("🔍 DEBUG: env::var(\"HOST\") = {:?}", env::var("HOST"));
        
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let preferred_port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);
        
        // ИЩЕМ СВОБОДНЫЙ ПОРТ (для пользователей)
        let (port, is_preferred) = Self::find_free_port(&host, preferred_port);
        
        if is_preferred {
            println!("✅ Используем порт {}", port);
        } else {
            println!("🔍 Порт {} занят, используем свободный порт {}", preferred_port, port);
        }
        
        Config { host, port }
    }
    
    // Поиск свободного порта
    fn find_free_port(host: &str, start_port: u16) -> (u16, bool) {
        println!("🔍 DEBUG: Ищем свободный порт начиная с {}", start_port);
        
        for port in start_port..start_port + 100 {
            match StdTcpListener::bind((host, port)) {
                Ok(_) => {
                    println!("🔍 DEBUG: Порт {} свободен", port);
                    return (port, port == start_port);
                }
                Err(e) => {
                    println!("🔍 DEBUG: Порт {} занят: {}", port, e);
                }
            }
        }
        println!("🔍 DEBUG: Не нашли свободный порт, возвращаем {}", start_port);
        (start_port, false)
    }
    
    pub fn addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Неверный формат адреса")
    }
}


// Встраиваем фронтенд
#[derive(RustEmbed)]
#[folder = "../frontend"]
struct Frontend;


async fn static_handler(
    uri: axum::http::Uri,
    state: axum::extract::State<AppState>,
) -> impl IntoResponse {
    let config = &state.config;
    let path = uri.path().trim_start_matches('/');
    
    if path.is_empty() || path == "/" {
        return match Frontend::get("index.html") {
            Some(content) => {
                let html = String::from_utf8_lossy(&content.data).to_string();
                
                // Вставляем конфиг
                let config_script = format!(
                    r#"<script>
                        window.APP_CONFIG = {{
                            host: "{}",
                            port: {}
                        }};
                    </script>"#,
                    config.host, config.port
                );
                
                let html_with_config = html.replace("<!-- CONFIG -->", &config_script);
                Html(html_with_config).into_response()
            }
            None => (StatusCode::NOT_FOUND, "index.html not found").into_response(),
        };
    }
    
    // Обработка статических файлов
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
    pub config: Config,
}


/// Сигнал для graceful shutdown
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Ошибка при установке обработчика Ctrl+C");
    println!("\n🛑 Получен сигнал завершения, останавливаем сервер...");
}

/// Запуск сервера
pub async fn run_server(config: Config) {  // ПРИНИМАЕМ CONFIG
    let state = AppState {
        server_running: Arc::new(Mutex::new(true)),
        config: config.clone(),  // теперь clone работает
    };
    
    let app = Router::new()
        .route("/health", get(health))
        .route("/info", get(info))
        .route("/generate", post(generate))
        .route("/parse", post(parse))
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state.clone())  // передаём state
        .fallback(move |uri| {  // используем move
            let state = state.clone();  // клонируем для каждого запроса
            async move {
                static_handler(uri, axum::extract::State(state)).await  // оборачиваем в State
            }
        });

    let addr = config.addr();  // ИСПОЛЬЗУЕМ CONFIG
    
    println!("{}", "=".repeat(50));
    println!("🚀 Сервер запущен на http://{}", addr);
    println!("📚 Документация: http://{}/swagger-ui", addr);
    println!("🌐 Веб-интерфейс: http://{}", addr);
    println!("💡 Health check: http://{}/health", addr);
    println!("{}", "=".repeat(50));
    
    let listener = TcpListener::bind(addr).await.unwrap();
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    
    println!("✅ Сервер остановлен");
}

