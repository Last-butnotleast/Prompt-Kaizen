use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    Json,
};
use std::sync::Arc;

use crate::interface::web::handlers::{
    app_state::AppState,
    auth::extract_user_id,
    response_types::ApiKeyResponse,
};

pub async fn list_api_keys(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<ApiKeyResponse>>, (StatusCode, String)> {
    let user_id = extract_user_id(&headers)?;

    let api_keys = state
        .list_api_keys
        .execute(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(api_keys.iter().map(ApiKeyResponse::from).collect()))
}