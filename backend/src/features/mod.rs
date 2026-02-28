pub mod scn_generations;
pub mod potok; 
use axum::Router;

// Группируем общие фичи (пока только одна)
pub fn common_router() -> Router {
    Router::new()
        .nest("/scn-generations", scn_generations::router())
        // сюда добавятся другие общие по мере появления
        // .nest("/health", health::router())
        // .nest("/version", version::router())
}


pub fn potok_router() -> Router {
    Router::new()
        .nest("/tlc-condition-generations", potok::condition_generations_router())
}