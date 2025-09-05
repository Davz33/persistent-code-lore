use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};

use crate::config::Config;
use crate::{ChatGeneration, ChatPrompt, ComposerData};

/// The ChatExtractor is responsible for pulling data out of the SQLite database.
/// Think of it as our "data miner" - it knows how to connect to the database
/// and extract all the chat-related information we need.
pub struct ChatExtractor {
    /// Our connection to the SQLite database
    pool: SqlitePool,
    /// Configuration settings that tell us what to look for
    config: Config,
}

impl ChatExtractor {
    /// Create a new ChatExtractor and connect to the database.
    /// This is where we establish our connection to the SQLite database
    /// so we can start pulling out chat data.
    pub async fn new(config: &Config) -> Result<Self> {
        // Build the database URL that SQLx needs to connect
        let database_url = format!("sqlite:{}", config.database_path());
        let pool = SqlitePool::connect(&database_url).await?;
        
        Ok(ChatExtractor {
            pool,
            config: config.clone(),
        })
    }
    
    /// Extract all the chat sessions from the database.
    /// This pulls out the main session data that tells us about each
    /// conversation that happened in the chat application.
    pub async fn extract_sessions(&self) -> Result<Vec<ComposerData>> {
        // Query the database for the composer data (this contains session info)
        let query = format!(
            "SELECT value FROM ItemTable WHERE key = '{}'",
            self.config.composer_data_key
        );
        
        let row = sqlx::query(&query)
            .fetch_one(&self.pool)
            .await?;
        
        // Parse the JSON data we got from the database
        let json_str: String = row.get(0);
        let composer_data: ComposerData = serde_json::from_str(&json_str)?;
        
        Ok(vec![composer_data])
    }
    
    /// Extract all the generation data from the database.
    /// This contains information about what the AI generated during conversations.
    pub async fn extract_generations(&self) -> Result<Vec<ChatGeneration>> {
        let query = format!(
            "SELECT value FROM ItemTable WHERE key = '{}'",
            self.config.generations_key
        );
        
        let row = sqlx::query(&query)
            .fetch_one(&self.pool)
            .await?;
        
        let json_str: String = row.get(0);
        let generations: Vec<ChatGeneration> = serde_json::from_str(&json_str)?;
        
        Ok(generations)
    }
    
    /// Extract all the prompt data from the database.
    /// This contains the user's input prompts that started each conversation.
    pub async fn extract_prompts(&self) -> Result<Vec<ChatPrompt>> {
        let query = format!(
            "SELECT value FROM ItemTable WHERE key = '{}'",
            self.config.prompts_key
        );
        
        let row = sqlx::query(&query)
            .fetch_one(&self.pool)
            .await?;
        
        let json_str: String = row.get(0);
        let prompts: Vec<ChatPrompt> = serde_json::from_str(&json_str)?;
        
        Ok(prompts)
    }
    
    pub async fn get_database_info(&self) -> Result<DatabaseInfo> {
        let tables_query = "SELECT name FROM sqlite_master WHERE type='table'";
        let tables: Vec<String> = sqlx::query_scalar(tables_query)
            .fetch_all(&self.pool)
            .await?;
        
        let item_count_query = "SELECT COUNT(*) FROM ItemTable";
        let item_count: i64 = sqlx::query_scalar(item_count_query)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(DatabaseInfo {
            tables,
            item_count,
            database_path: self.config.sanitize_path(&self.config.database_path()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseInfo {
    pub tables: Vec<String>,
    pub item_count: i64,
    pub database_path: String,
}
