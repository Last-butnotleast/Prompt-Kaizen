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
};

pub async fn get_version(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((prompt_id, version_id)): Path<(String, String)>,
) -> Result<Json<VersionResponse>, (StatusCode, String)> {
    let user_id = extract_user_id(&headers)?;

    let version = state
        .get_version
        .execute(prompt_id, user_id, version_id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(Json(VersionResponse::from(&version)))
}