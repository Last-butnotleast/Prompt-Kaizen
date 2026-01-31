use crate::application::PromptRepository;
use std::sync::Arc;

pub struct DeletePrompt {
    repository: Arc<dyn PromptRepository>,
}

impl DeletePrompt {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, prompt_id: String) -> Result<(), String> {
        self.repository.delete(&prompt_id).await
    }
}