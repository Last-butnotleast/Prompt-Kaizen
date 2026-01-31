use axum::http::{HeaderMap, StatusCode};
use uuid::Uuid;

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