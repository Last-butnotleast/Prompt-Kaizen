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
pub struct CreateVersionRequest {
    pub version: String,
    pub content: String,
    pub changelog: Option<String>,
}

#[derive(Serialize)]
pub struct CreateVersionResponse {
    pub version_id: String,
}

pub async fn create_version(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(prompt_id): Path<String>,
    Json(payload): Json<CreateVersionRequest>,
) -> Result<(StatusCode, Json<CreateVersionResponse>), (StatusCode, String)> {
    let user_id = extract_user_id_with_api_key(&headers, state.api_key_repository.clone()).await?;
    let prompt_uuid = parse_uuid(&prompt_id, "prompt_id")?;

    let version_id = state
        .create_version
        .execute(prompt_uuid, user_id, payload.version, payload.content, payload.changelog)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok((StatusCode::CREATED, Json(CreateVersionResponse {
        version_id: version_id.to_string()
    })))
}