use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest as Sha2Digest};
use uuid::Uuid;
use super::{Feedback, Version};

#[derive(Debug, Clone)]
pub struct PromptVersion {
    id: Uuid,
    prompt_id: Uuid,
    version: Version,
    digest: String,
    content: String,
    changelog: Option<String>,
    created_at: DateTime<Utc>,
    feedbacks: Vec<Feedback>,
}

impl PromptVersion {
    pub fn new(
        id: Uuid,
        prompt_id: Uuid,
        version: Version,
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

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn prompt_id(&self) -> Uuid {
        self.prompt_id
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn version_string(&self) -> String {
        self.version.to_string()
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
        feedback_id: Uuid,
        rating: u8,
        comment: Option<String>,
    ) -> Result<&Feedback, String> {
        let feedback = Feedback::new(feedback_id, self.id, rating, comment)?;
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

    pub fn delete_feedback(&mut self, feedback_id: Uuid) -> Result<(), String> {
        let initial_len = self.feedbacks.len();
        self.feedbacks.retain(|f| f.id() != feedback_id);

        if self.feedbacks.len() == initial_len {
            return Err("Feedback not found".to_string());
        }

        Ok(())
    }

    pub fn find_feedback(&self, feedback_id: Uuid) -> Option<&Feedback> {
        self.feedbacks.iter().find(|f| f.id() == feedback_id)
    }

    pub(crate) fn update_feedback(
        &mut self,
        feedback_id: Uuid,
        rating: Option<u8>,
        comment: Option<Option<String>>,
    ) -> Result<(), String> {
        let feedback = self.feedbacks.iter_mut()
            .find(|f| f.id() == feedback_id)
            .ok_or("Feedback not found")?;

        if let Some(r) = rating {
            feedback.update_rating(r)?;
        }
        if let Some(c) = comment {
            feedback.update_comment(c);
        }
        Ok(())
    }
}