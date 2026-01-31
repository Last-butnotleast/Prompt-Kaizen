use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::interface::web::handlers::{app_state::AppState, auth::extract_user_id};

#[derive(Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateApiKeyResponse {
    pub id: String,
    pub api_key: String,  // Only returned once!
    pub message: String,
}

pub async fn create_api_key(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<CreateApiKeyResponse>), (StatusCode, String)> {
    let user_id = extract_user_id(&headers)?;

    let (id, api_key) = state
        .create_api_key
        .execute(user_id, payload.name)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok((StatusCode::CREATED, Json(CreateApiKeyResponse {
        id: id.to_string(),
        api_key,
        message: "Save this API key securely. You won't be able to see it again.".to_string(),
    })))
}