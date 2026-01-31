use axum::http::{HeaderMap, StatusCode};
use uuid::Uuid;
use std::sync::Arc;

use crate::application::api_key_repository::ApiKeyRepository;
use crate::domain::api_key::hash_api_key;

pub fn extract_user_id(headers: &HeaderMap) -> Result<Uuid, (StatusCode, String)> {
    let user_id_str = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            (StatusCode::UNAUTHORIZED, "Missing x-user-id header".to_string())
        })?;

    // Parse the user_id string as UUID
    Uuid::parse_str(user_id_str).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            format!("Invalid user ID format - must be a valid UUID: {}", e),
        )
    })
}

/// Extract user_id from either x-user-id header (for authenticated frontend users)
/// or from x-api-key header (for API key authentication from n8n, etc.)
pub async fn extract_user_id_with_api_key(
    headers: &HeaderMap,
    api_key_repo: Arc<dyn ApiKeyRepository>,
) -> Result<Uuid, (StatusCode, String)> {
    // First, try x-user-id header (frontend with Supabase auth)
    if let Some(user_id) = headers.get("x-user-id") {
        let user_id_str = user_id.to_str()
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid x-user-id header".to_string()))?;

        return Uuid::parse_str(user_id_str)
            .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid user ID format: {}", e)));
    }

    // Try API key authentication
    if let Some(api_key_header) = headers.get("x-api-key") {
        let api_key = api_key_header.to_str()
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid x-api-key header".to_string()))?;

        // Hash the provided API key
        let key_hash = hash_api_key(api_key);

        // Look up the API key in the database
        let api_key_entity = api_key_repo
            .find_by_key_hash(&key_hash)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Invalid API key".to_string()))?;

        // Update last used timestamp (fire and forget, don't block on this)
        let repo_clone = api_key_repo.clone();
        let key_id = api_key_entity.id();
        tokio::spawn(async move {
            let _ = repo_clone.update_last_used(key_id).await;
        });

        return Ok(api_key_entity.user_id());
    }

    // No authentication provided
    Err((StatusCode::UNAUTHORIZED, "Missing authentication: provide either x-user-id or x-api-key header".to_string()))
}