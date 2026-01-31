use axum::http::{HeaderMap, StatusCode};

pub fn extract_user_id(headers: &HeaderMap) -> Result<String, (StatusCode, String)> {
    headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing x-user-id header".to_string()))
}