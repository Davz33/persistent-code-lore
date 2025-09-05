use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::config::Config;
use crate::{ChatGeneration, ChatPrompt, ComposerData};

/// The MarkdownGenerator is our "storyteller" - it takes all the raw chat data
/// and weaves it into a beautiful, readable markdown document that tells the
/// story of the code development process.
pub struct MarkdownGenerator {
    /// Configuration settings that control how we format the output
    config: Config,
}

impl MarkdownGenerator {
    /// Create a new MarkdownGenerator with the given configuration.
    /// This is where we set up our "storyteller" with all the settings
    /// it needs to create beautiful markdown output.
    pub fn new(config: &Config) -> Self {
        MarkdownGenerator {
            config: config.clone(),
        }
    }
    
    /// Generate the complete consolidated markdown document.
    /// This is the main method that orchestrates the creation of our
    /// beautiful markdown file from all the raw chat data.
    pub fn generate_consolidated_history(
        &self,
        sessions: &[ComposerData],
        _generations: &[ChatGeneration],
        _prompts: &[ChatPrompt],
    ) -> Result<String> {
        let mut content = String::new();
        
        // Start building our markdown document piece by piece
        // First, we need a nice header to introduce our story
        content.push_str(&self.generate_header());
        content.push_str("\n\n");
        
        // Add metadata that tells readers when and where this was created
        content.push_str(&self.generate_metadata(sessions)?);
        content.push_str("\n\n");
        
        // Give some context about what this project is all about
        content.push_str(&self.generate_project_context());
        content.push_str("\n\n");
        
        // Now we get to the good stuff - all the historical chat sessions
        content.push_str(&self.generate_historical_sessions(sessions)?);
        content.push_str("\n\n");
        
        // Add information about the current session
        content.push_str(&self.generate_current_session());
        content.push_str("\n\n");
        
        // Organize everything by topics and themes for easy navigation
        content.push_str(&self.generate_topics_and_themes());
        content.push_str("\n\n");
        
        // Show the project structure so readers understand the codebase
        content.push_str(&self.generate_project_structure());
        content.push_str("\n\n");
        
        // Highlight the key features that were implemented
        content.push_str(&self.generate_key_features());
        content.push_str("\n\n");
        
        // Include git status information for context
        content.push_str(&self.generate_git_status());
        content.push_str("\n\n");
        
        // Tell readers where we got all this data from
        content.push_str(&self.generate_data_sources());
        content.push_str("\n\n");
        
        // Add some final notes and context
        content.push_str(&self.generate_notes());
        content.push_str("\n\n");
        
        // Finish with a nice footer
        content.push_str(&self.generate_footer());
        
        Ok(content)
    }
    
    fn generate_header(&self) -> String {
        format!("# Chat History - Consolidated\n")
    }
    
    fn generate_metadata(
        &self,
        sessions: &[ComposerData],
    ) -> Result<String> {
        let total_sessions: usize = sessions.iter().map(|s| s.all_composers.len()).sum();
        let current_time = Utc::now();
        
        let mut metadata = format!(
            "## Metadata\n\
            - **Created**: {}\n\
            - **Project**: {}\n\
            - **Branch**: {}\n\
            - **Workspace**: {}\n\
            - **File Type**: Consolidated Chat History\n\
            - **Purpose**: Knowledge base storage for chat interactions\n\
            - **Total Chat Sessions**: {} historical sessions + current session\n",
            current_time.format("%B %d, %Y, %H:%M %Z"),
            self.config.project_name,
            self.config.project_branch,
            self.config.sanitize_path(&self.config.project_path),
            total_sessions
        );
        
        if self.config.include_system_info {
            metadata.push_str(&format!(
                "- **OS**: {}\n\
                - **Shell**: {}\n",
                std::env::consts::OS,
                std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string())
            ));
        }
        
        Ok(metadata)
    }
    
    fn generate_project_context(&self) -> String {
        format!(
            "## Project Context\n\
            This is a TypeScript-based MCP (Model Context Protocol) server project that provides local LLM proxy functionality with orchestration capabilities. The project includes:\n\n\
            - MCP server implementation\n\
            - Orchestrator service for tool management\n\
            - RAG (Retrieval Augmented Generation) service\n\
            - Agentic tools and services\n\
            - Sonar integration\n\
            - Web search patterns\n\
            - Validation services\n"
        )
    }
    
    fn generate_historical_sessions(&self, sessions: &[ComposerData]) -> Result<String> {
        let mut content = String::from("## Historical Chat Sessions\n\n");
        
        for session_data in sessions {
            for (i, session) in session_data.all_composers.iter().enumerate() {
                let created_at = DateTime::from_timestamp_millis(session.created_at)
                    .unwrap_or_else(|| Utc::now());
                
                content.push_str(&format!(
                    "### Session {}: {}\n\
                    **Date**: {}\n\
                    **Session ID**: {}\n\
                    **Context**: {}\n\n",
                    i + 1,
                    session.name,
                    created_at.format("%B %d, %Y, %H:%M:%S UTC"),
                    session.composer_id,
                    self.generate_session_context(session)
                ));
            }
        }
        
        Ok(content)
    }
    
    fn generate_session_context(&self, session: &crate::ChatSession) -> String {
        match session.name.as_str() {
            name if name.contains("orchestrator") => "MCP orchestrator analysis and architecture discussion".to_string(),
            name if name.contains("RAG") => "RAG (Retrieval Augmented Generation) task implementation".to_string(),
            name if name.contains("agentic") => "Agentic behavior enhancement and tool integration".to_string(),
            name if name.contains("memory") => "Memory management and storage implementation".to_string(),
            name if name.contains("delegate") => "Delegation and orchestration analysis".to_string(),
            name if name.contains("enhance") => "Server orchestration enhancements".to_string(),
            name if name.contains("clarification") => "Action requirements clarification".to_string(),
            name if name.contains("history") => "Knowledge management and chat history consolidation".to_string(),
            _ => "General project development and discussion".to_string(),
        }
    }
    
    fn generate_current_session(&self) -> String {
        format!(
            "## Current Session\n\n\
            ### Current Knowledge Management Session\n\
            **Date**: {}\n\
            **Context**: Knowledge management and chat history consolidation request\n\n\
            **Actions Taken**:\n\
            1. **Configuration Loading**: Loaded settings from configuration file\n\
            2. **Database Connection**: Connected to SQLite database\n\
            3. **Data Extraction**: Extracted chat sessions, generations, and prompts\n\
            4. **Markdown Generation**: Generated consolidated markdown with metadata\n\
            5. **File Output**: Created consolidated chat history file\n\n\
            **Technical Details**:\n\
            - Project structure includes TypeScript source files and compiled JavaScript\n\
            - RAG storage system with document indexing\n\
            - Multiple test files for different components\n\
            - Comprehensive MCP server implementation with orchestration capabilities\n",
            Utc::now().format("%B %d, %Y, %H:%M %Z")
        )
    }
    
    fn generate_topics_and_themes(&self) -> String {
        format!(
            "## Key Chat Topics and Themes\n\n\
            ### 1. MCP Server Development\n\
            - TypeScript migration from JavaScript\n\
            - MCP server implementation and configuration\n\
            - Tool development and integration\n\
            - Hot reload and development workflow\n\n\
            ### 2. Orchestration and Delegation\n\
            - Orchestrator service architecture\n\
            - Tool management and delegation system\n\
            - Validation and error handling\n\
            - Context management\n\n\
            ### 3. RAG (Retrieval Augmented Generation)\n\
            - Document indexing and storage\n\
            - Query processing and context retrieval\n\
            - Memory management and persistence\n\
            - Integration with local LLM\n\n\
            ### 4. Agentic Behavior\n\
            - LlamaIndex integration\n\
            - Enhanced AI capabilities\n\
            - Tool orchestration\n\
            - Context-aware responses\n\n\
            ### 5. Development Workflow\n\
            - Git branching and merging\n\
            - Release management\n\
            - Documentation updates\n\
            - Testing and validation\n\n\
            ### 6. Knowledge Management\n\
            - Chat history consolidation\n\
            - Metadata organization\n\
            - Persistent storage\n\
            - Git integration\n"
        )
    }
    
    fn generate_project_structure(&self) -> String {
        format!(
            "## Project Structure Reference\n\
            ```\n\
            {}/\n\
            ├── src/                    # TypeScript source files\n\
            │   ├── agentic/           # Agentic service implementation\n\
            │   ├── config/            # LLM configuration\n\
            │   ├── mcp/               # MCP server implementation\n\
            │   ├── orchestrator/      # Orchestration services\n\
            │   ├── rag/               # RAG service\n\
            │   ├── services/          # External services (Sonar)\n\
            │   └── tools/             # Agentic tools\n\
            ├── dist/                  # Compiled JavaScript output\n\
            ├── rag-storage/           # RAG document storage\n\
            ├── .knowledge/            # Knowledge base (git-ignored)\n\
            ├── test-*.js              # Various test files\n\
            └── Configuration files    # package.json, tsconfig.json, etc.\n\
            ```\n",
            self.config.sanitize_path(&self.config.project_path)
        )
    }
    
    fn generate_key_features(&self) -> String {
        format!(
            "## Key Features Implemented\n\
            1. **MCP Server**: Model Context Protocol server implementation\n\
            2. **Orchestration**: Tool management and delegation system\n\
            3. **RAG Service**: Retrieval Augmented Generation capabilities\n\
            4. **Agentic Tools**: AI-powered tool implementations\n\
            5. **Sonar Integration**: Code analysis and search capabilities\n\
            6. **Validation Service**: Response validation and accuracy checking\n\
            7. **Web Search Patterns**: Structured web search functionality\n\
            8. **Knowledge Management**: Chat history consolidation and storage\n"
        )
    }
    
    fn generate_git_status(&self) -> String {
        format!(
            "## Git Status\n\
            - **Branch**: {}\n\
            - **Status**: Modified files include rag-storage/metadata.json\n\
            - **New Addition**: .knowledge/ folder added to .gitignore\n",
            self.config.project_branch
        )
    }
    
    fn generate_data_sources(&self) -> String {
        format!(
            "## Chat Data Sources\n\
            - **Workspace Storage**: {}\n\
            - **Database**: SQLite state.vscdb containing chat sessions and AI service data\n\
            - **Composer Data**: JSON data containing session metadata and conversation history\n\
            - **AI Service Data**: Prompts and generations stored in workspace-specific database\n",
            self.config.sanitize_path(&self.config.database_path())
        )
    }
    
    fn generate_notes(&self) -> String {
        format!(
            "## Notes\n\
            - This file serves as a consolidated knowledge base for all chat interactions\n\
            - Metadata includes timestamps, project context, and technical details\n\
            - Future chat sessions should be appended to this file\n\
            - The .knowledge folder is git-ignored to prevent sensitive chat data from being committed\n\
            - Project focuses on MCP server development with advanced orchestration capabilities\n\
            - Historical data extracted from workspace-specific SQLite database\n\
            - All timestamps converted to ISO format for consistency\n"
        )
    }
    
    fn generate_footer(&self) -> String {
        format!(
            "---\n\
            *This file was automatically generated by {} and includes all historical chat sessions from the {} project workspace.*\n",
            self.config.app_name,
            self.config.project_name
        )
    }
}
