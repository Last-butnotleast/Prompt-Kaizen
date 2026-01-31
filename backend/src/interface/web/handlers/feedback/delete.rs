use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
};
use std::sync::Arc;

use crate::interface::web::handlers::{app_state::AppState, auth::extract_user_id};

pub async fn delete_feedback(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((prompt_id, version_id, feedback_id)): Path<(String, String, String)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = extract_user_id(&headers)?;

    state
        .delete_feedback
        .execute(prompt_id, user_id, version_id, feedback_id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(StatusCode::NO_CONTENT)
}