use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::interface::web::handlers::{
    app_state::AppState,
    auth::extract_user_id_with_api_key,
    uuid_helpers::parse_uuid,
};

#[derive(Deserialize)]
pub struct SubmitFeedbackRequest {
    pub version_id: String,
    pub rating: u8,
    pub comment: Option<String>,
    pub test_input: Option<String>,
    pub test_actual_output: Option<String>,
    pub test_expected_output: Option<String>,
}

#[derive(Serialize)]
pub struct SubmitFeedbackResponse {
    pub feedback_id: String,
}

pub async fn submit_feedback(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(prompt_id): Path<String>,
    Json(payload): Json<SubmitFeedbackRequest>,
) -> Result<(StatusCode, Json<SubmitFeedbackResponse>), (StatusCode, String)> {
    let user_id = extract_user_id_with_api_key(&headers, state.api_key_repository.clone()).await?;
    let prompt_uuid = parse_uuid(&prompt_id, "prompt_id")?;
    let version_uuid = parse_uuid(&payload.version_id, "version_id")?;

    let feedback_id = state
        .submit_feedback
        .execute(
            prompt_uuid,
            user_id,
            version_uuid,
            payload.rating,
            payload.comment,
            payload.test_input,
            payload.test_actual_output,
            payload.test_expected_output,
        )
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok((StatusCode::CREATED, Json(SubmitFeedbackResponse {
        feedback_id: feedback_id.to_string()
    })))
}