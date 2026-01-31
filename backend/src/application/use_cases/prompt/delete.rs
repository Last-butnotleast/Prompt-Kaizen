use crate::application::PromptRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeletePrompt {
    repository: Arc<dyn PromptRepository>,
}

impl DeletePrompt {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, prompt_id: Uuid, user_id: Uuid) -> Result<(), String> {
        self.repository
            .find_by_id_and_user(prompt_id, user_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        self.repository.delete(prompt_id).await
    }
}