use axum::{
    routing::post,
    Router,
};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};

use super::handlers::prompt_handlers::{
    create_prompt, create_version, tag_version, submit_feedback, AppState,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/prompts", post(create_prompt))
        .route("/prompts/{prompt_id}/versions", post(create_version))
        .route("/prompts/{prompt_id}/tags", post(tag_version))
        .route("/prompts/{prompt_id}/feedback", post(submit_feedback))
        .layer(cors)
        .with_state(state)
}