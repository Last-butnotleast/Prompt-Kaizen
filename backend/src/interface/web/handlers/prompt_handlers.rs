use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::application::use_cases::*;
use crate::domain::prompt::{Prompt, PromptVersion, Feedback};
use chrono::{DateTime, Utc};

pub struct AppState {
    pub create_prompt: Arc<CreatePrompt>,
    pub update_prompt: Arc<UpdatePrompt>,
    pub get_prompt: Arc<GetPrompt>,
    pub list_prompts: Arc<ListPrompts>,
    pub delete_prompt: Arc<DeletePrompt>,

    pub create_version: Arc<CreateVersion>,
    pub get_version: Arc<GetVersion>,
    pub delete_version: Arc<DeleteVersion>,

    pub create_tag: Arc<CreateTag>,
    pub delete_tag: Arc<DeleteTag>,
    pub get_version_by_tag: Arc<GetVersionByTag>,

    pub submit_feedback: Arc<SubmitFeedback>,
    pub update_feedback: Arc<UpdateFeedback>,
    pub delete_feedback: Arc<DeleteFeedback>,
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

#[derive(Deserialize)]
pub struct UpdatePromptRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
}

#[derive(Serialize)]
pub struct PromptResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub versions: Vec<VersionResponse>,
    pub tags: Vec<TagResponse>,
}

#[derive(Serialize)]
pub struct VersionResponse {
    pub id: String,
    pub version: String,
    pub digest: String,
    pub content: String,
    pub changelog: Option<String>,
    pub created_at: DateTime<Utc>,
    pub average_rating: Option<f64>,
    pub feedback_count: usize,
}

#[derive(Serialize)]
pub struct TagResponse {
    pub id: String,
    pub name: String,
    pub version_id: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct FeedbackResponse {
    pub id: String,
    pub rating: u8,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<&Prompt> for PromptResponse {
    fn from(prompt: &Prompt) -> Self {
        Self {
            id: prompt.id().to_string(),
            name: prompt.name().to_string(),
            description: prompt.description().map(|s| s.to_string()),
            created_at: prompt.created_at(),
            updated_at: prompt.updated_at(),
            versions: prompt.versions().iter().map(VersionResponse::from).collect(),
            tags: prompt.tags().iter().map(TagResponse::from).collect(),
        }
    }
}

impl From<&PromptVersion> for VersionResponse {
    fn from(version: &PromptVersion) -> Self {
        Self {
            id: version.id().to_string(),
            version: version.version().to_string(),
            digest: version.digest().to_string(),
            content: version.content().to_string(),
            changelog: version.changelog().map(|s| s.to_string()),
            created_at: version.created_at(),
            average_rating: version.average_rating(),
            feedback_count: version.feedbacks().len(),
        }
    }
}

impl From<&crate::domain::prompt::Tag> for TagResponse {
    fn from(tag: &crate::domain::prompt::Tag) -> Self {
        Self {
            id: tag.id().to_string(),
            name: tag.name().to_string(),
            version_id: tag.version_id().to_string(),
            updated_at: tag.updated_at(),
        }
    }
}

impl From<&Feedback> for FeedbackResponse {
    fn from(feedback: &Feedback) -> Self {
        Self {
            id: feedback.id().to_string(),
            rating: feedback.rating(),
            comment: feedback.comment().map(|s| s.to_string()),
            created_at: feedback.created_at(),
        }
    }
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

pub async fn update_prompt(
    State(state): State<Arc<AppState>>,
    Path(prompt_id): Path<String>,
    Json(payload): Json<UpdatePromptRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .update_prompt
        .execute(prompt_id, payload.name, payload.description)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(StatusCode::OK)
}

pub async fn get_prompt(
    State(state): State<Arc<AppState>>,
    Path(prompt_id): Path<String>,
) -> Result<Json<PromptResponse>, (StatusCode, String)> {
    let prompt = state
        .get_prompt
        .execute(prompt_id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(Json(PromptResponse::from(&prompt)))
}

pub async fn list_prompts(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<PromptResponse>>, (StatusCode, String)> {
    let prompts = state
        .list_prompts
        .execute()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(prompts.iter().map(PromptResponse::from).collect()))
}

pub async fn delete_prompt(
    State(state): State<Arc<AppState>>,
    Path(prompt_id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .delete_prompt
        .execute(prompt_id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(StatusCode::NO_CONTENT)
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

pub async fn get_version(
    State(state): State<Arc<AppState>>,
    Path((prompt_id, version_id)): Path<(String, String)>,
) -> Result<Json<VersionResponse>, (StatusCode, String)> {
    let version = state
        .get_version
        .execute(prompt_id, version_id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(Json(VersionResponse::from(&version)))
}

pub async fn delete_version(
    State(state): State<Arc<AppState>>,
    Path((prompt_id, version_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .delete_version
        .execute(prompt_id, version_id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(StatusCode::NO_CONTENT)
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
        .create_tag
        .execute(prompt_id, payload.tag_name, payload.version_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(StatusCode::OK)
}

pub async fn get_version_by_tag(
    State(state): State<Arc<AppState>>,
    Path((prompt_id, tag_name)): Path<(String, String)>,
) -> Result<Json<VersionResponse>, (StatusCode, String)> {
    let version = state
        .get_version_by_tag
        .execute(prompt_id, tag_name)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(Json(VersionResponse::from(&version)))
}

pub async fn delete_tag(
    State(state): State<Arc<AppState>>,
    Path((prompt_id, tag_name)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .delete_tag
        .execute(prompt_id, tag_name)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(StatusCode::NO_CONTENT)
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

#[derive(Deserialize)]
pub struct UpdateFeedbackRequest {
    pub rating: Option<u8>,
    pub comment: Option<Option<String>>,
}

pub async fn update_feedback(
    State(state): State<Arc<AppState>>,
    Path((prompt_id, version_id, feedback_id)): Path<(String, String, String)>,
    Json(payload): Json<UpdateFeedbackRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .update_feedback
        .execute(prompt_id, version_id, feedback_id, payload.rating, payload.comment)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(StatusCode::OK)
}

pub async fn delete_feedback(
    State(state): State<Arc<AppState>>,
    Path((prompt_id, version_id, feedback_id)): Path<(String, String, String)>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .delete_feedback
        .execute(prompt_id, version_id, feedback_id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(StatusCode::NO_CONTENT)
}