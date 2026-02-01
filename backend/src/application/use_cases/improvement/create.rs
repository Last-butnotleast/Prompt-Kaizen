use crate::application::PromptRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateImprovementSuggestion {
    repository: Arc<dyn PromptRepository>,
}

impl CreateImprovementSuggestion {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: Uuid,
        user_id: Uuid,
        version_id: Uuid,
        suggested_content: String,
        ai_rationale: String,
    ) -> Result<Uuid, String> {
        let mut prompt = self.repository
            .find_by_id_and_user(prompt_id, user_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        let version = prompt.versions_mut()
            .iter_mut()
            .find(|v| v.id() == version_id)
            .ok_or_else(|| "Version not found".to_string())?;

        let suggestion_id = Uuid::new_v4();
        version.create_improvement_suggestion(suggestion_id, suggested_content, ai_rationale)?;

        self.repository.save(&prompt).await?;
        Ok(suggestion_id)
    }
}