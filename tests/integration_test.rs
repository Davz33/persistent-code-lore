#[tokio::test]
async fn test_config_loading() {
    // Test that configuration can be loaded
    let config = chat_history_consolidator::config::Config::load("config.env");
    assert!(config.is_ok());
    
    let config = config.unwrap();
    assert_eq!(config.app_name, "persistent-code-lore");
    assert_eq!(config.output_dir, ".knowledge");
}

#[tokio::test]
async fn test_markdown_generation() {
    use chat_history_consolidator::config::Config;
    use chat_history_consolidator::generator::MarkdownGenerator;
    use chat_history_consolidator::{ChatSession, ComposerData};
    
    let config = Config::load("config.env").unwrap();
    let generator = MarkdownGenerator::new(&config);
    
    // Create test data
    let test_session = ChatSession {
        session_type: "head".to_string(),
        composer_id: "test-id".to_string(),
        name: "Test Session".to_string(),
        last_updated_at: 1757092753004,
        created_at: 1757092558319,
        unified_mode: "agent".to_string(),
        force_mode: "edit".to_string(),
        has_unread_messages: false,
    };
    
    let composer_data = ComposerData {
        all_composers: vec![test_session],
    };
    
    // Generate markdown
    let result = generator.generate_consolidated_history(&[composer_data], &[], &[]);
    assert!(result.is_ok());
    
    let markdown = result.unwrap();
    assert!(markdown.contains("# Chat History - Consolidated"));
    assert!(markdown.contains("Test Session"));
}

