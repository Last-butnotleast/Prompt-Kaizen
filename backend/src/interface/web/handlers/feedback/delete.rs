use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
};
use std::sync::Arc;

use crate::interface::web::handlers::{
    app_state::AppState,
    auth::extract_user_id,
    uuid_helpers::parse_uuid,
};

pub async fn delete_feedback(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((prompt_id, version_id, feedback_id)): Path<(String, String, String)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = extract_user_id(&headers)?;
    let prompt_uuid = parse_uuid(&prompt_id, "prompt_id")?;
    let version_uuid = parse_uuid(&version_id, "version_id")?;
    let feedback_uuid = parse_uuid(&feedback_id, "feedback_id")?;

    state
        .delete_feedback
        .execute(prompt_uuid, user_id, version_uuid, feedback_uuid)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(StatusCode::NO_CONTENT)
}