# Contributing to Scat

Thank you for your interest in contributing to Scat. This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful and professional in all interactions with other contributors and maintainers.

## Getting Started

### Prerequisites

- Rust 1.70+ (check with `rustc --version`)
- Cargo (comes with Rust)

### Setting Up Development Environment

```bash
git clone https://github.com/yourusername/scat.git
cd scat
cargo build
cargo test
```

### Building & Testing

```bash
# Build debug version
cargo build

# Build release version (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging (if implemented)
RUST_LOG=debug cargo run -- /dev/ttyUSB0

# Run clippy linter
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

Use descriptive branch names:
- `feature/` for new features
- `fix/` for bug fixes
- `docs/` for documentation improvements
- `refactor/` for code refactoring

### 2. Make Changes

- Keep commits atomic and logical
- Write descriptive commit messages
- Follow the code style (see below)
- Add tests for new functionality
- Update documentation as needed

### 3. Code Style

Follow Rust conventions:

```bash
# Format your code
cargo fmt

# Check for linting issues
cargo clippy -- -D warnings
```

**Guidelines:**
- Use meaningful variable names
- Keep functions small and focused
- Document public APIs with doc comments
- Use type hints for clarity

Example of a well-documented function:

```rust
/// Opens a serial port with the specified configuration.
///
/// # Arguments
///
/// * `port_name` - The serial port device path (e.g., `/dev/ttyUSB0`)
/// * `baud_rate` - Communication speed in bits per second
/// * `data_bits` - Number of data bits (5-8)
/// * `stop_bits` - Number of stop bits (1-2)
///
/// # Returns
///
/// A boxed SerialPort instance or an error
///
/// # Errors
///
/// Returns an error if the port cannot be opened or parameters are invalid
pub fn open_port(
    port_name: &str,
    baud_rate: u32,
    data_bits: u8,
    stop_bits: u8,
) -> Result<Box<dyn SerialPort>> {
    // implementation
}
```

### 4. Commit Message Format

Use clear, descriptive commit messages:

```
feat: add hex dump view mode

- Implement HexDumpView struct
- Add toggle command Ctrl+H
- Include colored bytes display
- Update documentation

Fixes #42
```

**Prefixes:**
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `refactor:` - Code reorganization
- `test:` - Test additions/modifications
- `chore:` - Dependencies, build, CI

### 5. Testing

- Write tests for new features
- Ensure all tests pass: `cargo test`
- Test on multiple platforms if possible

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_baud_rate_valid() {
        assert_eq!(parse_baud_rate(115200).unwrap(), 115200);
    }

    #[test]
    fn test_parse_baud_rate_invalid() {
        assert!(parse_baud_rate(0).is_err());
    }
}
```

### 6. Documentation

- Update README.md for user-facing changes
- Update DEVELOPMENT_PLAN.md if adding/modifying phases
- Add inline doc comments for complex logic
- Update CHANGELOG (if exists)

### 7. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Create a pull request on GitHub with:
- Clear title describing the change
- Description of what changed and why
- Reference to related issues (e.g., `Fixes #42`)
- Steps to test the changes
- Any breaking changes noted

**PR Template:**

```markdown
## Description
Brief description of changes

## Motivation & Context
Why is this change needed? What problem does it solve?

## Testing
How was this tested?
- [ ] Unit tests added/updated
- [ ] Manual testing performed
- [ ] Tested on: (Linux/macOS/Windows)

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex logic
- [ ] Documentation updated
- [ ] No new warnings from clippy
- [ ] Tests pass

## Related Issues
Fixes #<issue_number>
```

## Reporting Bugs

Create an issue with:

1. **Title**: Clear, concise description
2. **Description**: What happened?
3. **Reproduction**: Steps to reproduce
4. **Expected**: What should happen
5. **Actual**: What actually happened
6. **Environment**:
   - OS and version
   - Rust version (`rustc --version`)
   - Scat version
   - Serial device type (e.g., CH340, FT232)
   - Baud rate

Example:

```markdown
## Data loss at high baud rates

### Expected
Transfer 1MB of data at 921600 baud without errors

### Actual
After ~50KB, data starts being dropped

### Steps to Reproduce
1. Connect to device with 921600 baud rate
2. Send large file (1MB+)
3. Observe garbled output

### Environment
- OS: Linux (Ubuntu 22.04)
- Rust: 1.70.0
- Device: CH340 USB-UART adapter
```

## Feature Requests

Describe the feature clearly:

1. **Use case**: Why is this needed?
2. **Proposed solution**: How should it work?
3. **Alternatives**: Are there other approaches?
4. **Additional context**: Screenshots, examples, links

## Development Phases

Refer to [DEVELOPMENT_PLAN.md](DEVELOPMENT_PLAN.md) for current priorities:

- **Phase 1 (MVP)**: Core serial communication
- **Phase 2 (UX)**: Enhanced user experience
- **Phase 3 (Features)**: Power-user capabilities

Contributions aligning with the next planned phase are especially welcome.

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0 (see LICENSE file).

## Questions?

- Check existing issues and discussions
- Review the development plan
- Consult the code documentation
- Ask in a new issue with the `question` label

## Thanks!

Your contributions make Scat better for hardware developers everywhere.
