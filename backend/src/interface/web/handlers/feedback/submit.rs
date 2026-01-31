use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::interface::web::handlers::{app_state::AppState, auth::extract_user_id};

#[derive(Deserialize)]
pub struct SubmitFeedbackRequest {
    pub version_id: String,
    pub rating: u8,
    pub comment: Option<String>,
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
    let user_id = extract_user_id(&headers)?;

    let feedback_id = state
        .submit_feedback
        .execute(prompt_id, user_id, payload.version_id, payload.rating, payload.comment)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok((StatusCode::CREATED, Json(SubmitFeedbackResponse { feedback_id })))
}