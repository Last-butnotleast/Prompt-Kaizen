use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
};
use std::sync::Arc;

use crate::interface::web::handlers::{
    app_state::AppState,
    auth::extract_user_id_with_api_key,
    uuid_helpers::parse_uuid,
};

pub async fn delete_prompt(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(prompt_id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = extract_user_id_with_api_key(&headers, state.api_key_repository.clone()).await?;
    let prompt_uuid = parse_uuid(&prompt_id, "prompt_id")?;

    state
        .delete_prompt
        .execute(prompt_uuid, user_id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(StatusCode::NO_CONTENT)
}