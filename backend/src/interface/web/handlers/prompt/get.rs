use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    Json,
};
use std::sync::Arc;

use crate::interface::web::handlers::{
    app_state::AppState,
    auth::extract_user_id,
    response_types::PromptResponse,
};

pub async fn get_prompt(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(prompt_id): Path<String>,
) -> Result<Json<PromptResponse>, (StatusCode, String)> {
    let user_id = extract_user_id(&headers)?;

    let prompt = state
        .get_prompt
        .execute(prompt_id, user_id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(Json(PromptResponse::from(&prompt)))
}