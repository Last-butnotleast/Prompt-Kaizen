use crate::application::PromptRepository;
use std::sync::Arc;

pub struct ManageTagsUseCase {
    repository: Arc<dyn PromptRepository>,
}

impl ManageTagsUseCase {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn tag_version(
        &self,
        prompt_id: String,
        tag_name: String,
        version_id: String,
    ) -> Result<(), String> {
        let mut prompt = self.repository
            .find_by_id(&prompt_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        let tag_id = uuid::Uuid::new_v4().to_string();

        prompt.tag_version(tag_id, tag_name, &version_id)?;

        self.repository.save(&prompt).await?;

        Ok(())
    }
}