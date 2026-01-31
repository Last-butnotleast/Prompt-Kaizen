use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest as Sha2Digest};
use super::Feedback;

#[derive(Debug, Clone)]
pub struct PromptVersion {
    id: String,
    prompt_id: String,
    version: String,
    digest: String,
    content: String,
    changelog: Option<String>,
    created_at: DateTime<Utc>,
    feedbacks: Vec<Feedback>,
}

impl PromptVersion {
    pub fn new(
        id: String,
        prompt_id: String,
        version: String,
        content: String,
        changelog: Option<String>,
    ) -> Self {
        let digest = Self::generate_digest(&content);
        Self {
            id,
            prompt_id,
            version,
            digest,
            content,
            changelog,
            created_at: Utc::now(),
            feedbacks: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn prompt_id(&self) -> &str {
        &self.prompt_id
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn changelog(&self) -> Option<&str> {
        self.changelog.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn feedbacks(&self) -> &[Feedback] {
        &self.feedbacks
    }

    pub fn add_feedback(
        &mut self,
        feedback_id: String,
        rating: u8,
        comment: Option<String>,
    ) -> Result<&Feedback, String> {
        let feedback = Feedback::new(feedback_id, self.id.clone(), rating, comment)?;
        self.feedbacks.push(feedback);
        Ok(self.feedbacks.last().unwrap())
    }

    pub fn average_rating(&self) -> Option<f64> {
        if self.feedbacks.is_empty() {
            return None;
        }
        let sum: u32 = self.feedbacks.iter().map(|f| f.rating() as u32).sum();
        Some(sum as f64 / self.feedbacks.len() as f64)
    }

    fn generate_digest(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();
        format!("sha256:{}", hex::encode(result))
    }
}