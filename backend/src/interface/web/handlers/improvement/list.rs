use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    Json,
};
use std::sync::Arc;

use crate::interface::web::handlers::{
    app_state::AppState,
    auth::extract_user_id_with_api_key,
    response_types::ImprovementSuggestionResponse,
    uuid_helpers::parse_uuid,
};

pub async fn list_suggestions_for_version(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((prompt_id, version_id)): Path<(String, String)>,
) -> Result<Json<Vec<ImprovementSuggestionResponse>>, (StatusCode, String)> {
    let user_id = extract_user_id_with_api_key(&headers, state.api_key_repository.clone()).await?;
    let prompt_uuid = parse_uuid(&prompt_id, "prompt_id")?;
    let version_uuid = parse_uuid(&version_id, "version_id")?;

    let suggestions = state
        .get_suggestions_for_version
        .execute(prompt_uuid, user_id, version_uuid)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(Json(suggestions.iter().map(ImprovementSuggestionResponse::from).collect()))
}