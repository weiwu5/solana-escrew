# Contributing to Solana BPF Program Template

Thank you for your interest in contributing! This document provides guidelines for contributing to this project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/solana-escrew.git
   cd solana-escrew
   ```
3. **Install dependencies** (see README.md for full instructions):
   ```bash
   make install
   ```

## Development Workflow

### 1. Create a Branch

Create a new branch for your work:
```bash
git checkout -b feature/your-feature-name
```

Branch naming conventions:
- `feature/` - New features
- `bugfix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test improvements

### 2. Make Your Changes

- Write clear, self-documenting code
- Follow Rust best practices and idioms
- Add comments for complex logic
- Update documentation as needed

### 3. Test Your Changes

Run the full test suite:
```bash
make all
```

This will:
- Format your code with `rustfmt`
- Lint with `clippy`
- Build the project
- Run all tests

You can also run individual commands:
```bash
make fmt      # Format code
make lint     # Run linter
make test     # Run tests
make test-bpf # Run BPF integration tests
```

### 4. Commit Your Changes

Write clear, descriptive commit messages:
```bash
git add .
git commit -m "Add feature: brief description"
```

Commit message guidelines:
- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- First line should be 50 characters or less
- Add detailed description in body if needed

Examples:
```
Add instruction parsing for transfer operations

Implement instruction data parsing to handle different
operation types. Add validation for account requirements.
```

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then open a pull request on GitHub with:
- Clear title describing the change
- Description of what changed and why
- Any relevant issue numbers (e.g., "Fixes #123")

## Code Style

### Rust Formatting

- Use `rustfmt` for all code (run `make fmt`)
- Maximum line length: 100 characters
- Use 4 spaces for indentation (no tabs)

### Documentation

- Add doc comments (`///`) for all public functions
- Include examples in doc comments when helpful
- Update README.md if you change user-facing behavior

### Comments

- Use `//` for single-line comments
- Write comments that explain "why", not "what"
- Keep comments up-to-date with code changes

Example:
```rust
// Good: Explains why
// We need to validate ownership before allowing transfers
// to prevent unauthorized account modifications
validate_account_owner(&account)?;

// Bad: Restates what code does
// Check if account owner is valid
validate_account_owner(&account)?;
```

## Testing

### Test Coverage

- Add tests for all new functionality
- Include both unit tests and integration tests
- Test edge cases and error conditions

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // Test implementation
    }

    #[test]
    fn test_feature_error_case() {
        // Test error handling
    }
}
```

### Running Tests

```bash
# Run all tests
make test

# Run BPF integration tests
make test-bpf

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Pull Request Process

1. **Ensure CI passes** - All GitHub Actions checks must pass
2. **Update documentation** - Include relevant changes to README or docs
3. **Add tests** - New features require test coverage
4. **Follow code style** - Run `make fmt` and `make lint`
5. **Squash commits** if needed - Keep history clean
6. **Respond to feedback** - Address reviewer comments promptly

### PR Checklist

Before submitting, verify:

- [ ] Code follows project style guidelines
- [ ] `make all` passes without errors
- [ ] New tests added for new functionality
- [ ] Documentation updated (README, code comments)
- [ ] Commit messages are clear and descriptive
- [ ] No unrelated changes included
- [ ] Branch is up-to-date with main

## Code Review

### For Contributors

- Be receptive to feedback
- Ask questions if requirements are unclear
- Update PR based on reviewer comments
- Be patient - reviews take time

### For Reviewers

- Be respectful and constructive
- Explain the "why" behind suggestions
- Approve when requirements are met
- Suggest improvements but don't bikeshed

## Common Tasks

### Adding a New Feature

1. Create a feature branch
2. Implement the feature with tests
3. Update documentation
4. Run `make all` to verify
5. Submit pull request

### Fixing a Bug

1. Create a bugfix branch
2. Write a test that reproduces the bug
3. Fix the bug
4. Verify test now passes
5. Submit pull request

### Updating Dependencies

```bash
# Update all dependencies
cargo update

# Update specific dependency
cargo update solana-program

# Check for outdated dependencies
cargo outdated
```

### Running Security Audit

```bash
make audit
```

## Project Structure

```
solana-escrew/
├── src/
│   └── lib.rs              # Main program code
├── tests/
│   └── integration.rs      # Integration tests
├── scripts/
│   ├── patch.crates-io.sh
│   └── update-solana-dependencies.sh
├── .github/
│   └── workflows/
│       └── ci.yml          # GitHub Actions CI
├── Cargo.toml              # Dependencies and metadata
├── Makefile                # Development commands
├── README.md               # Project documentation
└── CONTRIBUTING.md         # This file
```

## Getting Help

- Check the [README](README.md) for setup instructions
- Review [Solana documentation](https://docs.solana.com/)
- Ask questions in GitHub Issues
- Join the [Solana Discord](https://discord.gg/solana)

## License

By contributing, you agree that your contributions will be licensed under the WTFPL license.

## Questions?

Feel free to open an issue for questions or clarifications!
