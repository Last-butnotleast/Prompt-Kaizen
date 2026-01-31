use crate::application::PromptRepository;
use crate::domain::prompt::Prompt;
use std::sync::Arc;

pub struct ListPrompts {
    repository: Arc<dyn PromptRepository>,
}

impl ListPrompts {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: String) -> Result<Vec<Prompt>, String> {
        self.repository.find_by_user(&user_id).await
    }
}