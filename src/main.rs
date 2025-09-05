use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::Path;

use chat_history_consolidator::{Config, ChatExtractor, MarkdownGenerator};

/// Command-line interface for the persistent code lore tool.
/// This struct defines all the command-line arguments that users can pass
/// to customize how the tool extracts and processes chat histories.
#[derive(Parser)]
#[command(name = "chat-history-consolidator")]
#[command(about = "Extract and consolidate chat histories from various sources into persistent code lore")]
struct Cli {
    /// Path to the configuration file that contains all our settings.
    /// If not specified, we'll look for 'config.env' in the current directory.
    #[arg(short, long, default_value = "config.env")]
    config: String,
    
    /// Where to put the generated markdown files.
    /// If not specified, we'll use the default from the config file.
    #[arg(long)]
    output_dir: Option<String>,
    
    /// What to name the consolidated markdown file.
    /// If not specified, we'll use the default from the config file.
    #[arg(long)]
    output_file: Option<String>,
    
    /// Print extra information about what we're doing.
    /// Useful for debugging or just seeing what's happening under the hood.
    #[arg(short, long)]
    verbose: bool,
}

/// Main entry point for our persistent code lore application.
/// This is where everything starts - we parse command line arguments,
/// load configuration, extract chat data, and generate the final markdown.
#[tokio::main]
async fn main() -> Result<()> {
    // First things first - let's see what the user wants us to do
    let cli = Cli::parse();
    
    // Load up our configuration from the file the user specified
    // (or the default one if they didn't specify anything)
    let config = Config::load(&cli.config)?;
    
    // If the user wants to see what's going on, let's tell them
    if cli.verbose {
        println!("Configuration loaded from: {}", cli.config);
        println!("Database path: {}", config.database_path());
        println!("Output directory: {}", config.output_dir);
    }
    
    // Now we need to connect to the database and set up our data extractor
    // This is where we'll pull all the chat history from the SQLite database
    let extractor = ChatExtractor::new(&config).await?;
    
    // Time to extract all the good stuff from the database
    // We're looking for three types of data: chat sessions, generations, and prompts
    let sessions = extractor.extract_sessions().await?;
    let generations = extractor.extract_generations().await?;
    let prompts = extractor.extract_prompts().await?;
    
    // Let the user know how much data we found (if they want to know)
    if cli.verbose {
        println!("Extracted {} chat sessions", sessions.len());
        println!("Extracted {} generations", generations.len());
        println!("Extracted {} prompts", prompts.len());
    }
    
    // Now comes the fun part - we take all that raw data and turn it into
    // a nice, readable markdown file that tells the story of the code
    let generator = MarkdownGenerator::new(&config);
    let markdown_content = generator.generate_consolidated_history(
        &sessions,
        &generations,
        &prompts,
    )?;
    
    // Make sure the output directory exists before we try to write to it
    // (nothing worse than a file write error because the directory doesn't exist)
    let output_dir = cli.output_dir.unwrap_or(config.output_dir.clone());
    fs::create_dir_all(&output_dir)?;
    
    // Finally, write our beautiful markdown file to disk
    let output_file = cli.output_file.unwrap_or(config.output_filename.clone());
    let output_path = Path::new(&output_dir).join(&output_file);
    fs::write(&output_path, markdown_content)?;
    
    // Success! Let the user know we're done and where to find their file
    println!("Chat history consolidated successfully!");
    println!("Output file: {}", output_path.display());
    
    Ok(())
}