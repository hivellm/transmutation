<!-- RUST:START -->
# Rust Project Rules

## Agent Automation Commands

**CRITICAL**: Execute these commands after EVERY implementation (see AGENT_AUTOMATION module for full workflow).

```bash
# Complete quality check sequence:
cargo fmt --all -- --check           # Format check
cargo clippy --workspace --all-targets --all-features -- -D warnings  # Lint
cargo test --workspace --all-features  # All tests (100% pass)
cargo build --release                # Build verification
cargo llvm-cov --all                 # Coverage (95%+ required)

# Security audit:
cargo audit                          # Vulnerability scan
cargo outdated                       # Check outdated deps
```

## Rust Edition and Toolchain

**CRITICAL**: Always use Rust Edition 2024 with nightly toolchain.

- **Edition**: 2024
- **Toolchain**: nightly 1.85+
- **Update**: Run `rustup update nightly` regularly

### Formatting

- Use `rustfmt` with nightly toolchain
- Configuration in `rustfmt.toml` or `.rustfmt.toml`
- Always format before committing: `cargo +nightly fmt --all`
- CI must check formatting: `cargo +nightly fmt --all -- --check`

### Linting

- Use `clippy` with `-D warnings` (warnings as errors)
- Fix all clippy warnings before committing
- Acceptable exceptions must be documented with `#[allow(clippy::...)]` and justification
- CI must enforce clippy: `cargo clippy --workspace -- -D warnings`

### Testing

- **Location**: Tests in `/tests` directory for integration tests
- **Unit Tests**: In same file as implementation with `#[cfg(test)]`
- **Coverage**: Must meet project threshold (default 95%)
- **Tools**: Use `cargo-nextest` for faster test execution
- **Async**: Use `tokio::test` for async tests with Tokio runtime

Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test implementation
    }

    #[tokio::test]
    async fn test_async_feature() {
        // Async test implementation
    }
}
```

## Async Programming

**CRITICAL**: Follow Tokio best practices for async code.

- **Runtime**: Use Tokio for async runtime
- **Blocking**: Never block in async context - use `spawn_blocking` for CPU-intensive tasks
- **Channels**: Use `tokio::sync::mpsc` or `tokio::sync::broadcast` for async communication
- **Timeouts**: Always set timeouts for network operations: `tokio::time::timeout`

Example:
```rust
use tokio::time::{timeout, Duration};

async fn fetch_data() -> Result<Data, Error> {
    timeout(Duration::from_secs(30), async {
        // Network operation
    }).await?
}
```

## Dependency Management

**CRITICAL**: Always verify latest versions before adding dependencies.

### Before Adding Any Dependency

1. **Check Context7 for latest version**:
   - Use MCP Context7 tool if available
   - Search for the crate documentation
   - Verify the latest stable version
   - Review breaking changes and migration guides

2. **Example Workflow**:
   ```
   Adding tokio → Check crates.io and docs.rs
   Adding serde → Verify latest version with security updates
   Adding axum → Check for breaking changes in latest version
   ```

3. **Document Version Choice**:
   - Note why specific version chosen in `Cargo.toml` comments
   - Document any compatibility constraints
   - Update CHANGELOG.md with new dependencies

### Dependency Guidelines

- ✅ Use latest stable versions
- ✅ Check for security advisories: `cargo audit`
- ✅ Prefer well-maintained crates (active development, good documentation)
- ✅ Minimize dependency count
- ✅ Use workspace dependencies for monorepos
- ❌ Don't use outdated versions without justification
- ❌ Don't add dependencies without checking latest version

## Codespell Configuration

**CRITICAL**: Use codespell to catch typos in code and documentation.

Install: `pip install 'codespell[toml]'`

Configuration in `pyproject.toml`:
```toml
[tool.codespell]
skip = "*.lock,*.json,target,node_modules,.git"
ignore-words-list = "crate,ser,deser"
```

Or run with flags:
```bash
codespell \
  --skip="*.lock,*.json,target,node_modules,.git" \
  --ignore-words-list="crate,ser,deser"
```

## Error Handling

- Use `Result<T, E>` for recoverable errors
- Use `thiserror` for custom error types
- Use `anyhow` for application-level error handling
- Document error conditions in function docs
- Never use `unwrap()` or `expect()` in production code without justification

Example:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub fn process_data(input: &str) -> Result<Data, MyError> {
    // Implementation
}
```

## Documentation

- **Public APIs**: Must have doc comments (`///`)
- **Examples**: Include examples in doc comments
- **Modules**: Document module purpose with `//!`
- **Unsafe**: Always document safety requirements for `unsafe` code
- **Run doctests**: `cargo test --doc`

Example:
```rust
/// Processes the input data and returns a result.
///
/// # Arguments
///
/// * `input` - The input string to process
///
/// # Examples
///
/// ```
/// use mylib::process;
/// let result = process("hello");
/// assert_eq!(result, "HELLO");
/// ```
///
/// # Errors
///
/// Returns `MyError::InvalidInput` if input is empty.
pub fn process(input: &str) -> Result<String, MyError> {
    // Implementation
}
```

## Project Structure

```
project/
├── Cargo.toml          # Package manifest
├── Cargo.lock          # Dependency lock file (commit this)
├── README.md           # Project overview (allowed in root)
├── CHANGELOG.md        # Version history (allowed in root)
├── AGENTS.md          # AI assistant rules (allowed in root)
├── LICENSE            # Project license (allowed in root)
├── CONTRIBUTING.md    # Contribution guidelines (allowed in root)
├── CODE_OF_CONDUCT.md # Code of conduct (allowed in root)
├── SECURITY.md        # Security policy (allowed in root)
├── src/
│   ├── lib.rs          # Library root (for libraries)
│   ├── main.rs         # Binary root (for applications)
│   └── ...
├── tests/              # Integration tests
├── examples/           # Example code
├── benches/            # Benchmarks
└── docs/               # Project documentation
```

## CI/CD Requirements

Must include GitHub Actions workflows for:

1. **Testing** (`rust-test.yml`):
   - Test on ubuntu-latest, windows-latest, macos-latest
   - Use `cargo-nextest` for fast test execution
   - Upload test results

2. **Linting** (`rust-lint.yml`):
   - Format check: `cargo +nightly fmt --all -- --check`
   - Clippy: `cargo clippy --workspace -- -D warnings`
   - All targets: `cargo clippy --workspace --all-targets -- -D warnings`

3. **Codespell** (`codespell.yml`):
   - Check for typos in code and documentation
   - Fail on errors

## Crate Publication

### Publishing to crates.io

**Prerequisites:**
1. Create account at https://crates.io
2. Generate API token: `cargo login`
3. Add `CARGO_TOKEN` to GitHub repository secrets

**Cargo.toml Configuration:**

```toml
[package]
name = "your-crate-name"
version = "1.0.0"
edition = "2024"
authors = ["Your Name <your.email@example.com>"]
license = "MIT OR Apache-2.0"
description = "A short description of your crate"
documentation = "https://docs.rs/your-crate-name"
homepage = "https://github.com/your-org/your-crate-name"
repository = "https://github.com/your-org/your-crate-name"
readme = "README.md"
keywords = ["your", "keywords", "here"]
categories = ["category"]
exclude = [
    ".github/",
    "tests/",
    "benches/",
    "examples/",
    "*.sh",
]

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

**Publishing Workflow:**

1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Run quality checks:
   ```bash
   cargo fmt --all
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --all-features
   cargo doc --no-deps --all-features
   ```
4. Create git tag: `git tag v1.0.0 && git push --tags`
5. GitHub Actions automatically publishes to crates.io
6. Or manual publish: `cargo publish`

**Publishing Checklist:**

- ✅ All tests passing (`cargo test --all-features`)
- ✅ No clippy warnings (`cargo clippy -- -D warnings`)
- ✅ Code formatted (`cargo fmt --all -- --check`)
- ✅ Documentation builds (`cargo doc --no-deps`)
- ✅ Version updated in Cargo.toml
- ✅ CHANGELOG.md updated
- ✅ README.md up to date
- ✅ LICENSE file present
- ✅ Package size < 10MB (check with `cargo package --list`)
- ✅ Verify with `cargo publish --dry-run`

**Semantic Versioning:**

Follow [SemVer](https://semver.org/) strictly:
- **MAJOR**: Breaking API changes
- **MINOR**: New features (backwards compatible)
- **PATCH**: Bug fixes (backwards compatible)

**Documentation:**

- Use `///` for public API documentation
- Include examples in doc comments
- Use `#![deny(missing_docs)]` for libraries
- Test documentation examples with `cargo test --doc`

```rust
/// Processes the input data and returns a result.
///
/// # Arguments
///
/// * `input` - The input string to process
///
/// # Examples
///
/// ```
/// use your_crate::process;
///
/// let result = process("hello");
/// assert_eq!(result, "HELLO");
/// ```
///
/// # Errors
///
/// Returns an error if the input is empty.
pub fn process(input: &str) -> Result<String, Error> {
    // Implementation
}
```

<!-- RUST:END -->