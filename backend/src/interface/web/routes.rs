use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use http::Method;
use tower_http::cors::{CorsLayer, Any};

use super::handlers::{
    AppState,
    prompt::{create_prompt, update_prompt, get_prompt, list_prompts, delete_prompt},
    version::{create_version, get_version, delete_version},
    tag::{tag_version, delete_tag, get_version_by_tag},
    feedback::{submit_feedback, update_feedback, delete_feedback},
};

pub fn create_router(state: Arc<AppState>) -> Router {
    let allowed_origin = std::env::var("ALLOWED_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());

    let cors = CorsLayer::new()
        .allow_origin(allowed_origin.parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    Router::new()
        .route("/prompts", post(create_prompt).get(list_prompts))
        .route("/prompts/{prompt_id}", get(get_prompt).put(update_prompt).delete(delete_prompt))
        .route("/prompts/{prompt_id}/versions", post(create_version))
        .route("/prompts/{prompt_id}/versions/{version_id}", get(get_version).delete(delete_version))
        .route("/prompts/{prompt_id}/tags", post(tag_version))
        .route("/prompts/{prompt_id}/tags/{tag_name}", delete(delete_tag))
        .route("/prompts/{prompt_id}/tags/{tag_name}/version", get(get_version_by_tag))
        .route("/prompts/{prompt_id}/feedback", post(submit_feedback))
        .route("/prompts/{prompt_id}/versions/{version_id}/feedback/{feedback_id}",
               put(update_feedback).delete(delete_feedback))
        .layer(cors)
        .with_state(state)
}