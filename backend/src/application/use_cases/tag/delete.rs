use crate::application::PromptRepository;
use std::sync::Arc;

pub struct DeleteTag {
    repository: Arc<dyn PromptRepository>,
}

impl DeleteTag {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: String,
        tag_name: String,
    ) -> Result<(), String> {
        let mut prompt = self.repository
            .find_by_id(&prompt_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        prompt.remove_tag(&tag_name)?;
        self.repository.save(&prompt).await?;
        Ok(())
    }
}