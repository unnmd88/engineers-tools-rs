use utoipa::OpenApi;
use crate::models::{ConditionRequest, GenerateResponse, ParseResponse, ErrorResponse};

// Импортируем все хендлеры, которые используем в документации
use crate::handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        // handlers::root,
        handlers::health,
        handlers::info,
        handlers::generate,
        handlers::parse
    ),
    components(
        schemas(
            ConditionRequest,
            GenerateResponse,
            ParseResponse,
            ErrorResponse
        )
    ),
    tags(
        (name = "system", description = "Системные эндпоинты"),
        (name = "conditions", description = "Работа с условиями DDR/MR")
    ),
    info(
        title = "Traffic Core API",
        description = "API для генерации и парсинга условий светофоров (DDR/MR)",
        version = "1.0.0",
    )
)]
pub struct ApiDoc;