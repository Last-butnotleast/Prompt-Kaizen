use crate::application::PromptRepository;
use crate::domain::prompt::Prompt;
use std::sync::Arc;

pub struct GetPrompt {
    repository: Arc<dyn PromptRepository>,
}

impl GetPrompt {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, prompt_id: String) -> Result<Prompt, String> {
        self.repository
            .find_by_id(&prompt_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())
    }
}