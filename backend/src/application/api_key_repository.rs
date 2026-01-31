use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::api_key::ApiKey;

#[async_trait]
pub trait ApiKeyRepository: Send + Sync {
    async fn create(&self, api_key: &ApiKey) -> Result<(), String>;
    async fn find_by_key_hash(&self, key_hash: &str) -> Result<Option<ApiKey>, String>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<ApiKey>, String>;
    async fn update_last_used(&self, id: Uuid) -> Result<(), String>;
    async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<(), String>;
    async fn deactivate(&self, id: Uuid, user_id: Uuid) -> Result<(), String>;
}