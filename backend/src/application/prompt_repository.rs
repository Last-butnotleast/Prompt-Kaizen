use crate::domain::prompt::{Prompt, PromptVersion, Tag, Feedback};
use async_trait::async_trait;

#[async_trait]
pub trait PromptRepository: Send + Sync {
    async fn save_prompt(&self, prompt: &Prompt) -> Result<(), String>;
    async fn find_prompt_by_id(&self, id: &str) -> Result<Option<Prompt>, String>;
    async fn find_all_prompts(&self) -> Result<Vec<Prompt>, String>;

    async fn save_version(&self, version: &PromptVersion) -> Result<(), String>;
    async fn find_versions_by_prompt_id(&self, prompt_id: &str) -> Result<Vec<PromptVersion>, String>;
    async fn find_version_by_digest(&self, digest: &str) -> Result<Option<PromptVersion>, String>;

    async fn save_tag(&self, tag: &Tag) -> Result<(), String>;
    async fn find_tags_by_prompt_id(&self, prompt_id: &str) -> Result<Vec<Tag>, String>;
    async fn find_tag(&self, prompt_id: &str, tag_name: &str) -> Result<Option<Tag>, String>;

    async fn save_feedback(&self, feedback: &Feedback) -> Result<(), String>;
    async fn find_feedback_by_version_id(&self, version_id: &str) -> Result<Vec<Feedback>, String>;
}