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
use crate::domain::prompt::ContentType;

#[derive(Deserialize)]
pub struct CreateVersionRequest {
    pub version: String,
    pub content: String,
    pub content_type: String,
    pub variables: Option<Vec<String>>,
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

    let content_type = match payload.content_type.as_str() {
        "static" => ContentType::Static,
        "template" => ContentType::Template,
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid content_type. Must be 'static' or 'template'".to_string())),
    };

    let version_id = state
        .create_version
        .execute(
            prompt_uuid,
            user_id,
            payload.version,
            payload.content,
            content_type,
            payload.variables,
            payload.changelog
        )
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok((StatusCode::CREATED, Json(CreateVersionResponse {
        version_id: version_id.to_string()
    })))
}