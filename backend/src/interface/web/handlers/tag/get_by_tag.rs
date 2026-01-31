use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    Json,
};
use std::sync::Arc;

use crate::interface::web::handlers::{
    app_state::AppState,
    auth::extract_user_id_with_api_key,
    response_types::VersionResponse,
    uuid_helpers::parse_uuid,
};

pub async fn get_version_by_tag(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((prompt_id, tag_name)): Path<(String, String)>,
) -> Result<Json<VersionResponse>, (StatusCode, String)> {
    let user_id = extract_user_id_with_api_key(&headers, state.api_key_repository.clone()).await?;
    let prompt_uuid = parse_uuid(&prompt_id, "prompt_id")?;

    let version = state
        .get_version_by_tag
        .execute(prompt_uuid, user_id, tag_name)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(Json(VersionResponse::from(&version)))
}