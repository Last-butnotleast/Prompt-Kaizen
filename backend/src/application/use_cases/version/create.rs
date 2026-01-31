use crate::application::PromptRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateVersion {
    repository: Arc<dyn PromptRepository>,
}

impl CreateVersion {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: Uuid,
        user_id: Uuid,
        version: String,
        content: String,
        changelog: Option<String>,
    ) -> Result<Uuid, String> {
        let mut prompt = self.repository
            .find_by_id_and_user(prompt_id, user_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        let version_id = Uuid::new_v4();
        prompt.add_version(version_id, version, content, changelog)?;
        self.repository.save(&prompt).await?;
        Ok(version_id)
    }
}