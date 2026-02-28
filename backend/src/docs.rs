use utoipa::OpenApi;

use crate::features::scn_generations::{
    ScnRequest, 
    ScnResponse
};
use crate::features::potok::condition_generations::{
    PotokConditionRequest,  
    PotokConditionResponse,  
};
use crate::shared::ApiError;



#[derive(OpenApi)]
#[openapi(
    paths(

        // Общие фичи
        crate::features::scn_generations::generate_scn,
        
        // Potok фичи
        crate::features::potok::condition_generations::generate_condition,
    ),
    components(
        schemas(
            // SCN фича
            ScnRequest,
            ScnResponse,
            
            // Condition фича
            PotokConditionRequest,
            PotokConditionResponse,
            
            // Общие ошибки
            ApiError,
        )
    ),
    tags(
        (name = "system", description = "Системные эндпоинты"),
        (name = "scn", description = "Генерация SCN"),
        (name = "potok", description = "Генерация условий для Potok"),
    ),
    info(
        title = "Traffic Core API",
        description = "API для инструментов транспортного инженера",
        version = "1.0.0",
    )
)]
pub struct ApiDoc;