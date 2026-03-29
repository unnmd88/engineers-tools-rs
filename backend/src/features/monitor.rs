use axum::{
    Json, 
    Router, 
    http::StatusCode,
    response::IntoResponse, 
    routing::get 
};
use utoipa;

use axum::extract::ws::{WebSocketUpgrade, WebSocket, Message};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::shared::ApiError;

// Хендлер
#[utoipa::path(
    post,
    path = "/api/v1/common/monitor",
    responses(
        (status = 200, description = "SCN успешно сгенерирован"),
        (status = 400, description = "Ошибка ввода")
    ),
    tag = "monitor"
)]
pub async fn ws_monitor_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}


async fn handle_socket(mut socket: WebSocket) {
    println!("🟢 WS подключен");

    // Ждём конфиг от клиента
    let msg = socket.recv().await;

    let config = match msg {
        Some(Ok(Message::Text(text))) => {
            println!("📩 Получен конфиг: {}", text);
            text
        }
        _ => {
            println!("❌ Не получили конфиг");
            return;
        }
    };

    // Псевдо-парсинг (пока без struct)
    let mut interval = 1;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&config) {
        interval = json["interval"].as_u64().unwrap_or(1);
    }

    let mut ticker = tokio::time::interval(std::time::Duration::from_secs(interval));

    loop {
        ticker.tick().await;

        let data = format!(
            r#"{{"value": {}, "ts": {}}}"#,
            rand::random::<u8>(),
            chrono::Utc::now().timestamp()
        );

        if socket.send(Message::Text(data.into())).await.is_err() {
            println!("🔴 WS отключен");
            break;
        }
    }
}

pub fn router() -> Router {
    Router::new().route("/", get(ws_monitor_handler))
}

