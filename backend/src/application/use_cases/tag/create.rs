use crate::application::PromptRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateTag {
    repository: Arc<dyn PromptRepository>,
}

impl CreateTag {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: Uuid,
        user_id: Uuid,
        tag_name: String,
        version_id: Uuid,
    ) -> Result<(), String> {
        let mut prompt = self.repository
            .find_by_id_and_user(prompt_id, user_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        let tag_id = Uuid::new_v4();
        prompt.tag_version(tag_id, tag_name, version_id)?;
        self.repository.save(&prompt).await?;
        Ok(())
    }
}