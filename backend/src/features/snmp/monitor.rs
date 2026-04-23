use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
//use futures_util::sink::SinkExt as _;
//use futures_util::stream::StreamExt as _;
use serde_json::json;
use std::time::Duration;
use tokio::time;

pub fn router() -> Router {
    Router::new().route("/", get(ws_handler))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket2(mut socket: WebSocket) {
    println!("[WS] Новое соединение");

    // 1. Получаем конфигурацию от клиента
    let Some(Ok(Message::Text(config_text))) =
        socket.recv().await
    else {
        println!("[WS] Не получен конфиг, закрываем");
        return;
    };

    // 2. Парсим конфиг
    let config: serde_json::Value =
        match serde_json::from_str(&config_text) {
            Ok(v) => v,
            Err(e) => {
                socket
                    .send(Message::Text(
                        "Ошибка конфигурации"
                            .to_string()
                            .into(),
                    ))
                    .await;
                println!(
                    "[WS] Ошибка парсинга конфига: {}",
                    e
                );
                return;
            }
        };

    let interval_secs = config
        .get("interval")
        .and_then(|v| v.as_u64())
        .unwrap_or(1);

    let interval = Duration::from_secs(interval_secs);
    let mut ticker = time::interval(interval);
    ticker.tick().await; // пропускаем первый тик

    println!(
        "[WS] Запущен мониторинг с интервалом {} сек",
        interval_secs
    );

    // 3. Основной цикл
    loop {
        tokio::select! {
            // Канал получения сообщений от клиента
            msg = socket.recv() => {
                match msg {
                    // Клиент закрыл соединение
                    None => {
                        println!("[WS] Клиент закрыл соединение");
                        break;
                    }
                    // Получено сообщение о закрытии
                    Some(Ok(Message::Close(_))) => {
                        println!("[WS] Получен Close frame");
                        break;
                    }
                    // Ошибка приёма
                    Some(Err(e)) => {
                        println!("[WS] Ошибка приёма: {}", e);
                        break;
                    }
                    // Любые другие сообщения игнорируем
                    _ => {}
                }
            }
            // Канал отправки данных по таймеру
            _ = ticker.tick() => {
                let data = json!({
                    "value": rand::random::<u8>(),
                    "ts": chrono::Utc::now().timestamp(),
                });

                let msg = Message::Text(data.to_string().into());

                if let Err(e) = socket.send(msg).await {
                    println!("[WS] Ошибка отправки: {}", e);
                    break;
                }
            }
        }
    }

    println!("[WS] Соединение завершено");
}

async fn handle_socket(mut socket: WebSocket) {}
