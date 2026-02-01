pub mod create;
pub mod accept;
pub mod decline;
pub mod get_for_version;

pub use create::CreateImprovementSuggestion;
pub use accept::AcceptImprovementSuggestion;
pub use decline::DeclineImprovementSuggestion;
pub use get_for_version::GetSuggestionsForVersion;