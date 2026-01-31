use crate::application::PromptRepository;
use std::sync::Arc;

pub struct SubmitFeedbackUseCase {
    repository: Arc<dyn PromptRepository>,
}

impl SubmitFeedbackUseCase {
    pub fn new(repository: Arc<dyn PromptRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        prompt_id: String,
        version_id: String,
        rating: u8,
        comment: Option<String>,
    ) -> Result<String, String> {
        let mut prompt = self.repository
            .find_by_id(&prompt_id)
            .await?
            .ok_or_else(|| "Prompt not found".to_string())?;

        let version = prompt.versions_mut()
            .iter_mut()
            .find(|v| v.id() == version_id)
            .ok_or_else(|| "Version not found".to_string())?;

        let feedback_id = uuid::Uuid::new_v4().to_string();

        version.add_feedback(feedback_id.clone(), rating, comment)?;

        self.repository.save(&prompt).await?;

        Ok(feedback_id)
    }
}