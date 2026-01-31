use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use super::handlers::prompt_handlers::{
    create_prompt, create_version, tag_version, submit_feedback, AppState,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/prompts", post(create_prompt))
        .route("/prompts/{prompt_id}/versions", post(create_version))
        .route("/prompts/{prompt_id}/tags", post(tag_version))
        .route("/prompts/{prompt_id}/feedback", post(submit_feedback))
        .with_state(state)
}