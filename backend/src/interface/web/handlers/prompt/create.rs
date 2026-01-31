use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::interface::web::handlers::{app_state::AppState, auth::extract_user_id_with_api_key};
use crate::domain::prompt::PromptType;

#[derive(Deserialize)]
pub struct CreatePromptRequest {
    pub name: String,
    pub description: Option<String>,
    pub prompt_type: String,
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
    let user_id = extract_user_id_with_api_key(&headers, state.api_key_repository.clone()).await?;

    let prompt_type = match payload.prompt_type.as_str() {
        "system" => PromptType::System,
        "user" => PromptType::User,
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid prompt_type. Must be 'system' or 'user'".to_string())),
    };

    let id = state
        .create_prompt
        .execute(user_id, payload.name, payload.description, prompt_type)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok((StatusCode::CREATED, Json(CreatePromptResponse {
        id: id.to_string()
    })))
}