use crate::application::PromptRepository;
use crate::domain::prompt::Prompt;
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
    ) -> Result<Uuid, String> {
        let id = Uuid::new_v4();
        let prompt = Prompt::new(id, user_id, name, description);
        self.repository.save(&prompt).await?;
        Ok(id)
    }
}