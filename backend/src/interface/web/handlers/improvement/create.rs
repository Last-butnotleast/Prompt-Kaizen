use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::interface::web::handlers::{
    app_state::AppState,
    auth::extract_user_id_with_api_key,
    uuid_helpers::parse_uuid,
};

#[derive(Deserialize)]
pub struct CreateImprovementSuggestionRequest {
    pub version_id: String,
    pub suggested_content: String,
    pub ai_rationale: String,
}

#[derive(Serialize)]
pub struct CreateImprovementSuggestionResponse {
    pub suggestion_id: String,
}

pub async fn create_improvement_suggestion(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(prompt_id): Path<String>,
    Json(payload): Json<CreateImprovementSuggestionRequest>,
) -> Result<(StatusCode, Json<CreateImprovementSuggestionResponse>), (StatusCode, String)> {
    let user_id = extract_user_id_with_api_key(&headers, state.api_key_repository.clone()).await?;
    let prompt_uuid = parse_uuid(&prompt_id, "prompt_id")?;
    let version_uuid = parse_uuid(&payload.version_id, "version_id")?;

    let suggestion_id = state
        .create_improvement_suggestion
        .execute(prompt_uuid, user_id, version_uuid, payload.suggested_content, payload.ai_rationale)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok((StatusCode::CREATED, Json(CreateImprovementSuggestionResponse {
        suggestion_id: suggestion_id.to_string()
    })))
}