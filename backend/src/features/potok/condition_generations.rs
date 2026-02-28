use axum::{
    Json,
    response::IntoResponse,
    http::StatusCode,
    Router,
    routing::post,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::shared::ApiError;
use traffic_controller::potok::conditions::{parse_input_expression, to_condition_string};

#[derive(Debug, Deserialize, ToSchema)]
pub struct PotokConditionRequest {
    pub input: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PotokConditionResponse {
    pub output: String,
}

fn generate_tlc_condition_string(string: &str) -> Result<String, String> {
    match parse_input_expression(string) {
        Ok(expr) => Ok(to_condition_string(&expr)),
        Err(e) => Err(e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/potok/tlc-condition-generations",
    request_body = PotokConditionRequest,
    responses(
        (status = 200, description = "Условие успешно сгенерировано", body = PotokConditionResponse),
        (status = 400, description = "Ошибка ввода")
    ),
    tag = "potok-generate-tlc-condition"
)]
pub async fn generate_condition(
    Json(req): Json<PotokConditionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    
    generate_tlc_condition_string(&req.input)
        .map(|output| {
            (StatusCode::OK, Json(PotokConditionResponse {
                output,
            }))
        })
        .map_err(|_| ApiError::bad_request("Некорректное условие"))
}

pub fn router() -> Router {
    Router::new().route("/", post(generate_condition))
}