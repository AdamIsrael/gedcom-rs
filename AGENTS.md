# Agent Guidelines for gedcom-rs

## Build, Lint, and Test Commands
- Build: `cargo build`
- Test all: `cargo test`
- Test single: `cargo test test_name`
- Format: `cargo fmt`
- Lint: `cargo clippy -- -D warnings`
- Benchmarks: `cargo bench`
- Full CI check: `make test` (runs test, fmt, clippy)

## Code Style Guidelines
- **Edition**: Rust 2018
- **Imports**: Use `use super::types::*;` for module re-exports; prefer specific imports otherwise
- **Formatting**: Use `cargo fmt` (enforced in CI with `--check`)
- **Linting**: All clippy warnings treated as errors (`-D warnings`)
- **Types**: Prefer structs with `#[derive(Debug, Default)]`; use lifetimes (`'a`, `'b`) for borrowed data in parsers
- **Naming**: snake_case for functions/variables, PascalCase for types, SCREAMING_SNAKE_CASE for constants
- **Error Handling**: Use `PResult<T>` from winnow parser combinators; use `Result` or `Option` for API functions
- **Parser Library**: Uses `winnow` 0.5.40 for parsing; use `parse_next(input)` pattern with mutable string slices
- **Modules**: Organize types in `src/types/`, use `mod.rs` for re-exports with `pub use`
- **Testing**: Use `#[test]` attribute; tests typically inline in implementation files
