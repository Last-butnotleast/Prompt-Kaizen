use crate::application::PromptRepository;
use std::sync::Arc;

pub struct UpdateFeedback {
    repository: Arc<dyn PromptRepository>,
}

impl UpdateFeedback {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: String,
        user_id: String,
        version_id: String,
        feedback_id: String,
        rating: Option<u8>,
        comment: Option<Option<String>>,
    ) -> Result<(), String> {
        let mut prompt = self.repository
            .find_by_id_and_user(&prompt_id, &user_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        prompt.update_feedback(&version_id, &feedback_id, rating, comment)?;
        self.repository.save(&prompt).await?;
        Ok(())
    }
}