use axum::{
    Json,
    response::IntoResponse,
    http::StatusCode,
    extract::State,
};
use serde_json::json;
use utoipa;
use traffic_core::conditions::{Parser, generate_condition};

use crate::models::{ConditionRequest, GenerateResponse, ParseResponse, ErrorResponse};
use crate::server::AppState;

// #[utoipa::path(
//     get,
//     path = "/",
//     responses((status = 200, description = "Приветственное сообщение", body = String)),
//     tag = "system"
// )]
// pub async fn root() -> &'static str {
//     "Traffic Core API"
// }

#[utoipa::path(
    get,
    path = "/health",
    responses((status = 200, description = "Сервер работает", body = String)),
    tag = "system"
)]
pub async fn health() -> &'static str {
    "OK"
}

#[utoipa::path(
    get,
    path = "/info",
    responses((status = 200, description = "Информация об API", body = serde_json::Value)),
    tag = "system"
)]
pub async fn info() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({
        "name": "Traffic Core API",
        "version": "0.1.0",
        "endpoints": ["/", "/health", "/info", "/generate", "/parse"]
    })))
}

#[utoipa::path(
    post,
    path = "/generate",
    request_body = ConditionRequest,
    responses(
        (status = 200, description = "Условие успешно сгенерировано", body = GenerateResponse),
        (status = 400, description = "Ошибка в формате ввода", body = ErrorResponse)
    ),
    tag = "conditions"
)]
pub async fn generate(
    State(_state): State<AppState>,
    Json(req): Json<ConditionRequest>,
) -> impl IntoResponse {
    let result = generate_condition(&req.input);
    let response = GenerateResponse {
        input: req.input.clone(),
        output: result,
    };
    (StatusCode::OK, Json(response)).into_response()
}

#[utoipa::path(
    post,
    path = "/parse",
    request_body = ConditionRequest,
    responses(
        (status = 200, description = "Условие успешно разобрано", body = ParseResponse),
        (status = 400, description = "Ошибка парсинга", body = ErrorResponse)
    ),
    tag = "conditions"
)]
pub async fn parse(
    State(_state): State<AppState>,
    Json(req): Json<ConditionRequest>,
) -> impl IntoResponse {
    let mut parser = Parser::new(&req.input);
    match parser.parse() {
        Ok(expr) => {
            let response = ParseResponse {
                input: req.input.clone(),
                ast: format!("{:#?}", expr),
                ddr: expr.to_ddr_string(),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            let error = ErrorResponse {
                error: e,
                input: req.input,
            };
            (StatusCode::BAD_REQUEST, Json(error)).into_response()
        }
    }
}

