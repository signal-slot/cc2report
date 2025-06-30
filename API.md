# API Documentation

This document describes the public API and module structure of cc2report.

## Module Overview

### Core Modules

#### `parser`
Handles parsing of Claude Code's JSONL log files.

**Key Types:**
- `LogEntry` - Represents a single log entry
- `LogRecord` - Enum for different log record types

**Key Functions:**
- `parse_jsonl_file()` - Parse a JSONL file into log entries

#### `conversation_analyzer`
Analyzes conversation flow and extracts topics.

**Key Types:**
- `ConversationFlow` - Tracks conversation state
- `Topic` - Represents a conversation topic
- `TaskItem` - Individual task within a topic

**Key Functions:**
- `analyze_user_message()` - Process user messages
- `analyze_assistant_action()` - Process assistant responses

#### `ai_analyzer`
Integrates with OpenAI API for intelligent analysis.

**Key Types:**
- `AiAnalysisRequest` - Request structure for AI analysis
- `AiAnalysisResponse` - AI analysis results
- `ConversationData` - Conversation data for analysis

**Key Functions:**
- `analyze_with_ai()` - Main AI analysis function
- `analyze_with_ai_parallel()` - Parallel processing variant

#### `smart_analyzer`
Generates the final work reports.

**Key Types:**
- `SmartReport` - Complete report structure
- `SmartProjectSummary` - Individual project summary
- `WorkBreakdown` - Activity categorization

**Key Functions:**
- `generate_smart_report()` - Generate markdown/output from report data

### Support Modules

#### `cache`
File-based caching system for API responses.

**Key Types:**
- `ApiCache` - Cache management

**Key Functions:**
- `get()` - Retrieve cached response
- `set()` - Store response in cache
- `clear()` - Clear all cache entries
- `size()` - Get cache size

#### `templates`
Template management for customizable prompts.

**Key Types:**
- `Templates` - Template configuration
- `PromptTemplates` - AI prompt templates
- `ReportTemplates` - Report format templates

**Key Functions:**
- `load_from_file()` - Load templates from TOML
- `create_default_template_file()` - Generate template file

#### `token_tracker`
Token usage tracking and cost estimation.

**Key Types:**
- `TokenTracker` - Token usage tracker
- `TokenUsage` - Token count data
- `CostEstimate` - Cost calculation

**Key Functions:**
- `add_usage()` - Record token usage
- `calculate_cost()` - Calculate estimated costs

#### `config`
Application configuration management.

**Key Types:**
- `Config` - Main configuration structure
- `ApiConfig` - API-related settings
- `OutputConfig` - Output format settings
- `ProcessingConfig` - Processing options

**Key Functions:**
- `from_args()` - Build config from CLI arguments
- `validate()` - Validate configuration

#### `error`
Custom error types for better error handling.

**Key Types:**
- `AppError` - Application-level errors
- `ApiError` - API-specific errors
- `Result<T>` - Type alias for results

#### `cli`
Command-line interface definitions.

**Key Functions:**
- `build_cli()` - Build clap CLI application

#### `logger`
Simple logging utilities.

**Key Types:**
- `LogLevel` - Log level enumeration

**Key Functions:**
- `init()` - Initialize logger
- `info()` - Log info message
- `error()` - Log error message
- `debug()` - Log debug message

## Usage Examples

### Basic Analysis

```rust
use cc2report::ai_analyzer::{analyze_with_ai, ConversationData};
use cc2report::smart_analyzer::generate_smart_report;

// Prepare conversation data
let conversations = vec![
    ConversationData {
        project_name: "my-project".to_string(),
        user_messages: vec!["Fix the bug".to_string()],
        assistant_actions: vec!["Fixed bug in main.rs".to_string()],
        timestamps: vec!["2024-07-01T10:00:00Z".to_string()],
    },
];

// Analyze with AI
let (results, token_tracker) = analyze_with_ai(
    "api-key",
    conversations,
    "en",
    "gpt-4o",
    false,  // quiet
    1,      // parallel
).await?;

// Generate report
let report = create_report_from_results(results, token_tracker);
generate_smart_report(&report, None, "en")?;
```

### Custom Templates

```rust
use cc2report::templates::Templates;

// Load custom templates
let templates = Templates::load_from_file(Path::new("my-templates.toml"))?;

// Or create default template file
Templates::create_default_template_file(Path::new("templates.toml"))?;
```

### Cache Management

```rust
use cc2report::cache::ApiCache;

// Initialize cache
let cache = ApiCache::new()?;

// Check cache
if let Some(response) = cache.get("project", "en", "gpt-4o", &messages) {
    // Use cached response
}

// Clear cache
cache.clear()?;
```

## Error Handling

All functions return `Result<T>` types. Handle errors appropriately:

```rust
match analyze_with_ai(...).await {
    Ok((results, tracker)) => {
        // Process results
    }
    Err(e) => {
        eprintln!("Analysis failed: {}", e);
    }
}
```

## Environment Variables

- `OPENAI_API_KEY` - OpenAI API key
- `LANG` - System language (for auto-detection)
- `HOME` - Home directory (for default paths)
- `RUST_LOG` - Logging level (debug, info, warn, error)

## File Formats

### JSONL Log Format
Each line is a JSON object representing a log entry:
```json
{
  "timestamp": "2024-07-01T10:00:00Z",
  "entry_type": "user",
  "message": "User message content",
  "session_id": "session-123"
}
```

### Template TOML Format
```toml
[prompts]
system_message = "You are an AI assistant..."
user_prompt = "Analyze the following..."

[prompts.language_instructions]
en = "Use English for all text fields."
ja = "Use Japanese for all text fields."

[report]
header = "# {title} - {date}\n\n"
project_section = "## {title}\n\n**Objective**: {objective}\n\n"
```

## Performance Considerations

1. **Caching**: Enable caching to avoid redundant API calls
2. **Parallel Processing**: Use `--parallel` for multiple projects
3. **Model Selection**: Choose appropriate model for cost/quality balance
4. **Batch Processing**: Process multiple conversations in one run

## Thread Safety

- `ApiCache` is thread-safe (uses file system locking)
- `TokenTracker` requires external synchronization for concurrent use
- Use `Arc<Mutex<T>>` for shared state in parallel processing