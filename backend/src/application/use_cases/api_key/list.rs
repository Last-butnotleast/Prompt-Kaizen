use std::sync::Arc;
use uuid::Uuid;

use crate::application::api_key_repository::ApiKeyRepository;
use crate::domain::api_key::ApiKey;

pub struct ListApiKeys {
    repository: Arc<dyn ApiKeyRepository>,
}

impl ListApiKeys {
    pub fn new(repository: Arc<dyn ApiKeyRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<Vec<ApiKey>, String> {
        self.repository.find_by_user_id(user_id).await
    }
}