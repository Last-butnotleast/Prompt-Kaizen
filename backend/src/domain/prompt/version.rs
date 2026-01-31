use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest as Sha2Digest};

#[derive(Debug, Clone)]
pub struct PromptVersion {
    pub id: String,
    pub prompt_id: String,
    pub version: String,
    pub digest: String,
    pub content: String,
    pub changelog: Option<String>,
    pub created_at: DateTime<Utc>,
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
        }
    }

    fn generate_digest(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();
        format!("sha256:{}", hex::encode(result))
    }
}