use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
};
use std::sync::Arc;

use crate::interface::web::handlers::{
    app_state::AppState,
    auth::extract_user_id,
    uuid_helpers::parse_uuid,
};

pub async fn delete_api_key(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(api_key_id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = extract_user_id(&headers).await?;
    let key_uuid = parse_uuid(&api_key_id, "api_key_id")?;

    state
        .delete_api_key
        .execute(key_uuid, user_id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(StatusCode::NO_CONTENT)
}