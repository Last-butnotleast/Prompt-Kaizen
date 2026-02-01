use crate::application::PromptRepository;
use crate::domain::prompt::Version;
use std::sync::Arc;
use uuid::Uuid;

pub struct AcceptImprovementSuggestion {
    repository: Arc<dyn PromptRepository>,
}

impl AcceptImprovementSuggestion {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: Uuid,
        user_id: Uuid,
        source_version_id: Uuid,
        suggestion_id: Uuid,
        new_version: String,
        changelog: Option<String>,
    ) -> Result<Uuid, String> {
        let mut prompt = self.repository
            .find_by_id_and_user(prompt_id, user_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        let version_number = Version::from_str(&new_version)?;
        let new_version_id = Uuid::new_v4();

        prompt.accept_improvement(
            source_version_id,
            suggestion_id,
            new_version_id,
            version_number,
            changelog,
        )?;

        self.repository.save(&prompt).await?;
        Ok(new_version_id)
    }
}