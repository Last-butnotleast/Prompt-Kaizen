use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    Json,
};
use serde::Serialize;
use std::sync::Arc;

use crate::interface::web::handlers::{
    app_state::AppState,
    auth::extract_user_id_with_api_key,
    uuid_helpers::parse_uuid,
};

#[derive(Serialize)]
pub struct AnalyzeFeedbackResponse {
    pub suggestion_id: String,
    pub suggested_content: String,
    pub ai_rationale: String,
}

pub async fn analyze_feedback(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((prompt_id, version_id)): Path<(String, String)>,
) -> Result<(StatusCode, Json<AnalyzeFeedbackResponse>), (StatusCode, String)> {
    let user_id = extract_user_id_with_api_key(&headers, state.api_key_repository.clone()).await?;
    let prompt_uuid = parse_uuid(&prompt_id, "prompt_id")?;
    let version_uuid = parse_uuid(&version_id, "version_id")?;

    let suggestion_id = state
        .analyze_feedback_and_suggest
        .execute(prompt_uuid, user_id, version_uuid)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    let prompt = state.get_prompt
        .execute(prompt_uuid, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let version = prompt.find_version_by_id(version_uuid)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Version not found".to_string()))?;

    let suggestion = version.find_suggestion(suggestion_id)
        .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, "Suggestion not found after creation".to_string()))?;

    Ok((StatusCode::CREATED, Json(AnalyzeFeedbackResponse {
        suggestion_id: suggestion_id.to_string(),
        suggested_content: suggestion.suggested_content().to_string(),
        ai_rationale: suggestion.ai_rationale().to_string(),
    })))
}