use crate::application::PromptRepository;
use crate::domain::prompt::Prompt;
use std::sync::Arc;

pub struct CreatePrompt {
    repository: Arc<dyn PromptRepository>,
}

impl CreatePrompt {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        user_id: String,
        name: String,
        description: Option<String>,
    ) -> Result<String, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let prompt = Prompt::new(id.clone(), user_id, name, description);
        self.repository.save(&prompt).await?;
        Ok(id)
    }
}