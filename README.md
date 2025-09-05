<div align="center">

# peRsistent code lore

![pcR Logo](logo-simple.svg)

*A Rust application that extracts and consolidates chat histories from various sources into a single metadata-equipped markdown file, preserving the knowledge and context of code development sessions.*

</div>

## Features

- **Multi-source extraction**: Supports SQLite databases from various chat applications
- **Configurable**: Environment-based configuration with sensible defaults
- **Metadata-rich**: Generates comprehensive metadata for each chat session
- **Privacy-focused**: Option to exclude secrets and absolute paths
- **Extensible**: Easy to add support for new chat sources

## Installation

### Prerequisites

- Rust 1.70+ 
- SQLite3

### Build from source

```bash
git clone <repository-url>
cd chat-history-consolidator
cargo build --release
```

## Configuration

The application uses environment variables for configuration. Copy `config.env` to `.env` and modify as needed:

```bash
cp config.env .env
```

### Configuration Options

| Variable | Default | Description |
|----------|---------|-------------|
| `APP_NAME` | `persistent-code-lore` | Application name |
| `OUTPUT_DIR` | `.knowledge` | Output directory for consolidated files |
| `OUTPUT_FILENAME` | `chat-history-consolidated.md` | Output filename |
| `DB_TYPE` | `sqlite` | Database type |
| `DB_PATH` | `~/Library/Application Support/Cursor/User/workspaceStorage` | Database path |
| `DB_FILENAME` | `state.vscdb` | Database filename |
| `WORKSPACE_ID` | `` | Workspace identifier |
| `PROJECT_NAME` | `` | Project name |
| `PROJECT_BRANCH` | `` | Git branch |
| `PROJECT_PATH` | `/Users/dav/coding/tools/mcp_servers/local-llm-proxy` | Project path |
| `COMPOSER_DATA_KEY` | `composer.composerData` | Composer data key in database |
| `GENERATIONS_KEY` | `aiService.generations` | Generations data key |
| `PROMPTS_KEY` | `aiService.prompts` | Prompts data key |
| `INCLUDE_SECRETS` | `false` | Include sensitive information |
| `INCLUDE_ABSOLUTE_PATHS` | `false` | Include absolute file paths |
| `INCLUDE_SYSTEM_INFO` | `true` | Include system information |

## Usage

### Basic usage

```bash
# Use default configuration
cargo run

# Use custom configuration file
cargo run -- --config custom.env

# Specify output directory and filename
cargo run -- --output-dir ./output --output-file my-code-lore.md

# Enable verbose output
cargo run -- --verbose
```

### Command line options

- `--config <FILE>`: Path to configuration file (default: `config.env`)
- `--output-dir <DIR>`: Output directory for consolidated files
- `--output-file <FILE>`: Output filename for consolidated markdown
- `--verbose`: Enable verbose output

## Output Format

The application generates a comprehensive markdown file containing:

- **Metadata**: Creation time, project info, system details
- **Historical Sessions**: All chat sessions with timestamps and context
- **Project Context**: Description of the project and its components
- **Topics and Themes**: Categorized analysis of chat topics
- **Project Structure**: File and directory structure reference
- **Key Features**: Implemented functionality overview
- **Data Sources**: Information about where data was extracted from
- **Notes**: Additional context and usage information

## Supported Sources

Currently supports:
- **Cursor**: SQLite-based chat storage from Cursor IDE

Planned support:
- **VS Code**: Extension-based chat storage
- **GitHub Copilot**: Chat history export
- **Custom**: Generic SQLite database format

## Development

### Project Structure

```
src/
├── main.rs          # Application entry point
├── config.rs        # Configuration management
├── extractor.rs     # Data extraction logic
└── generator.rs     # Markdown generation
```

### Adding New Sources

To add support for a new chat source:

1. Implement extraction logic in `extractor.rs`
2. Add configuration options in `config.rs`
3. Update the main application logic in `main.rs`

### Testing

```bash
# Run tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Troubleshooting

### Common Issues

**Database not found**: Ensure the database path is correct and the file exists.

**Permission denied**: Check file permissions for the database and output directory.

**Configuration errors**: Verify all required environment variables are set.

### Debug Mode

Run with verbose output to see detailed information:

```bash
cargo run -- --verbose
```

## Changelog

### v0.1.0
- Initial release
- SQLite database support
- Cursor IDE integration
- Markdown generation with metadata
- Configurable privacy settings

## Roadmap
- sensitive data filtering before storage - replace with placeholders
- add memory graph (e.g. OWL / RDFL) to aid cause-effect and logical thinking to assist coding