use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ApiKey {
    id: Uuid,
    user_id: Uuid,
    name: String,
    key_hash: String,
    key_prefix: String,
    last_used_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    is_active: bool,
}

impl ApiKey {
    pub fn new(
        user_id: Uuid,
        name: String,
        key_hash: String,
        key_prefix: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            key_hash,
            key_prefix,
            last_used_at: None,
            created_at: Utc::now(),
            is_active: true,
        }
    }

    pub fn from_repository(
        id: Uuid,
        user_id: Uuid,
        name: String,
        key_hash: String,
        key_prefix: String,
        last_used_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        is_active: bool,
    ) -> Self {
        Self {
            id,
            user_id,
            name,
            key_hash,
            key_prefix,
            last_used_at,
            created_at,
            is_active,
        }
    }

    // Getters
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn key_hash(&self) -> &str {
        &self.key_hash
    }

    pub fn key_prefix(&self) -> &str {
        &self.key_prefix
    }

    pub fn last_used_at(&self) -> Option<DateTime<Utc>> {
        self.last_used_at
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    // Methods
    pub fn mark_as_used(&mut self) {
        self.last_used_at = Some(Utc::now());
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }
}

// Utility function to generate API key
pub fn generate_api_key() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    const KEY_LENGTH: usize = 32;

    let mut rng = rand::thread_rng();
    let key: String = (0..KEY_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    format!("sk_live_{}", key)
}

// Utility function to hash API key
pub fn hash_api_key(key: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())  // Use hex::encode instead of format!
}

// Utility function to get key prefix for display
pub fn get_key_prefix(key: &str) -> String {
    if key.len() > 20 {
        format!("{}...", &key[..20])
    } else {
        key.to_string()
    }
}