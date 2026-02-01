use crate::domain::prompt::{Feedback, ContentType};
use async_trait::async_trait;

#[async_trait]
pub trait AIService: Send + Sync {
    async fn analyze_feedback_and_suggest(
        &self,
        prompt_content: &str,
        content_type: ContentType,
        feedbacks: &[Feedback],
    ) -> Result<(String, String), String>;
}