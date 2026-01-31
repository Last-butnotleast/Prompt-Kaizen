use crate::application::PromptRepository;
use std::sync::Arc;

pub struct UpdatePrompt {
    repository: Arc<dyn PromptRepository>,
}

impl UpdatePrompt {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: String,
        user_id: String,
        name: Option<String>,
        description: Option<Option<String>>,
    ) -> Result<(), String> {
        let mut prompt = self.repository
            .find_by_id_and_user(&prompt_id, &user_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        if let Some(n) = name {
            prompt.update_name(n)?;
        }

        if let Some(d) = description {
            prompt.update_description(d);
        }

        self.repository.save(&prompt).await?;
        Ok(())
    }
}