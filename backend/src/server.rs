use axum::{
    Router, 
    routing::get,
    response::{Html, IntoResponse},
    http::StatusCode,
};
use std::net::TcpListener as StdTcpListener;
use std::net::SocketAddr;
use rust_embed::RustEmbed;
use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
// use tower_http::services::ServeDir;

use crate::features;

use crate::docs::ApiDoc;
use mime_guess;  

use dotenvy::from_filename;
use std::env;


// Структура конфигурации
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub api_prefix: String,
}

impl Config {
    pub fn from_env() -> Self {
        // Загружаем .env
        let env_file = env::var("ENV_FILE").unwrap();

      // 2. ЗАГРУЖАЕМ переменные из файла
        match from_filename(&env_file) {
            Ok(_) => println!("✅ Загружен файл: {}", env_file),
            Err(e) => println!("⚠️ Ошибка загрузки {}: {}", env_file, e),
        }
        
        // 3. ТЕПЕРЬ читаем переменные
        println!("🔍 DEBUG: ENVIRONMENT = {:?}", env::var("ENVIRONMENT"));
        println!("🔍 DEBUG: PORT = {:?}", env::var("PORT"));
        println!("🔍 DEBUG: HOST = {:?}", env::var("HOST"));
        
        let host = env::var("HOST").unwrap().to_string();
        let preferred_port: u16 = env::var("PORT").unwrap().parse().unwrap();
            
        
        let api_prefix = env::var("API_PREFIX").unwrap().to_string();
            
        // ИЩЕМ СВОБОДНЫЙ ПОРТ (для пользователей)
        let (port, is_preferred) = Self::find_free_port(&host, preferred_port);
        
        if is_preferred {
            println!("✅ Используем порт {}", port);
        } else {
            println!("🔍 Порт {} занят, используем свободный порт {}", preferred_port, port);
        }
        
        Config { host, port , api_prefix }
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


async fn embedded_static_handler(
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

// Для dev окружения, чтобы не изменения видны были при изменении файлов во ../frontend
async fn dev_static_handler(uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    
    if path.is_empty() || path == "/" {
        match tokio::fs::read_to_string("../frontend/index.html").await {
            Ok(html) => {
                // Читаем переменные окружения
                let host = env::var("HOST").unwrap().to_string();
                let port = env::var("PORT").unwrap().to_string();
                let current_env = env::var("ENVIRONMENT").unwrap().to_string();
                let api_prefix = env::var("API_PREFIX").unwrap().to_string();

                // Формируем конфиг на основе переменных окружения
                let config_script = format!(
                    r#"<script>
                        window.APP_CONFIG = {{
                            host: "{}",
                            port: {},
                            api_url: "http://{}:{}{}/",
                            environment: "{}"
                        }};
                        
                        window.DEV_MODE = true;
                    </script>"#,
                    host, port, host, port, api_prefix, current_env
                );
                
                let html_with_config = html.replace("<!-- CONFIG -->", &config_script);
                Html(html_with_config).into_response()
            }
            Err(_) => (StatusCode::NOT_FOUND, "index.html not found").into_response(),
        }
    } else {
        let full_path = format!("../frontend/{}", path);
        match tokio::fs::read(full_path).await {
            Ok(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                (
                    [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
                    content,
                )
                    .into_response()
            }
            Err(_) => (StatusCode::NOT_FOUND, "404").into_response(),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    // pub server_running: Arc<Mutex<bool>>,
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
        // server_running: Arc::new(Mutex::new(true)),
        config: config.clone(),  // теперь clone работает
    };

    // let is_dev = env::var("ENVIRONMENT").is_ok();

    let current_env = env::var("ENVIRONMENT").unwrap();
    
    let api_router = Router::new()
        .nest("/common", features::common_router())
        .nest("/potok", features::potok_router());

    // Основной роутер с префиксом
    let app = Router::new()
        .nest(&config.api_prefix, api_router) 
        .merge(SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi()));
        
    // Добавляем fallback в зависимости от окружения (без изменений)
    let app = if current_env == "DEV" {
        println!("📁 Режим разработки: файлы читаются с диска");
        app.fallback(get(dev_static_handler).post(dev_static_handler))
    } else if current_env == "PROD" {
        println!("📦 Режим production: используются встроенные файлы");
        app.fallback(
            get(embedded_static_handler)
                .post(embedded_static_handler)
                .with_state(state)
        )
    } else {
        panic!("Неподдерживаемое значение ENVIRONMENT: {}", current_env);
    };

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

