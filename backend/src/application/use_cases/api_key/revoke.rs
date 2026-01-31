use std::sync::Arc;
use uuid::Uuid;

use crate::application::api_key_repository::ApiKeyRepository;

pub struct RevokeApiKey {
    repository: Arc<dyn ApiKeyRepository>,
}

impl RevokeApiKey {
    pub fn new(repository: Arc<dyn ApiKeyRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, api_key_id: Uuid, user_id: Uuid) -> Result<(), String> {
        self.repository.deactivate(api_key_id, user_id).await
    }
}