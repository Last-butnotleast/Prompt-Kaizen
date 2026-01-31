use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::interface::web::handlers::{
    app_state::AppState,
    auth::extract_user_id,
    uuid_helpers::parse_uuid,
};

#[derive(Deserialize)]
pub struct UpdateFeedbackRequest {
    pub rating: Option<u8>,
    pub comment: Option<Option<String>>,
}

pub async fn update_feedback(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((prompt_id, version_id, feedback_id)): Path<(String, String, String)>,
    Json(payload): Json<UpdateFeedbackRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = extract_user_id(&headers)?;
    let prompt_uuid = parse_uuid(&prompt_id, "prompt_id")?;
    let version_uuid = parse_uuid(&version_id, "version_id")?;
    let feedback_uuid = parse_uuid(&feedback_id, "feedback_id")?;

    state
        .update_feedback
        .execute(prompt_uuid, user_id, version_uuid, feedback_uuid, payload.rating, payload.comment)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(StatusCode::OK)
}