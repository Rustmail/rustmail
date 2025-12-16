# Contributing Guide

Thank you for your interest in contributing to Rustmail. This guide explains how to contribute effectively.

---

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Set up the development environment (see [Building](building.md))
4. Create a branch for your changes

```bash
git clone https://github.com/YOUR_USERNAME/rustmail.git
cd rustmail
git checkout -b feature/your-feature-name
```

---

## Development Workflow

### 1. Find or Create an Issue

- Check existing issues for something to work on
- For new features, open an issue first to discuss the approach
- Bug fixes can go directly to a pull request

### 2. Make Changes

- Write clean, readable code
- Follow existing patterns in the codebase
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run tests
cargo test --workspace

# Check formatting
cargo fmt --all --check

# Run linter
cargo clippy --workspace
```

### 4. Commit

Write clear commit messages:

```
feat: add snippet management command

- Add /snippet command for using saved responses
- Add snippet CRUD operations in database
- Include tests for snippet operations
```

Commit message prefixes:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `test:` - Test additions/changes
- `chore:` - Build/tooling changes

### 5. Submit Pull Request

- Push your branch to your fork
- Open a pull request against `main`
- Fill out the PR template
- Wait for review

---

## Code Style

### Rust

- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Address `cargo clippy` warnings
- Prefer explicit types over inference when it aids readability

```rust
// Good
let thread_id: u64 = message.channel_id.get();

// Also acceptable when obvious
let content = message.content.clone();
```

### Documentation

- Document public APIs with doc comments
- Include examples for complex functions
- Keep comments current with code

```rust
/// Creates a new ticket for the specified user.
///
/// # Arguments
///
/// * `user_id` - Discord user ID
/// * `initial_message` - Optional first message content
///
/// # Returns
///
/// The created thread's channel ID.
pub async fn create_ticket(user_id: u64, initial_message: Option<&str>) -> Result<u64> {
    // ...
}
```

---

## Adding Features

### New Commands

1. Create a module in `rustmail/src/commands/`
2. Implement the command handler
3. Add slash command definition
4. Add text command parser
5. Register in `commands/mod.rs`
6. Add to documentation

### New API Endpoints

1. Create handler in `rustmail/src/api/handler/`
2. Define route in `rustmail/src/api/routes/`
3. Add authentication/authorization as needed
4. Document in `docs/reference/api.md`

### Database Changes

1. Create migration in `migrations/`
2. Update relevant structs in `rustmail_types`
3. Add database functions
4. Test migration up and down

---

## Pull Request Guidelines

### Before Submitting

- [ ] Code compiles without errors
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings
- [ ] Documentation updated
- [ ] Commit messages are clear

### PR Description

Include:
- What the change does
- Why it's needed
- How to test it
- Any breaking changes

### Review Process

1. Maintainers will review the PR
2. Address feedback with additional commits
3. Once approved, the PR will be merged

---

## Testing

### Unit Tests

Place tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("1h"), Some(3600));
        assert_eq!(parse_duration("30m"), Some(1800));
    }
}
```

### Integration Tests

For tests requiring multiple components, use `tests/` directory.

### Running Specific Tests

```bash
# Single test
cargo test test_name

# Tests in a module
cargo test module_name::

# With output
cargo test -- --nocapture
```

---

## Translations

### Adding a New Language

1. Add variant to `Language` enum in `rustmail/src/i18n/`
2. Create translation module
3. Add JSON file for panel in `rustmail_panel/src/i18n/translations/`
4. Test all strings render correctly

### Updating Translations

- Keep all languages in sync
- Use English as the source
- Maintain consistency in terminology

---

## Reporting Issues

### Bug Reports

Include:
- Rustmail version
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs

### Feature Requests

Include:
- Use case description
- Proposed solution
- Alternative approaches considered

---

## Communication

- GitHub Issues - Bug reports, feature requests
- GitHub Discussions - Questions, ideas
- Discord Server - Real-time chat

---

## License

By contributing, you agree that your contributions will be licensed under the AGPLv3 license.

---

## Recognition

Contributors are recognized in:
- Git history
- Release notes for significant contributions
- README acknowledgments for major features
