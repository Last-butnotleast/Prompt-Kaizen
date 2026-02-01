use serde::Serialize;
use chrono::{DateTime, Utc};
use crate::domain::api_key::ApiKey;
use crate::domain::prompt::{Prompt, PromptVersion, Tag, Feedback, PromptType, ContentType, ImprovementSuggestion, SuggestionStatus};

#[derive(Serialize)]
pub struct PromptResponse {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub prompt_type: String,
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
    pub content_type: String,
    pub variables: Option<Vec<String>>,
    pub changelog: Option<String>,
    pub created_at: DateTime<Utc>,
    pub average_rating: Option<f64>,
    pub feedback_count: usize,
    pub feedback: Vec<FeedbackResponse>,
    pub improvement_suggestions: Vec<ImprovementSuggestionResponse>,
}

#[derive(Serialize)]
pub struct TagResponse {
    pub id: String,
    pub name: String,
    pub version_id: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct TestScenarioResponse {
    pub input: String,
    pub actual_output: String,
    pub expected_output: Option<String>,
}

#[derive(Serialize)]
pub struct FeedbackResponse {
    pub id: String,
    pub rating: u8,
    pub comment: Option<String>,
    pub test_scenario: Option<TestScenarioResponse>,
    pub created_at: DateTime<Utc>,
}

impl From<&Prompt> for PromptResponse {
    fn from(prompt: &Prompt) -> Self {
        let prompt_type = match prompt.prompt_type() {
            PromptType::System => "system",
            PromptType::User => "user",
        };

        Self {
            id: prompt.id().to_string(),
            user_id: prompt.user_id().to_string(),
            name: prompt.name().to_string(),
            description: prompt.description().map(|s| s.to_string()),
            prompt_type: prompt_type.to_string(),
            created_at: prompt.created_at(),
            updated_at: prompt.updated_at(),
            versions: prompt.versions().iter().map(VersionResponse::from).collect(),
            tags: prompt.tags().iter().map(TagResponse::from).collect(),
        }
    }
}

impl From<&PromptVersion> for VersionResponse {
    fn from(version: &PromptVersion) -> Self {
        let content_type = match version.content_type() {
            ContentType::Static => "static",
            ContentType::Template => "template",
        };

        Self {
            id: version.id().to_string(),
            version: version.version().to_string(),
            digest: version.digest().to_string(),
            content: version.content().to_string(),
            content_type: content_type.to_string(),
            variables: version.variables().map(|v| v.to_vec()),
            changelog: version.changelog().map(|s| s.to_string()),
            created_at: version.created_at(),
            average_rating: version.average_rating(),
            feedback_count: version.feedbacks().len(),
            feedback: version.feedbacks().iter().map(FeedbackResponse::from).collect(),
            improvement_suggestions: version.improvement_suggestions().iter().map(ImprovementSuggestionResponse::from).collect(),
        }
    }
}

impl From<&Tag> for TagResponse {
    fn from(tag: &Tag) -> Self {
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
            test_scenario: feedback.test_scenario().map(|ts| TestScenarioResponse {
                input: ts.input().to_string(),
                actual_output: ts.actual_output().to_string(),
                expected_output: ts.expected_output().map(|s| s.to_string()),
            }),
            created_at: feedback.created_at(),
        }
    }
}

#[derive(Serialize)]
pub struct ApiKeyResponse {
    pub id: String,
    pub name: String,
    pub key_prefix: String,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

impl From<&ApiKey> for ApiKeyResponse {
    fn from(api_key: &ApiKey) -> Self {
        Self {
            id: api_key.id().to_string(),
            name: api_key.name().to_string(),
            key_prefix: api_key.key_prefix().to_string(),
            last_used_at: api_key.last_used_at(),
            created_at: api_key.created_at(),
            is_active: api_key.is_active(),
        }
    }
}

#[derive(Serialize)]
pub struct ImprovementSuggestionResponse {
    pub id: String,
    pub suggested_content: String,
    pub ai_rationale: String,
    pub status: String,
    pub decline_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resulting_version_id: Option<String>,
}

impl From<&ImprovementSuggestion> for ImprovementSuggestionResponse {
    fn from(suggestion: &ImprovementSuggestion) -> Self {
        let status = match suggestion.status() {
            SuggestionStatus::Pending => "pending",
            SuggestionStatus::Accepted => "accepted",
            SuggestionStatus::Declined => "declined",
        };

        Self {
            id: suggestion.id().to_string(),
            suggested_content: suggestion.suggested_content().to_string(),
            ai_rationale: suggestion.ai_rationale().to_string(),
            status: status.to_string(),
            decline_reason: suggestion.decline_reason().map(|s| s.to_string()),
            created_at: suggestion.created_at(),
            resolved_at: suggestion.resolved_at(),
            resulting_version_id: suggestion.resulting_version_id().map(|id| id.to_string()),
        }
    }
}