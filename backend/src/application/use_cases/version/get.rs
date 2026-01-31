use crate::application::PromptRepository;
use crate::domain::prompt::PromptVersion;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetVersion {
    repository: Arc<dyn PromptRepository>,
}

impl GetVersion {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: Uuid,
        user_id: Uuid,
        version_id: Uuid,
    ) -> Result<PromptVersion, String> {
        let prompt = self.repository
            .find_by_id_and_user(prompt_id, user_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        prompt
            .find_version_by_id(version_id)
            .cloned()
            .ok_or_else(|| "Version not found".to_string())
    }
}