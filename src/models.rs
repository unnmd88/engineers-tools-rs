use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ConditionRequest {
    pub input: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GenerateResponse {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ParseResponse {
    pub input: String,
    pub ast: String,
    pub ddr: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub input: String,
}