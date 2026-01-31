use std::sync::Arc;
use uuid::Uuid;

use crate::application::api_key_repository::ApiKeyRepository;
use crate::domain::api_key::{ApiKey, generate_api_key, hash_api_key, get_key_prefix};

pub struct CreateApiKey {
    repository: Arc<dyn ApiKeyRepository>,
}

impl CreateApiKey {
    pub fn new(repository: Arc<dyn ApiKeyRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: Uuid, name: String) -> Result<(Uuid, String), String> {
        // Validate name
        if name.trim().is_empty() {
            return Err("API key name cannot be empty".to_string());
        }

        if name.len() > 255 {
            return Err("API key name too long (max 255 characters)".to_string());
        }

        // Generate the actual API key (this will be shown to user only once)
        let api_key_value = generate_api_key();

        // Hash it for storage
        let key_hash = hash_api_key(&api_key_value);

        // Get prefix for display
        let key_prefix = get_key_prefix(&api_key_value);

        // Create the API key entity
        let api_key = ApiKey::new(user_id, name, key_hash, key_prefix);

        let api_key_id = api_key.id();

        // Save to repository
        self.repository.create(&api_key).await?;

        // Return the ID and the PLAIN TEXT key (only time we return it!)
        Ok((api_key_id, api_key_value))
    }
}