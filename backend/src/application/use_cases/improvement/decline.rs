use crate::application::PromptRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeclineImprovementSuggestion {
    repository: Arc<dyn PromptRepository>,
}

impl DeclineImprovementSuggestion {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: Uuid,
        user_id: Uuid,
        version_id: Uuid,
        suggestion_id: Uuid,
        reason: String,
    ) -> Result<(), String> {
        let mut prompt = self.repository
            .find_by_id_and_user(prompt_id, user_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        let version = prompt.versions_mut()
            .iter_mut()
            .find(|v| v.id() == version_id)
            .ok_or_else(|| "Version not found".to_string())?;

        version.decline_suggestion(suggestion_id, reason)?;

        self.repository.save(&prompt).await?;
        Ok(())
    }
}