use axum::http::StatusCode;
use uuid::Uuid;

/// Parse a UUID from a string, returning a proper HTTP error on failure
pub fn parse_uuid(id: &str, field_name: &str) -> Result<Uuid, (StatusCode, String)> {
    Uuid::parse_str(id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid UUID format for {}", field_name),
        )
    })
}

/// Parse multiple UUIDs, short-circuiting on first error
pub fn parse_uuids(
    ids: &[(&str, &str)], // (value, field_name) pairs
) -> Result<Vec<Uuid>, (StatusCode, String)> {
    ids.iter()
        .map(|(id, name)| parse_uuid(id, name))
        .collect()
}