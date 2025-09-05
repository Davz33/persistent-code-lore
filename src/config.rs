use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

/// Configuration structure that holds all the settings for our persistent code lore tool.
/// This is where we store everything from database paths to privacy settings.
/// Think of it as the "brain" that tells our application how to behave.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// What we call ourselves - the name of our application
    pub app_name: String,
    /// Where to put the generated markdown files
    pub output_dir: String,
    /// What to name the main output file
    pub output_filename: String,
    /// What type of database we're connecting to (currently just SQLite)
    pub db_type: String,
    /// The base path where the database files live
    pub db_path: String,
    /// The name of the database file we're looking for
    pub db_filename: String,
    /// The specific workspace ID we're interested in
    pub workspace_id: String,
    /// The name of the project we're extracting lore from
    pub project_name: String,
    /// Which git branch we're working with
    pub project_branch: String,
    /// The full path to the project directory
    pub project_path: String,
    /// The key in the database where composer data is stored
    pub composer_data_key: String,
    /// The key in the database where generation data is stored
    pub generations_key: String,
    /// The key in the database where prompt data is stored
    pub prompts_key: String,
    /// Whether to include sensitive information in the output
    pub include_secrets: bool,
    /// Whether to include full absolute paths (privacy concern)
    pub include_absolute_paths: bool,
    /// Whether to include system information in the metadata
    pub include_system_info: bool,
}

impl Config {
    /// Load configuration from a file and environment variables.
    /// This is where we read all our settings from the config file and
    /// set up sensible defaults for anything that's not specified.
    pub fn load(config_file: &str) -> Result<Self> {
        // First, try to load environment variables from the config file
        // If the file doesn't exist, that's okay - we'll just use defaults
        dotenv::from_filename(config_file).ok();
        
        // Now we build our configuration struct, reading from environment variables
        // and falling back to sensible defaults if something isn't set
        Ok(Config {
            app_name: env::var("APP_NAME").unwrap_or_else(|_| "chat-history-consolidator".to_string()),
            output_dir: env::var("OUTPUT_DIR").unwrap_or_else(|_| ".knowledge".to_string()),
            output_filename: env::var("OUTPUT_FILENAME").unwrap_or_else(|_| "chat-history-consolidated.md".to_string()),
            db_type: env::var("DB_TYPE").unwrap_or_else(|_| "sqlite".to_string()),
            db_path: env::var("DB_PATH").unwrap_or_else(|_| "~/Library/Application Support/Cursor/User/workspaceStorage".to_string()),
            db_filename: env::var("DB_FILENAME").unwrap_or_else(|_| "state.vscdb".to_string()),
            workspace_id: env::var("WORKSPACE_ID").unwrap_or_else(|_| "default-workspace".to_string()),
            project_name: env::var("PROJECT_NAME").unwrap_or_else(|_| "unknown-project".to_string()),
            project_branch: env::var("PROJECT_BRANCH").unwrap_or_else(|_| "main".to_string()),
            project_path: env::var("PROJECT_PATH").unwrap_or_else(|_| "/path/to/project".to_string()),
            composer_data_key: env::var("COMPOSER_DATA_KEY").unwrap_or_else(|_| "composer.composerData".to_string()),
            generations_key: env::var("GENERATIONS_KEY").unwrap_or_else(|_| "aiService.generations".to_string()),
            prompts_key: env::var("PROMPTS_KEY").unwrap_or_else(|_| "aiService.prompts".to_string()),
            include_secrets: env::var("INCLUDE_SECRETS").unwrap_or_else(|_| "false".to_string()).parse().unwrap_or(false),
            include_absolute_paths: env::var("INCLUDE_ABSOLUTE_PATHS").unwrap_or_else(|_| "false".to_string()).parse().unwrap_or(false),
            include_system_info: env::var("INCLUDE_SYSTEM_INFO").unwrap_or_else(|_| "true".to_string()).parse().unwrap_or(true),
        })
    }
    
    /// Build the full path to the database file we want to connect to.
    /// This takes the base database path, expands any ~ symbols, and
    /// combines it with the workspace ID and database filename.
    pub fn database_path(&self) -> String {
        // Expand the tilde (~) to the user's home directory
        let expanded_path = shellexpand::tilde(&self.db_path).to_string();
        Path::new(&expanded_path)
            .join(&self.workspace_id)
            .join(&self.db_filename)
            .to_string_lossy()
            .to_string()
    }
    
    /// Clean up paths for privacy by replacing absolute paths with placeholders.
    /// This is useful when we want to share the generated markdown without
    /// exposing sensitive directory information.
    pub fn sanitize_path(&self, path: &str) -> String {
        if self.include_absolute_paths {
            // User wants to keep absolute paths, so just return as-is
            path.to_string()
        } else {
            // Replace sensitive path information with generic placeholders
            path.replace(&self.project_path, "<PROJECT_PATH>")
                .replace(&self.db_path, "<DB_PATH>")
        }
    }
}
