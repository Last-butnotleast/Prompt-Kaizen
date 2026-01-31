use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{Utc};

use crate::application::api_key_repository::ApiKeyRepository;
use crate::domain::api_key::ApiKey;

pub struct PostgresApiKeyRepository {
    pool: PgPool,
}

impl PostgresApiKeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ApiKeyRepository for PostgresApiKeyRepository {
    async fn create(&self, api_key: &ApiKey) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO api_keys (id, user_id, name, key_hash, key_prefix, created_at, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
            .bind(api_key.id())
            .bind(api_key.user_id())
            .bind(api_key.name())
            .bind(api_key.key_hash())
            .bind(api_key.key_prefix())
            .bind(api_key.created_at())
            .bind(api_key.is_active())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to create API key: {}", e))?;

        Ok(())
    }

    async fn find_by_key_hash(&self, key_hash: &str) -> Result<Option<ApiKey>, String> {
        let record = sqlx::query(
            "SELECT id, user_id, name, key_hash, key_prefix, last_used_at, created_at, is_active
            FROM api_keys
            WHERE key_hash = $1 AND is_active = true"
        )
            .bind(key_hash)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Failed to find API key: {}", e))?;

        Ok(record.map(|r| {
            ApiKey::from_repository(
                r.get("id"),
                r.get("user_id"),
                r.get("name"),
                r.get("key_hash"),
                r.get("key_prefix"),
                r.get("last_used_at"),
                r.get("created_at"),
                r.get("is_active"),
            )
        }))
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<ApiKey>, String> {
        let records = sqlx::query(
            "SELECT id, user_id, name, key_hash, key_prefix, last_used_at, created_at, is_active
            FROM api_keys
            WHERE user_id = $1
            ORDER BY created_at DESC"
        )
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to list API keys: {}", e))?;

        Ok(records
            .into_iter()
            .map(|r| {
                ApiKey::from_repository(
                    r.get("id"),
                    r.get("user_id"),
                    r.get("name"),
                    r.get("key_hash"),
                    r.get("key_prefix"),
                    r.get("last_used_at"),
                    r.get("created_at"),
                    r.get("is_active"),
                )
            })
            .collect())
    }

    async fn update_last_used(&self, id: Uuid) -> Result<(), String> {
        sqlx::query(
            "UPDATE api_keys
            SET last_used_at = $1
            WHERE id = $2"
        )
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to update API key last used: {}", e))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<(), String> {
        let result = sqlx::query(
            "DELETE FROM api_keys
            WHERE id = $1 AND user_id = $2"
        )
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete API key: {}", e))?;

        if result.rows_affected() == 0 {
            return Err("API key not found or unauthorized".to_string());
        }

        Ok(())
    }

    async fn deactivate(&self, id: Uuid, user_id: Uuid) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE api_keys
            SET is_active = false
            WHERE id = $1 AND user_id = $2"
        )
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to deactivate API key: {}", e))?;

        if result.rows_affected() == 0 {
            return Err("API key not found or unauthorized".to_string());
        }

        Ok(())
    }
}