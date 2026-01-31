use axum::http::{HeaderMap, StatusCode};
use uuid::Uuid;
use std::sync::Arc;
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;
use tokio::sync::RwLock;

use crate::application::api_key_repository::ApiKeyRepository;
use crate::domain::api_key::hash_api_key;

#[derive(Debug, Serialize, Deserialize)]
struct SupabaseClaims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
struct Jwk {
    kid: String,
    kty: String,
    alg: String,
    #[serde(default)]
    n: Option<String>,
    #[serde(default)]
    e: Option<String>,
    #[serde(default)]
    crv: Option<String>,
    #[serde(default)]
    x: Option<String>,
    #[serde(default)]
    y: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<Jwk>,
}

static JWKS_CACHE: OnceLock<RwLock<HashMap<String, DecodingKey>>> = OnceLock::new();

fn get_cache() -> &'static RwLock<HashMap<String, DecodingKey>> {
    JWKS_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

async fn get_jwks() -> Result<HashMap<String, DecodingKey>, String> {
    let supabase_url = std::env::var("SUPABASE_URL")
        .map_err(|_| "SUPABASE_URL not configured".to_string())?;

    let jwks_url = format!("{}/auth/v1/.well-known/jwks.json", supabase_url);

    let response = reqwest::get(&jwks_url)
        .await
        .map_err(|e| format!("Failed to fetch JWKS: {}", e))?;

    let status = response.status();

    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("JWKS endpoint returned {}: {}", status, body));
    }

    let body_text = response.text().await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    let jwks: JwksResponse = serde_json::from_str(&body_text)
        .map_err(|e| format!("Failed to parse JWKS: {}", e))?;

    if jwks.keys.is_empty() {
        return Err("No keys found in JWKS".to_string());
    }

    let mut keys = HashMap::new();
    for jwk in jwks.keys {
        let key = match jwk.kty.as_str() {
            "RSA" => {
                if let (Some(n), Some(e)) = (jwk.n, jwk.e) {
                    DecodingKey::from_rsa_components(&n, &e).ok()
                } else {
                    None
                }
            }
            "EC" => {
                if let (Some(x), Some(y)) = (jwk.x, jwk.y) {
                    DecodingKey::from_ec_components(&x, &y).ok()
                } else {
                    None
                }
            }
            _ => None
        };

        if let Some(k) = key {
            keys.insert(jwk.kid, k);
        }
    }

    if keys.is_empty() {
        return Err("Failed to parse any keys from JWKS".to_string());
    }

    Ok(keys)
}

async fn get_decoding_key(kid: &str) -> Result<DecodingKey, String> {
    {
        let cache = get_cache().read().await;
        if let Some(key) = cache.get(kid) {
            return Ok(key.clone());
        }
    }

    let keys = get_jwks().await?;
    let key = keys.get(kid)
        .ok_or_else(|| "Key ID not found in JWKS".to_string())?
        .clone();

    {
        let mut cache = get_cache().write().await;
        *cache = keys;
    }

    Ok(key)
}

async fn extract_user_id_from_jwt(headers: &HeaderMap) -> Result<Uuid, (StatusCode, String)> {
    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Invalid authorization format".to_string()))?;

    let header = decode_header(token)
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid JWT header: {}", e)))?;

    let kid = header.kid
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing kid in JWT header".to_string()))?;

    let key = get_decoding_key(&kid).await
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Failed to get decoding key: {}", e)))?;

    let alg = header.alg;
    let mut validation = Validation::new(alg);
    validation.validate_exp = true;
    validation.set_audience(&["authenticated"]);

    let token_data = decode::<SupabaseClaims>(token, &key, &validation)
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid JWT: {}", e)))?;

    Uuid::parse_str(&token_data.claims.sub)
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid user ID in token: {}", e)))
}

pub async fn extract_user_id(headers: &HeaderMap) -> Result<Uuid, (StatusCode, String)> {
    extract_user_id_from_jwt(headers).await
}

pub async fn extract_user_id_with_api_key(
    headers: &HeaderMap,
    api_key_repo: Arc<dyn ApiKeyRepository>,
) -> Result<Uuid, (StatusCode, String)> {
    if headers.contains_key("x-api-key") {
        let api_key = headers
            .get("x-api-key")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Invalid x-api-key header".to_string()))?;

        let key_hash = hash_api_key(api_key);
        let api_key_entity = api_key_repo
            .find_by_key_hash(&key_hash)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Invalid API key".to_string()))?;

        let repo_clone = api_key_repo.clone();
        let key_id = api_key_entity.id();
        tokio::spawn(async move {
            let _ = repo_clone.update_last_used(key_id).await;
        });

        return Ok(api_key_entity.user_id());
    }

    extract_user_id_from_jwt(headers).await
}