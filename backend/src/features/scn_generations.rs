use axum::{
    Json,
    response::IntoResponse,
    http::StatusCode,
    Router,
    routing::post, 
};
use utoipa;

use traffic_controller::format::scn::to_scn_format;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::shared::ApiError;


#[derive(Debug, Deserialize, ToSchema)]
pub struct ScnRequest {
    /// Входные данные для генерации SCN
    pub input: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ScnResponse {
    /// Сгенерированный SCN
    pub output: String,
}

// Хендлер
#[utoipa::path(
    post,
    path = "/api/v1/common/scn-generations",
    request_body = ScnRequest,
    responses(
        (status = 200, description = "SCN успешно сгенерирован", body = ScnResponse),
        (status = 400, description = "Ошибка ввода")
    ),
    tag = "scn"
)]
pub async fn generate_scn(
    Json(req): Json<ScnRequest>,
) -> Result<impl IntoResponse, ApiError> {
    
    if req.input.is_empty() {
        return Err(ApiError::input_cant_be_empty());
    }

    if !req.input.is_ascii() {
        return Err(ApiError::bad_request("Только ASCII символы"));
    }

    if req.input.len() > 32 { 
        return Err(ApiError::bad_request("Максимум 32 символа"));
    }

    let response = ScnResponse {
        output: to_scn_format(&req.input),
    };
    
    Ok((StatusCode::OK, Json(response)))
}

pub fn router() -> Router {
    Router::new().route("/", post(generate_scn))
}

