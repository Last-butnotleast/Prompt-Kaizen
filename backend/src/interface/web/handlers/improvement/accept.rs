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
pub struct AcceptImprovementSuggestionRequest {
    pub new_version: String,
    pub changelog: Option<String>,
}

#[derive(Serialize)]
pub struct AcceptImprovementSuggestionResponse {
    pub new_version_id: String,
}

pub async fn accept_improvement_suggestion(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((prompt_id, version_id, suggestion_id)): Path<(String, String, String)>,
    Json(payload): Json<AcceptImprovementSuggestionRequest>,
) -> Result<(StatusCode, Json<AcceptImprovementSuggestionResponse>), (StatusCode, String)> {
    let user_id = extract_user_id_with_api_key(&headers, state.api_key_repository.clone()).await?;
    let prompt_uuid = parse_uuid(&prompt_id, "prompt_id")?;
    let version_uuid = parse_uuid(&version_id, "version_id")?;
    let suggestion_uuid = parse_uuid(&suggestion_id, "suggestion_id")?;

    let new_version_id = state
        .accept_improvement_suggestion
        .execute(prompt_uuid, user_id, version_uuid, suggestion_uuid, payload.new_version, payload.changelog)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok((StatusCode::OK, Json(AcceptImprovementSuggestionResponse {
        new_version_id: new_version_id.to_string()
    })))
}