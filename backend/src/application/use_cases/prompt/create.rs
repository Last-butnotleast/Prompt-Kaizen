use crate::application::PromptRepository;
use crate::domain::prompt::{Prompt, PromptType};
use std::sync::Arc;
use uuid::Uuid;

pub struct CreatePrompt {
    repository: Arc<dyn PromptRepository>,
}

impl CreatePrompt {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        name: String,
        description: Option<String>,
        prompt_type: PromptType,
    ) -> Result<Uuid, String> {
        let id = Uuid::new_v4();
        let prompt = Prompt::new(id, user_id, name, description, prompt_type);
        self.repository.save(&prompt).await?;
        Ok(id)
    }
}