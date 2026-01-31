use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};

use super::handlers::prompt_handlers::{
    create_prompt, update_prompt, get_prompt, list_prompts, delete_prompt,
    create_version, get_version, delete_version,
    tag_version, delete_tag, get_version_by_tag,
    submit_feedback, update_feedback, delete_feedback,
    AppState,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(Any)
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