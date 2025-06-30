# cc2report

*Read this in other languages: [日本語](README-ja.md) | [简体中文](README-zh.md) | [한국어](README-ko.md) | [Español](README-es.md) | [Français](README-fr.md) | [Deutsch](README-de.md) | [Português](README-pt.md) | [Русский](README-ru.md) | [Italiano](README-it.md) | [Nederlands](README-nl.md) | [Polski](README-pl.md) | [Türkçe](README-tr.md) | [العربية](README-ar.md) | [हिन्दी](README-hi.md) | [ไทย](README-th.md) | [Tiếng Việt](README-vi.md) | [Bahasa Indonesia](README-id.md) | [Bahasa Melayu](README-ms.md)*

An intelligent work report generator for Claude Code that analyzes conversation logs and creates comprehensive work reports using AI.

## Features

- **AI-Powered Analysis**: Uses OpenAI's GPT models to analyze conversations and generate intelligent summaries
- **Multi-language Support**: Generates reports in 19 languages (auto-detected from system locale)
- **Smart Caching**: Caches API responses to reduce costs and improve performance (12x speedup on cached runs)
- **Parallel Processing**: Process multiple projects concurrently for faster analysis
- **Flexible Date Filtering**: Generate reports for specific dates, weeks, or months
- **Progress Indicators**: Visual progress bars for long-running operations
- **Token Usage Tracking**: Monitor API usage and costs
- **Template Customization**: Customize AI prompts and report formats

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/signal-slot/cc2report.git
cd cc2report

# Build and install
cargo build --release
cargo install --path .
```

### Prerequisites

- Rust 1.70 or higher
- OpenAI API key

## Configuration

### API Key Setup

Set your OpenAI API key as an environment variable:

```bash
export OPENAI_API_KEY="your-api-key-here"
```

Or pass it directly via command line:

```bash
cc2report --api-key "your-api-key-here"
```

### Default Paths

- **Log Directory**: `~/.claude/projects/` (Claude Code's default project directory)
- **Cache Directory**: `~/.cache/cc2report/`
- **Template File**: `./cc2report.toml` or `~/.config/cc2report/templates.toml`

## Usage

### Basic Usage

Generate a report for today's conversations (default):

```bash
cc2report
```

Generate a report for all conversations (no date filter):

```bash
cc2report --all
```

### Date Filtering

```bash
# Specific date
cc2report --date 2024-07-01

# Date range
cc2report --from 2024-07-01 --to 2024-07-07

# Current week
cc2report --weekly

# Current month
cc2report --monthly
```

### Output Options

```bash
# Save to file
cc2report --output report.md

# JSON format
cc2report --format json --output report.json

# Specify language (auto-detected by default)
cc2report --lang ja  # Japanese
cc2report --lang zh  # Chinese
cc2report --lang es  # Spanish

# Example: Today's report in Japanese
cc2report --lang ja

# Example: This week's report in Spanish
cc2report --weekly --lang es
```

### Performance Options

```bash
# Enable parallel processing (max 10)
cc2report --parallel 4

# Disable progress indicators
cc2report --quiet

# Show token usage and costs
cc2report --show-token-usage
```

### Cache Management

```bash
# Clear cache
cc2report --clear-cache

# Show cache information
cc2report --cache-info
```

### Template Customization

Generate a template file:

```bash
cc2report --generate-template my-template.toml
```

Edit the template to customize prompts and report formats.

## Supported Languages

The tool automatically detects your system language and generates reports accordingly. Supported languages include:

- English (en)
- Japanese (ja)
- Chinese (zh)
- Korean (ko)
- Spanish (es)
- French (fr)
- German (de)
- Portuguese (pt)
- Russian (ru)
- Italian (it)
- Dutch (nl)
- Polish (pl)
- Turkish (tr)
- Arabic (ar)
- Hindi (hi)
- Thai (th)
- Vietnamese (vi)
- Indonesian (id)
- Malay (ms)

## Report Structure

The generated reports include:

- **Project Title and Objective**: Clear summary of what was being worked on
- **Activities**: Categorized list of work performed
- **Delivered Value**: Concrete accomplishments and features implemented
- **Technical Improvements**: Code quality and performance enhancements
- **Unresolved Issues**: Any blockers or pending tasks

## Cost Optimization

The tool includes several features to minimize API costs:

1. **Caching**: Responses are cached for 24 hours
2. **Model Selection**: Choose between different models based on cost/quality tradeoff
3. **Token Tracking**: Monitor usage to stay within budget

### Model Pricing (as of 2024)

| Model | Input Cost | Output Cost | Recommendation |
|-------|------------|-------------|----------------|
| gpt-4o (default) | $2.50/1M | $10.00/1M | Best quality |
| gpt-4o-mini | $0.15/1M | $0.60/1M | Best value |
| gpt-3.5-turbo | $0.50/1M | $1.50/1M | Budget option |

## Example Report

```markdown
# Work Report - 2024-07-01

## cc2report - Work Report Generator

**Objective**: Generate human-readable work reports from Claude Code logs

**Activities**:
- Development - Implemented OpenAI API integration with GPT-4
- Feature Addition - Developed smart analysis with intelligent categorization
- UI Enhancement - Improved command-line interface experience

**Delivered Value**:
- Reduced "Other Tasks" category by 90%
- Significantly improved report readability
- Achieved multi-language support (19 languages)

**Technical Improvements**:
- Optimized performance
- Enhanced error handling
```

## Troubleshooting

### Common Issues

1. **"OpenAI API key is required"**
   - Ensure `OPENAI_API_KEY` is set in your environment
   - Or use `--api-key` option

2. **"Log directory does not exist"**
   - Specify the correct path with `--log-dir`
   - Default is `~/.claude/projects/`

3. **Rate Limit Errors**
   - Reduce parallel requests: `--parallel 1`
   - Use a lower-tier model: `--model gpt-3.5-turbo`

## Development

### Building from Source

```bash
# Development build
cargo build

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run
```

### Architecture

The project is organized into the following modules:

- `parser`: JSONL log file parsing
- `conversation_analyzer`: Extract topics and context from conversations
- `ai_analyzer`: OpenAI API integration
- `smart_analyzer`: Report generation
- `cache`: API response caching
- `templates`: Customizable prompts and formats
- `config`: Configuration management
- `error`: Error handling
- `cli`: Command-line interface

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built for [Claude Code](https://github.com/cline/cline) (formerly Claude Engineer)
- Uses OpenAI's GPT models for intelligent analysis
- Inspired by the need for automated work reporting in AI-assisted development