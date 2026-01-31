use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::interface::web::handlers::{app_state::AppState, auth::extract_user_id};

#[derive(Deserialize)]
pub struct UpdatePromptRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
}

pub async fn update_prompt(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(prompt_id): Path<String>,
    Json(payload): Json<UpdatePromptRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = extract_user_id(&headers)?;

    state
        .update_prompt
        .execute(prompt_id, user_id, payload.name, payload.description)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(StatusCode::OK)
}