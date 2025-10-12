# Contributing to Transmutation

Thank you for your interest in contributing to Transmutation! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for everyone.

## Getting Started

### Prerequisites

- Rust nightly (1.85+)
- Git
- Familiarity with async Rust

### Development Setup

1. Clone the repository:
```bash
git clone https://github.com/hivellm/transmutation.git
cd transmutation
```

2. Install Rust nightly:
```bash
rustup install nightly
rustup default nightly
```

3. Build the project:
```bash
cargo build
```

4. Run tests:
```bash
cargo test
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 2. Make Changes

- Write clear, documented code
- Follow Rust conventions and idioms
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with all features
cargo test --all-features

# Run benchmarks
cargo bench
```

### 4. Format and Lint

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy --all-features --all-targets -- -D warnings
```

### 5. Commit Changes

Follow conventional commits format:
```
feat: add support for EPUB format
fix: resolve memory leak in PDF parser
docs: update installation instructions
test: add integration tests for DOCX
perf: optimize Markdown generation
refactor: simplify error handling
```

### 6. Push and Create PR

```bash
git push origin your-branch-name
```

Then create a Pull Request on GitHub.

## Code Style

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` with the project's configuration
- Maximum line length: 100 characters
- Use meaningful variable names
- Add documentation comments for public APIs

### Documentation

- Document all public APIs with `///` comments
- Include examples in documentation
- Explain complex algorithms or logic
- Keep comments up to date with code

## Testing

### Unit Tests

- Place tests in the same file as the code they test
- Use the `#[cfg(test)]` attribute
- Test edge cases and error conditions
- Aim for >80% code coverage

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test implementation
    }
}
```

### Integration Tests

- Place integration tests in `tests/` directory
- Test end-to-end workflows
- Use realistic test fixtures

### Benchmarks

- Add benchmarks for performance-critical code
- Use criterion for benchmarking
- Document performance expectations

## Adding New Features

### New File Format Support

1. Create converter module in `src/converters/`
2. Implement `DocumentConverter` trait
3. Add format to `FileFormat` enum
4. Add tests with sample files
5. Update documentation

### New Output Format

1. Add variant to `OutputFormat` enum
2. Implement generator in `src/output/`
3. Add conversion logic
4. Add tests
5. Update documentation

## Project Structure

```
transmutation/
├── src/
│   ├── bin/              # CLI application
│   ├── converters/       # Format-specific converters
│   ├── engines/          # Core parsing engines
│   ├── output/           # Output generators
│   ├── types.rs          # Core type definitions
│   ├── error.rs          # Error types
│   └── lib.rs            # Library entry point
├── tests/                # Integration tests
├── benches/              # Benchmarks
├── docs/                 # Documentation
└── examples/             # Usage examples
```

## Performance Considerations

- Profile before optimizing
- Use `cargo flamegraph` for performance analysis
- Minimize allocations in hot paths
- Use appropriate data structures
- Consider parallel processing with `rayon`

## Documentation

### API Documentation

```bash
# Generate and open docs
cargo doc --open --all-features
```

### User Documentation

- Update `README.md` for user-facing changes
- Add examples to `examples/` directory
- Update `docs/` for major features

## Release Process

1. Update `CHANGELOG.md`
2. Bump version in `Cargo.toml`
3. Create git tag: `git tag -a v0.x.0 -m "Release v0.x.0"`
4. Push tag: `git push origin v0.x.0`
5. GitHub Actions will handle the rest

## Getting Help

- Open an issue for questions
- Join discussions in GitHub Discussions
- Check existing issues and PRs

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be recognized in the project's README and release notes.

Thank you for contributing to Transmutation! 🚀

