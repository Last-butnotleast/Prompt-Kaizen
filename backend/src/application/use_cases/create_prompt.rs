use crate::application::PromptRepository;
use crate::domain::prompt::Prompt;
use std::sync::Arc;

pub struct CreatePromptUseCase {
    repository: Arc<dyn PromptRepository>,
}

impl CreatePromptUseCase {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<String, String> {
        let id = uuid::Uuid::new_v4().to_string();

        let prompt = Prompt::new(id.clone(), name, description);

        self.repository.save(&prompt).await?;

        Ok(id)
    }
}