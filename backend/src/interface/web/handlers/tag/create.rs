use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::interface::web::handlers::{app_state::AppState, auth::extract_user_id};

#[derive(Deserialize)]
pub struct TagVersionRequest {
    pub tag_name: String,
    pub version_id: String,
}

pub async fn tag_version(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(prompt_id): Path<String>,
    Json(payload): Json<TagVersionRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = extract_user_id(&headers)?;

    state
        .create_tag
        .execute(prompt_id, user_id, payload.tag_name, payload.version_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(StatusCode::OK)
}