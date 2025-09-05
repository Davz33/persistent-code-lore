use serde::{Deserialize, Serialize};

// Re-export our main modules so users can easily access everything they need
pub mod config;
pub mod extractor;
pub mod generator;

// Make the main types available at the crate root for convenience
pub use config::Config;
pub use extractor::ChatExtractor;
pub use generator::MarkdownGenerator;

/// Represents a single chat session from the database.
/// This contains all the metadata about a conversation that happened
/// in the chat application, like when it was created and what mode it used.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSession {
    /// The type of session (usually "head" for main sessions)
    #[serde(rename = "type")]
    pub session_type: String,
    /// Unique identifier for this session
    pub composer_id: String,
    /// Human-readable name for the session
    pub name: String,
    /// When this session was last updated (Unix timestamp in milliseconds)
    pub last_updated_at: i64,
    /// When this session was created (Unix timestamp in milliseconds)
    pub created_at: i64,
    /// What mode the session was in (usually "agent")
    pub unified_mode: String,
    /// What force mode was used (usually "edit")
    pub force_mode: String,
    /// Whether there are unread messages in this session
    pub has_unread_messages: bool,
}

/// Represents a single AI generation from the database.
/// This contains information about what the AI generated during a conversation,
/// including the text and when it was created.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGeneration {
    /// When this generation was created (Unix timestamp in milliseconds)
    pub unix_ms: i64,
    /// Unique identifier for this generation
    pub generation_uuid: String,
    /// The type of generation (usually "composer")
    pub r#type: String,
    /// The actual text that was generated
    pub text_description: String,
}

/// Represents a user prompt from the database.
/// This contains the text that the user typed to start or continue a conversation.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatPrompt {
    /// The actual prompt text that the user entered
    pub text: String,
    /// The type of command this prompt represents
    pub command_type: i32,
}

/// Container for all the chat sessions from the database.
/// This is what we get when we extract the composer data - it contains
/// a list of all the chat sessions that were found.
#[derive(Debug, Serialize, Deserialize)]
pub struct ComposerData {
    /// All the chat sessions that were found in the database
    pub all_composers: Vec<ChatSession>,
}
