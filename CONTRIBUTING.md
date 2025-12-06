# Contributing to gedcom-rs

Thank you for your interest in contributing to gedcom-rs! This document provides guidelines and information for contributors.

## Ways to Contribute

### 1. Report Issues

- **Bugs**: Report parsing failures, incorrect behavior, or crashes
- **Encoding Issues**: Especially ANSEL-related character encoding problems
- **Missing Features**: Request support for additional GEDCOM record types
- **Documentation**: Point out unclear or incorrect documentation

When reporting issues:
- Include the GEDCOM specification section if applicable
- Provide a minimal GEDCOM sample that reproduces the issue (if possible)
- Describe expected vs. actual behavior
- Include your environment (OS, Rust version)

### 2. Submit Pull Requests

We welcome pull requests for:
- Bug fixes
- New features (discuss large features in an issue first)
- Documentation improvements
- Test coverage improvements
- Performance optimizations

## Development Setup

### Prerequisites

- Rust 2018 edition or later
- Cargo

### Getting Started

```bash
# Clone the repository
git clone https://github.com/AdamIsrael/gedcom-rs.git
cd gedcom-rs

# Build the project
cargo build

# Run tests
cargo test

# Run examples
cargo run --example basic_parse data/complete.ged
cargo run --example find_person data/complete.ged "Smith"
```

## Code Style Guidelines

This project follows standard Rust conventions plus some additional guidelines:

### General Rust Style

- **Edition**: Rust 2018
- **Formatting**: Use `cargo fmt` (enforced in CI)
- **Linting**: All `clippy` warnings treated as errors: `cargo clippy -- -D warnings`
- **Naming**:
  - `snake_case` for functions and variables
  - `PascalCase` for types and structs
  - `SCREAMING_SNAKE_CASE` for constants

### Module Organization

- **Types**: Organize in `src/types/` directory
- **Re-exports**: Use `mod.rs` with `pub use` for clean public API
- **Submodules**: Group related types (e.g., `individual/` for Individual-related types)

### Parser Guidelines

- **Library**: Uses `winnow` 0.5.40 for parsing
- **Pattern**: `parse_next(input)` with mutable string slices
- **Lifetimes**: Use `'a`, `'b` for borrowed data in parsers
- **Return Type**: `PResult<T>` for parser functions

### Error Handling

- **Parsers**: Return `PResult<T>` from `winnow`
- **API Functions**: Return `Result<T, GedcomError>` or `Option<T>`
- **Unwrap**: NEVER use `.unwrap()` or `.expect()` in production code
  - Tests are allowed to use `unwrap` (marked with `#[allow(clippy::unwrap_used)]`)

### Type Design

```rust
// Prefer #[derive(Debug, Default)]
#[derive(Debug, Default)]
pub struct MyType {
    pub field: String,
}

// Common events use Vec
pub birth: Vec<Birth>,

// Rare events use Option<Vec<T>> to save memory
pub baptism: Option<Vec<EventDetail>>,
```

### Testing

- **Location**: Tests typically inline in implementation files
- **Attribute**: Use `#[test]` for test functions
- **Coverage**: Add tests for new functionality
- **Data**: Use test GEDCOM files from `data/` directory

## Testing Your Changes

### Run All Checks

```bash
# Format check
cargo fmt --check

# Linting
cargo clippy -- -D warnings

# All tests
cargo test

# Benchmarks (optional)
cargo bench

# Or use the Makefile
make test
```

### Test with Real GEDCOM Files

```bash
# Test with provided samples
cargo run --bin gedcom-rs data/complete.ged
cargo run --bin gedcom-rs data/TGC551LF.ged

# Test with verbose mode (for encoding issues)
cargo run --bin gedcom-rs --verbose data/TGC551LF.ged
```

## Priority Areas for Contribution

The following areas especially need help:

### 1. Full ANSEL Encoding Support ⭐

**Status**: Currently approximated with Windows-1252

**What's needed**:
- Stateful byte-level parser for ANSEL prefix diacritics
- Complete character mapping table (ANSEL → Unicode)
- Character reordering logic (prefix → suffix combining marks)
- Test cases with real ANSEL-encoded GEDCOM files

**Resources**:
- See `docs/ENCODING.md` for detailed technical information
- ANSI/NISO Z39.47-1993 specification
- Tracking issue: #[TBD]

### 2. Parse Additional Record Types

**What's needed**:
- Family records (FAM) - currently recognized but not parsed
- Source records (SOUR)
- Repository records (REPO)
- Multimedia records (OBJE)
- Note records (NOTE)

**Example**:
Look at `src/types/individual/` for examples of parsing individual records.

### 3. Test Coverage

**What's needed**:
- Additional GEDCOM sample files (especially edge cases)
- Tests for error conditions
- Tests for all character encodings
- Performance benchmarks for large files

### 4. Documentation

**What's needed**:
- More examples in `examples/`
- API documentation improvements
- Tutorial/guide for common use cases
- Better error messages

## Pull Request Process

1. **Fork the repository** and create a new branch for your feature
   ```bash
   git checkout -b feature/my-new-feature
   ```

2. **Make your changes** following the code style guidelines above

3. **Add tests** for new functionality

4. **Run all checks** (formatting, linting, tests)
   ```bash
   make test  # or run individual commands
   ```

5. **Update documentation**:
   - Add/update doc comments for new/modified public APIs
   - Update README if adding major features
   - Add examples if appropriate

6. **Commit your changes** with clear, descriptive commit messages
   ```bash
   git commit -m "Add support for FAM record parsing"
   ```

7. **Push to your fork** and submit a pull request
   ```bash
   git push origin feature/my-new-feature
   ```

8. **In your PR description**:
   - Explain what changes you made and why
   - Reference any related issues
   - Include example usage if adding a feature
   - Note any breaking changes

## Code Review

- Maintainers will review your PR and may request changes
- Address feedback by pushing additional commits
- Once approved, a maintainer will merge your PR

## Questions?

- Open an issue for questions about contributing
- Check existing issues and PRs for similar work
- Be respectful and constructive in all interactions

## License

By contributing to gedcom-rs, you agree that your contributions will be licensed under the MIT License (same as the project).

## Thank You!

Your contributions help make genealogical data more accessible to everyone. Thank you for being part of this project!
