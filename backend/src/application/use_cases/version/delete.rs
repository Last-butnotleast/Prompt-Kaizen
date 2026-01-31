use crate::application::PromptRepository;
use std::sync::Arc;

pub struct DeleteVersion {
    repository: Arc<dyn PromptRepository>,
}

impl DeleteVersion {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: String,
        version_id: String,
    ) -> Result<(), String> {
        let mut prompt = self.repository
            .find_by_id(&prompt_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        prompt.delete_version(&version_id)?;
        self.repository.save(&prompt).await?;
        Ok(())
    }
}