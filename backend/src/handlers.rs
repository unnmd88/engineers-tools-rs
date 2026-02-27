use axum::{
    Json,
    response::IntoResponse,
    http::StatusCode,
    extract::State,
};
use serde_json::json;
use utoipa;
// use traffic_core::conditions::{
//     parse_ddr_expression, 
//     to_ddr_string, 
// };
use traffic_controller::potok::conditions::{parse_input_expression, to_condition_string};
use traffic_controller::format::scn::to_scn_format;

use crate::models::{ConditionRequest, GenerateResponse, ErrorResponse};
use crate::server::AppState;


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
        "endpoints": ["/", "/health", "/info", "/generate",]
    })))
}

#[utoipa::path(
    post,
    path = "/api/v1/common/generate-scn",
    request_body = ConditionRequest,
    responses(
        (status = 200, description = "Условие успешно сгенерировано", body = GenerateResponse),
        (status = 400, description = "Ошибка в формате ввода", body = ErrorResponse)
    ),
    tag = "generate-scn"
)]
pub async fn generate_scn(
    State(_state): State<AppState>,
    Json(req): Json<ConditionRequest>,
) -> impl IntoResponse {

    let response = GenerateResponse {
        input: req.input.clone(),
        output: to_scn_format(&req.input),
    };
    (StatusCode::OK, Json(response)).into_response()
}


#[utoipa::path(
    post,
    path = "/api/v1/potok/generate-condition",
    request_body = ConditionRequest,
    responses(
        (status = 200, description = "Условие успешно сгенерировано", body = GenerateResponse),
        (status = 400, description = "Ошибка в формате ввода", body = ErrorResponse)
    ),
    tag = "conditions"
)]
pub async fn generate_potok_condition(
    State(_state): State<AppState>,
    Json(req): Json<ConditionRequest>,
) -> impl IntoResponse {
    match parse_input_expression(&req.input) {
        Ok(expr) => {
            let response = GenerateResponse {
                input: req.input.clone(),
                output: to_condition_string(&expr),
            };
            (StatusCode::OK, Json(response)).into_response()
        },
        Err(e) => {
            let response = GenerateResponse {
                input: req.input.clone(),
                output: e.to_string(),
            };
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
    }
}

