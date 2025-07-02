# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-01-02

Initial release of cc2report - an intelligent work report generator for Claude Code.

### Features
- **AI-Powered Analysis**: Uses OpenAI GPT models to analyze Claude Code conversation logs
- **Multi-Language Support**: Generates reports in 19 languages with automatic locale detection
- **Smart Date Filtering**: Default behavior generates today's report; use `--all` for all conversations
- **Progress Indicators**: Visual feedback during analysis with the indicatif crate
- **Template Customization**: TOML-based templates for customizing AI prompts and report formats
- **API Response Caching**: File-based caching with configurable TTL to reduce API costs
- **Token Usage Tracking**: Monitors API usage and provides cost estimates
- **Parallel Processing**: Concurrent API requests for faster multi-project analysis
- **Flexible Output**: Supports both Markdown and JSON output formats
- **Comprehensive CLI**: Rich command-line interface with date filtering, language selection, and more

### Technical Details
- Modular architecture with separate config, error, and logging modules
- Custom error types for better error handling
- UTF-8 safe string handling for international content
- Async/await with Tokio for efficient I/O operations
- Comprehensive test suite with integration tests

