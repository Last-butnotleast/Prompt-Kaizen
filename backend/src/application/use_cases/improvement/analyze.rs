use crate::application::{PromptRepository, AIService};
use std::sync::Arc;
use uuid::Uuid;

pub struct AnalyzeFeedbackAndSuggest {
    repository: Arc<dyn PromptRepository>,
    ai_service: Arc<dyn AIService>,
}

impl AnalyzeFeedbackAndSuggest {
    pub fn new(repository: Arc<dyn PromptRepository>, ai_service: Arc<dyn AIService>) -> Self {
        Self { repository, ai_service }
    }

    pub async fn execute(
        &self,
        prompt_id: Uuid,
        user_id: Uuid,
        version_id: Uuid,
    ) -> Result<Uuid, String> {
        let mut prompt = self.repository
            .find_by_id_and_user(prompt_id, user_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        let version = prompt.versions_mut()
            .iter_mut()
            .find(|v| v.id() == version_id)
            .ok_or_else(|| "Version not found".to_string())?;

        if version.feedbacks().is_empty() {
            return Err("No feedback available to analyze".to_string());
        }

        let (suggested_content, ai_rationale) = self.ai_service
            .analyze_feedback_and_suggest(
                version.content(),
                version.content_type(),
                version.feedbacks(),
            )
            .await?;

        let suggestion_id = Uuid::new_v4();
        version.create_improvement_suggestion(suggestion_id, suggested_content, ai_rationale)?;

        self.repository.save(&prompt).await?;
        Ok(suggestion_id)
    }
}