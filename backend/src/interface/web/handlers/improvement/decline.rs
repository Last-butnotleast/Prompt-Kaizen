use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::interface::web::handlers::{
    app_state::AppState,
    auth::extract_user_id_with_api_key,
    uuid_helpers::parse_uuid,
};

#[derive(Deserialize)]
pub struct DeclineImprovementSuggestionRequest {
    pub reason: String,
}

pub async fn decline_improvement_suggestion(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((prompt_id, version_id, suggestion_id)): Path<(String, String, String)>,
    Json(payload): Json<DeclineImprovementSuggestionRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = extract_user_id_with_api_key(&headers, state.api_key_repository.clone()).await?;
    let prompt_uuid = parse_uuid(&prompt_id, "prompt_id")?;
    let version_uuid = parse_uuid(&version_id, "version_id")?;
    let suggestion_uuid = parse_uuid(&suggestion_id, "suggestion_id")?;

    state
        .decline_improvement_suggestion
        .execute(prompt_uuid, user_id, version_uuid, suggestion_uuid, payload.reason)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(StatusCode::OK)
}