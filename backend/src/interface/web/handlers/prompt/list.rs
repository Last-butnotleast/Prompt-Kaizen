use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    Json,
};
use std::sync::Arc;

use crate::interface::web::handlers::{
    app_state::AppState,
    auth::extract_user_id,
    response_types::PromptResponse,
};

pub async fn list_prompts(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<PromptResponse>>, (StatusCode, String)> {
    let user_id = extract_user_id(&headers)?;

    let prompts = state
        .list_prompts
        .execute(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(prompts.iter().map(PromptResponse::from).collect()))
}