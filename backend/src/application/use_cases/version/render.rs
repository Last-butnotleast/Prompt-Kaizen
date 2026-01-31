use crate::application::PromptRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct RenderVersion {
    repository: Arc<dyn PromptRepository>,
}

impl RenderVersion {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: Uuid,
        user_id: Uuid,
        version_id: Uuid,
        context: Option<serde_json::Value>,
    ) -> Result<String, String> {
        let prompt = self.repository
            .find_by_id_and_user(prompt_id, user_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        let version = prompt
            .find_version_by_id(version_id)
            .ok_or_else(|| "Version not found".to_string())?;

        version.render(context.as_ref())
    }
}