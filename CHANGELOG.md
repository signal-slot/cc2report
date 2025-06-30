# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Architecture Improvements**
  - Modular design with separate config, error, and logging modules
  - Custom error types for better error handling
  - Centralized configuration management
  - Simple logging system with different log levels
- **New CLI Options**
  - `--all` flag to generate reports for all conversations without date filter

### Changed
- **Default Behavior**
  - **BREAKING**: Default behavior now generates report for today's conversations only
  - Use `--all` flag to get the previous behavior (report for all conversations)
  - This change makes the tool more practical for daily use
- **Report Format Simplification**
  - Removed executive summary section by default
  - Token usage now hidden by default (use `--show-token-usage` to display)
  - Projects now appear at `##` level in markdown reports
  - Inverted token tracking flag logic for better UX
- **Language Unification**
  - All error messages unified to English
  - System messages in English across all modules
  - Consistent English messaging for better international usability

### Fixed
- Compilation error with orphan rule violation
- Test failures due to missing token_tracker field

## [2.1.0] - 2024-07-02

### Added
- **Progress Indication**: Added progress bars during API analysis and directory scanning
  - Visual feedback shows which project is being analyzed
  - Can be suppressed with `--quiet` flag
  
- **Template Customization**: Added ability to customize AI prompts and report formats
  - Generate template with `--generate-template`
  - Templates support TOML format
  - Automatic template discovery in current directory or `~/.config/cc2report/`
  
- **API Response Caching**: Implemented file-based caching to reduce API costs
  - 24-hour TTL for cached responses
  - Cache management commands: `--cache-info` and `--clear-cache`
  - Achieved 12x speedup for repeated queries
  
- **Token Usage Tracking**: Added detailed token usage and cost estimation
  - Shows input/output token counts
  - Calculates costs based on current OpenAI pricing
  - Per-project token usage breakdown
  - Hidden by default, enable with `--show-token-usage`
  
- **Parallel API Processing**: Implemented concurrent API requests for faster processing
  - Configure with `--parallel N` (default: 1, max: 10)
  - Significant performance improvements for multi-project analysis
  - Maintains proper rate limiting

### Changed
- Improved error handling for failed API requests
- Better support for multi-language prompts

### Technical Details
- Added dependencies: `indicatif`, `toml`, `futures`
- Introduced new modules: `templates`, `cache`, `token_tracker`, `ai_analyzer_parallel`
- All features integrate seamlessly with existing multi-language support

## [2.0.0] - 2024-06-15

### Added
- AI-powered conversation analysis using OpenAI GPT models
- Support for 19 languages with automatic detection
- Intelligent categorization of development activities
- Value-focused reporting instead of tool-based summaries

### Changed
- Complete rewrite from keyword-based to AI-driven analysis
- Moved from simple categorization to semantic understanding
- Enhanced report quality and readability

### Removed
- Keyword-based categorization system
- Hardcoded activity patterns

## [1.0.0] - 2024-05-01

### Added
- Initial release
- Basic JSONL log parsing
- Simple keyword-based categorization
- Markdown report generation
- Date filtering options
- Japanese language support

[Unreleased]: https://github.com/signal-slot/cc2report/compare/v2.1.0...HEAD
[2.1.0]: https://github.com/signal-slot/cc2report/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/signal-slot/cc2report/compare/v1.0.0...v2.0.0
[1.0.0]: https://github.com/signal-slot/cc2report/releases/tag/v1.0.0