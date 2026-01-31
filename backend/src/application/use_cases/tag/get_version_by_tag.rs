use crate::application::PromptRepository;
use crate::domain::prompt::PromptVersion;
use std::sync::Arc;

pub struct GetVersionByTag {
    repository: Arc<dyn PromptRepository>,
}

impl GetVersionByTag {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: String,
        tag_name: String,
    ) -> Result<PromptVersion, String> {
        let prompt = self.repository
            .find_by_id(&prompt_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        let tag = prompt
            .find_tag(&tag_name)
            .ok_or_else(|| "Tag not found".to_string())?;

        prompt
            .find_version_by_id(tag.version_id())
            .cloned()
            .ok_or_else(|| "Version not found".to_string())
    }
}