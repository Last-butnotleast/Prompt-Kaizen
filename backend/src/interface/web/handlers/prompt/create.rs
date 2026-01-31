use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::interface::web::handlers::{app_state::AppState, auth::extract_user_id};

#[derive(Deserialize)]
pub struct CreatePromptRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct CreatePromptResponse {
    pub id: String,
}

pub async fn create_prompt(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<CreatePromptRequest>,
) -> Result<(StatusCode, Json<CreatePromptResponse>), (StatusCode, String)> {
    let user_id = extract_user_id(&headers)?;

    let id = state
        .create_prompt
        .execute(user_id, payload.name, payload.description)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok((StatusCode::CREATED, Json(CreatePromptResponse {
        id: id.to_string()
    })))
}