use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    Json,
};
use std::sync::Arc;

use crate::interface::web::handlers::{
    app_state::AppState,
    auth::extract_user_id,
    response_types::VersionResponse,
    uuid_helpers::parse_uuid,
};

pub async fn get_version(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((prompt_id, version_id)): Path<(String, String)>,
) -> Result<Json<VersionResponse>, (StatusCode, String)> {
    let user_id = extract_user_id(&headers)?;
    let prompt_uuid = parse_uuid(&prompt_id, "prompt_id")?;
    let version_uuid = parse_uuid(&version_id, "version_id")?;

    let version = state
        .get_version
        .execute(prompt_uuid, user_id, version_uuid)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(Json(VersionResponse::from(&version)))
}