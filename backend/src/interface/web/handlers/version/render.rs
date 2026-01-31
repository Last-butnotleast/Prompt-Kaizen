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
pub struct RenderVersionRequest {
    pub context: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct RenderVersionResponse {
    pub rendered_content: String,
}

pub async fn render_version(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((prompt_id, version_id)): Path<(String, String)>,
    Json(payload): Json<RenderVersionRequest>,
) -> Result<Json<RenderVersionResponse>, (StatusCode, String)> {
    let user_id = extract_user_id_with_api_key(&headers, state.api_key_repository.clone()).await?;
    let prompt_uuid = parse_uuid(&prompt_id, "prompt_id")?;
    let version_uuid = parse_uuid(&version_id, "version_id")?;

    let rendered = state
        .render_version
        .execute(prompt_uuid, user_id, version_uuid, payload.context)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(Json(RenderVersionResponse {
        rendered_content: rendered
    }))
}

pub async fn render_version_by_tag(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((prompt_id, tag_name)): Path<(String, String)>,
    Json(payload): Json<RenderVersionRequest>,
) -> Result<Json<RenderVersionResponse>, (StatusCode, String)> {
    let user_id = extract_user_id_with_api_key(&headers, state.api_key_repository.clone()).await?;
    let prompt_uuid = parse_uuid(&prompt_id, "prompt_id")?;

    let rendered = state
        .render_version_by_tag
        .execute(prompt_uuid, user_id, tag_name, payload.context)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(Json(RenderVersionResponse {
        rendered_content: rendered
    }))
}