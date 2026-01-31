pub mod create;
pub mod update;
pub mod get;
pub mod list;
pub mod delete;

pub use create::CreatePrompt;
pub use update::UpdatePrompt;
pub use get::GetPrompt;
pub use list::ListPrompts;
pub use delete::DeletePrompt;