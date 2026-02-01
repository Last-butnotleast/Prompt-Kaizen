use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest as Sha2Digest};
use uuid::Uuid;
use super::{Feedback, TestScenario, Version, ContentType, ImprovementSuggestion};

#[derive(Debug, Clone)]
pub struct PromptVersion {
    id: Uuid,
    prompt_id: Uuid,
    version: Version,
    digest: String,
    content: String,
    content_type: ContentType,
    variables: Option<Vec<String>>,
    changelog: Option<String>,
    created_at: DateTime<Utc>,
    feedbacks: Vec<Feedback>,
    improvement_suggestions: Vec<ImprovementSuggestion>,
}

impl PromptVersion {
    pub fn new(
        id: Uuid,
        prompt_id: Uuid,
        version: Version,
        content: String,
        content_type: ContentType,
        variables: Option<Vec<String>>,
        changelog: Option<String>,
    ) -> Self {
        let digest = Self::generate_digest(&content, content_type);
        Self {
            id,
            prompt_id,
            version,
            digest,
            content,
            content_type,
            variables,
            changelog,
            created_at: Utc::now(),
            feedbacks: Vec::new(),
            improvement_suggestions: Vec::new(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn prompt_id(&self) -> Uuid {
        self.prompt_id
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn version_string(&self) -> String {
        self.version.to_string()
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn content_type(&self) -> ContentType {
        self.content_type
    }

    pub fn variables(&self) -> Option<&[String]> {
        self.variables.as_deref()
    }

    pub fn changelog(&self) -> Option<&str> {
        self.changelog.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn feedbacks(&self) -> &[Feedback] {
        &self.feedbacks
    }

    pub fn render(&self, context: Option<&serde_json::Value>) -> Result<String, String> {
        match self.content_type {
            ContentType::Static => Ok(self.content.clone()),
            ContentType::Template => {
                let ctx = context.ok_or("Template requires context")?;

                let handlebars = handlebars::Handlebars::new();
                handlebars
                    .render_template(&self.content, ctx)
                    .map_err(|e| format!("Template rendering failed: {}", e))
            }
        }
    }

    pub fn add_feedback(
        &mut self,
        feedback_id: Uuid,
        rating: u8,
        comment: Option<String>,
        test_scenario: Option<TestScenario>,
    ) -> Result<&Feedback, String> {
        let feedback = Feedback::new(feedback_id, self.id, rating, comment, test_scenario)?;
        self.feedbacks.push(feedback);
        Ok(self.feedbacks.last().unwrap())
    }

    pub fn average_rating(&self) -> Option<f64> {
        if self.feedbacks.is_empty() {
            return None;
        }
        let sum: u32 = self.feedbacks.iter().map(|f| f.rating() as u32).sum();
        Some(sum as f64 / self.feedbacks.len() as f64)
    }

    fn generate_digest(content: &str, content_type: ContentType) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let type_suffix = match content_type {
            ContentType::Static => "static",
            ContentType::Template => "template",
        };
        hasher.update(type_suffix.as_bytes());
        let result = hasher.finalize();
        format!("sha256:{}", hex::encode(result))
    }

    pub fn delete_feedback(&mut self, feedback_id: Uuid) -> Result<(), String> {
        let initial_len = self.feedbacks.len();
        self.feedbacks.retain(|f| f.id() != feedback_id);

        if self.feedbacks.len() == initial_len {
            return Err("Feedback not found".to_string());
        }

        Ok(())
    }

    pub fn find_feedback(&self, feedback_id: Uuid) -> Option<&Feedback> {
        self.feedbacks.iter().find(|f| f.id() == feedback_id)
    }

    pub(crate) fn update_feedback(
        &mut self,
        feedback_id: Uuid,
        rating: Option<u8>,
        comment: Option<Option<String>>,
    ) -> Result<(), String> {
        let feedback = self.feedbacks.iter_mut()
            .find(|f| f.id() == feedback_id)
            .ok_or("Feedback not found")?;

        if let Some(r) = rating {
            feedback.update_rating(r)?;
        }
        if let Some(c) = comment {
            feedback.update_comment(c);
        }
        Ok(())
    }

    pub fn extract_variables(&self) -> Result<Vec<String>, String> {
        if self.content_type != ContentType::Template {
            return Ok(Vec::new());
        }

        let handlebars = handlebars::Handlebars::new();
        let template = handlebars
            .render_template(&self.content, &serde_json::json!({}))
            .err()
            .and_then(|e| {
                let error_msg = e.to_string();
                if error_msg.contains("variable") {
                    Some(error_msg)
                } else {
                    None
                }
            });

        let mut vars = std::collections::HashSet::new();
        let re = regex::Regex::new(r"\{\{([^}]+)\}\}").unwrap();

        for cap in re.captures_iter(&self.content) {
            if let Some(var) = cap.get(1) {
                let var_name = var.as_str().trim();
                if !var_name.starts_with('#') && !var_name.starts_with('/') {
                    vars.insert(var_name.to_string());
                }
            }
        }

        Ok(vars.into_iter().collect())
    }

    pub fn improvement_suggestions(&self) -> &[ImprovementSuggestion] {
        &self.improvement_suggestions
    }

    pub fn create_improvement_suggestion(
        &mut self,
        suggestion_id: Uuid,
        suggested_content: String,
        ai_rationale: String,
    ) -> Result<&ImprovementSuggestion, String> {
        let suggestion = ImprovementSuggestion::new(
            suggestion_id,
            self.id,
            suggested_content,
            ai_rationale,
        );
        self.improvement_suggestions.push(suggestion);
        Ok(self.improvement_suggestions.last().unwrap())
    }

    pub fn accept_suggestion(
        &mut self,
        suggestion_id: Uuid,
        resulting_version_id: Uuid,
    ) -> Result<(), String> {
        let suggestion = self.improvement_suggestions.iter_mut()
            .find(|s| s.id() == suggestion_id)
            .ok_or("Suggestion not found")?;

        suggestion.accept(resulting_version_id)
    }

    pub fn decline_suggestion(
        &mut self,
        suggestion_id: Uuid,
        reason: String,
    ) -> Result<(), String> {
        let suggestion = self.improvement_suggestions.iter_mut()
            .find(|s| s.id() == suggestion_id)
            .ok_or("Suggestion not found")?;

        suggestion.decline(reason)
    }

    pub fn find_suggestion(&self, suggestion_id: Uuid) -> Option<&ImprovementSuggestion> {
        self.improvement_suggestions.iter().find(|s| s.id() == suggestion_id)
    }

    pub fn improvement_suggestions_mut(&mut self) -> &mut Vec<ImprovementSuggestion> {
        &mut self.improvement_suggestions
    }
}