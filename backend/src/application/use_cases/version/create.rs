use crate::application::PromptRepository;
use std::sync::Arc;

pub struct CreateVersion {
    repository: Arc<dyn PromptRepository>,
}

impl CreateVersion {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: String,
        user_id: String,
        version: String,
        content: String,
        changelog: Option<String>,
    ) -> Result<String, String> {
        let mut prompt = self.repository
            .find_by_id_and_user(&prompt_id, &user_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        let version_id = uuid::Uuid::new_v4().to_string();
        prompt.add_version(version_id.clone(), version, content, changelog)?;
        self.repository.save(&prompt).await?;
        Ok(version_id)
    }
}