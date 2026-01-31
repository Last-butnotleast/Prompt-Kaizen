use crate::domain::prompt::Prompt;
use async_trait::async_trait;

#[async_trait]
pub trait PromptRepository: Send + Sync {
    async fn save(&self, prompt: &Prompt) -> Result<(), String>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Prompt>, String>;
    async fn find_by_id_and_user(&self, id: &str, user_id: &str) -> Result<Option<Prompt>, String>;
    async fn find_all(&self) -> Result<Vec<Prompt>, String>;
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<Prompt>, String>;
    async fn find_by_tag(&self, tag_name: &str) -> Result<Vec<Prompt>, String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
}