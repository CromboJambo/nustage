# Contributing to Nustage

Thank you for your interest in contributing to Nustage! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Style Guidelines](#code-style-guidelines)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Community Guidelines](#community-guidelines)

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo package manager
- Git
- A code editor (VS Code, vim, neovim, etc.)

### Setting Up Development Environment

1. Fork and clone the repository:
   ```bash
    git clone https://github.com/<org>/nustage.git
   cd nustage
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Run the examples:
   ```bash
   cargo run --example simple_demo
   ```

## Development Workflow

### Branch Naming Convention

- Use descriptive branch names: `feature/your-feature-name` or `fix/your-bug-fix`
- Keep branch names lowercase with hyphens

### Commit Messages

Follow conventional commit format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `refactor:` for code refactoring
- `test:` for test additions or updates
- `chore:` for maintenance tasks

Example:
```bash
git commit -m "feat: add column filtering to transformation pipeline"
```

## Code Style Guidelines

### Rust Style

- Use `rustfmt` to format your code
- Follow the [Rust Style Guide](https://rust-lang.github.io/rust-dev-guide/style-guide.html)
- Add comments for complex logic
- Use meaningful variable and function names

### Example Code Structure

```rust
/// Function documentation
/// 
/// # Arguments
/// * `param` - Description of parameter
///
/// # Returns
/// Result type with description
fn my_function(param: Type) -> Result<Type, Error> {
    // Implementation
    Ok(value)
}
```

### Testing Requirements

- Add unit tests for new functions
- Add integration tests for new features
- Ensure all tests pass before submitting PR
- Test with different data scenarios

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Test implementation
        assert!(result);
    }
}
```

## Pull Request Process

### Before Submitting

1. Ensure all tests pass:
   ```bash
   cargo test --all-features
   ```

2. Run code quality checks:
   ```bash
   cargo clippy -- -D warnings
   cargo fmt -- --check
   ```

3. Update documentation if needed

### PR Template

When creating a pull request, include:

- Clear title and description
- List of changes made
- Link to related issues
- Test results
- Screenshots (if applicable)

### Review Process

1. Address feedback from maintainers
2. Keep PRs focused and small when possible
3. Respond to comments promptly

## Community Guidelines

### Communication

- Be respectful and constructive
- Assume positive intent
- Ask questions when unclear
- Share knowledge with others

### Issue Reporting

When reporting bugs or requesting features:

- Use the issue tracker
- Provide clear reproduction steps
- Include relevant error messages
- Suggest possible solutions

### Code of Conduct

- Be friendly and welcoming
- Be inclusive
- Don't attack or harass others
- Respect community decisions

## Getting Help

- Check existing issues and documentation
- Ask questions in the GitHub Discussions
- Review examples in the `examples/` directory
- Consult the project's README and API documentation

## Thank You!

Your contributions make Nustage better. Thank you for helping build a focused tabular workflow toolkit in Rust.

---

**Note**: This is a work in progress. Guidelines may be updated as the project evolves.
