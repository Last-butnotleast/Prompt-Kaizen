use crate::application::PromptRepository;
use std::sync::Arc;

pub struct DeleteFeedback {
    repository: Arc<dyn PromptRepository>,
}

impl DeleteFeedback {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: String,
        user_id: String,
        version_id: String,
        feedback_id: String,
    ) -> Result<(), String> {
        let mut prompt = self.repository
            .find_by_id_and_user(&prompt_id, &user_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        prompt.delete_feedback(&version_id, &feedback_id)?;
        self.repository.save(&prompt).await?;
        Ok(())
    }
}