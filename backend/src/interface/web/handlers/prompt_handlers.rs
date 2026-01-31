use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::application::use_cases::{
    CreatePromptUseCase, CreateVersionUseCase, ManageTagsUseCase, SubmitFeedbackUseCase,
};

pub struct AppState {
    pub create_prompt: Arc<CreatePromptUseCase>,
    pub create_version: Arc<CreateVersionUseCase>,
    pub manage_tags: Arc<ManageTagsUseCase>,
    pub submit_feedback: Arc<SubmitFeedbackUseCase>,
}

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
    Json(payload): Json<CreatePromptRequest>,
) -> Result<(StatusCode, Json<CreatePromptResponse>), (StatusCode, String)> {
    let id = state
        .create_prompt
        .execute(payload.name, payload.description)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok((StatusCode::CREATED, Json(CreatePromptResponse { id })))
}

#[derive(Deserialize)]
pub struct CreateVersionRequest {
    pub version: String,
    pub content: String,
    pub changelog: Option<String>,
}

#[derive(Serialize)]
pub struct CreateVersionResponse {
    pub version_id: String,
}

pub async fn create_version(
    State(state): State<Arc<AppState>>,
    Path(prompt_id): Path<String>,
    Json(payload): Json<CreateVersionRequest>,
) -> Result<(StatusCode, Json<CreateVersionResponse>), (StatusCode, String)> {
    let version_id = state
        .create_version
        .execute(prompt_id, payload.version, payload.content, payload.changelog)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok((StatusCode::CREATED, Json(CreateVersionResponse { version_id })))
}

#[derive(Deserialize)]
pub struct TagVersionRequest {
    pub tag_name: String,
    pub version_id: String,
}

pub async fn tag_version(
    State(state): State<Arc<AppState>>,
    Path(prompt_id): Path<String>,
    Json(payload): Json<TagVersionRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .manage_tags
        .tag_version(prompt_id, payload.tag_name, payload.version_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(StatusCode::OK)
}

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
    Path(prompt_id): Path<String>,
    Json(payload): Json<SubmitFeedbackRequest>,
) -> Result<(StatusCode, Json<SubmitFeedbackResponse>), (StatusCode, String)> {
    let feedback_id = state
        .submit_feedback
        .execute(prompt_id, payload.version_id, payload.rating, payload.comment)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok((StatusCode::CREATED, Json(SubmitFeedbackResponse { feedback_id })))
}