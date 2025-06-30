# Contributing to cc2report

Thank you for your interest in contributing to cc2report! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

By participating in this project, you agree to abide by our code of conduct:

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on what is best for the community
- Show empathy towards other community members

## How to Contribute

### Reporting Issues

1. **Check existing issues** - Before creating a new issue, please check if it already exists
2. **Use issue templates** - When available, use the appropriate issue template
3. **Provide details** - Include:
   - Clear description of the problem
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - System information (OS, Rust version, etc.)
   - Relevant logs or error messages

### Suggesting Features

1. **Check the roadmap** - See if the feature is already planned
2. **Create a feature request** - Use the feature request template
3. **Explain the use case** - Help us understand why this feature would be valuable
4. **Consider implementation** - If possible, suggest how it might be implemented

### Submitting Pull Requests

1. **Fork the repository** - Create your own fork of cc2report
2. **Create a branch** - Use a descriptive branch name (e.g., `feat/add-pdf-export`, `fix/cache-error`)
3. **Make your changes** - Follow the coding standards below
4. **Test your changes** - Ensure all tests pass and add new tests if needed
5. **Commit your changes** - Use clear, descriptive commit messages
6. **Push to your fork** - Push your branch to your GitHub fork
7. **Create a pull request** - Submit a PR with a clear description

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- Git
- An OpenAI API key for testing

### Getting Started

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/cc2report.git
cd cc2report

# Add upstream remote
git remote add upstream https://github.com/signal-slot/cc2report.git

# Install dependencies and build
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run
```

### Project Structure

```
cc2report/
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library exports
│   ├── parser.rs         # JSONL parsing
│   ├── conversation_analyzer.rs
│   ├── ai_analyzer.rs    # OpenAI integration
│   ├── smart_analyzer.rs # Report generation
│   ├── cache.rs          # Caching system
│   ├── templates.rs      # Template management
│   ├── config.rs         # Configuration
│   ├── error.rs          # Error types
│   ├── cli.rs            # CLI definitions
│   └── logger.rs         # Logging utilities
├── tests/                # Integration tests
├── Cargo.toml           # Package manifest
└── README.md            # Project documentation
```

## Coding Standards

### Rust Style

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- Use `cargo fmt` to format your code
- Use `cargo clippy` to catch common mistakes
- Prefer explicit error handling over `unwrap()`
- Document public APIs with doc comments

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test additions or modifications
- `chore`: Build process or auxiliary tool changes

Examples:
```
feat(cache): add TTL configuration option

fix(parser): handle empty JSONL files correctly

docs(readme): add troubleshooting section
```

### Testing

- Write tests for new functionality
- Ensure all tests pass before submitting PR
- Include both unit tests and integration tests where appropriate
- Test edge cases and error conditions

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Documentation

- Update README.md if adding new features or changing behavior
- Add inline documentation for complex code
- Update API documentation for public interfaces
- Include examples where helpful

## Pull Request Process

1. **Update documentation** - Ensure README and other docs are updated
2. **Add tests** - Include tests for new functionality
3. **Run checks** - Ensure `cargo fmt`, `cargo clippy`, and `cargo test` all pass
4. **Update CHANGELOG** - Add your changes to the unreleased section
5. **Request review** - Tag maintainers for review
6. **Address feedback** - Respond to review comments promptly
7. **Squash commits** - If requested, squash commits before merging

## Release Process

Releases are managed by maintainers following semantic versioning:

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create a git tag
4. Push to trigger CI/CD
5. Publish to crates.io

## Getting Help

- **Discord**: Join our community server (if available)
- **Discussions**: Use GitHub Discussions for questions
- **Issues**: For bugs and feature requests

## Recognition

Contributors will be recognized in:
- The project README
- Release notes
- The AUTHORS file (if applicable)

Thank you for contributing to cc2report!