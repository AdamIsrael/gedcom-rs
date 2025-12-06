# gedcom-rs

[![Continuous integration](https://github.com/AdamIsrael/rust-gedcom/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/AdamIsrael/rust-gedcom/actions/workflows/ci.yaml)

A Rust library for parsing [GEDCOM 5.5.1](https://gedcom.io/specifications/ged551.pdf) genealogical data files.

**Status:** This library is a work in progress. While basic parsing functionality is implemented, it is not feature-complete. Use with caution in production environments. Contributions are welcome!

## Features

- ✅ Parse GEDCOM 5.5.1 files
- ✅ Support for multiple character encodings (UTF-8, ASCII, ANSI/Windows-1252, ANSEL*)
- ✅ Automatic encoding detection from GEDCOM header
- ✅ Individual (INDI) record parsing
- ✅ Header (HEAD) metadata parsing
- ⚠️ Family (FAM) records recognized but not fully parsed
- ⚠️ Source (SOUR), Repository (REPO), and Multimedia (OBJE) records recognized but not parsed

## Installation

### From crates.io (Stable Release)

Add this to your `Cargo.toml`:

```toml
[dependencies]
gedcom-rs = "0.1"
```

### From Git (Latest Development Version)

To use the latest development version directly from GitHub:

```toml
[dependencies]
gedcom-rs = { git = "https://github.com/AdamIsrael/gedcom-rs" }
```

You can also specify a particular branch:

```toml
[dependencies]
# Use the main branch
gedcom-rs = { git = "https://github.com/AdamIsrael/gedcom-rs", branch = "main" }

# Or use a specific branch like charset
gedcom-rs = { git = "https://github.com/AdamIsrael/gedcom-rs", branch = "charset" }
```

Or a specific commit:

```toml
[dependencies]
gedcom-rs = { git = "https://github.com/AdamIsrael/gedcom-rs", rev = "7b53fde" }
```

**Note:** Development versions may contain breaking changes or incomplete features. Use the stable crates.io release for production applications.

## Usage

### Basic Parsing

```rust
use gedcom_rs::parse::{parse_gedcom, GedcomConfig};

fn main() {
    match parse_gedcom("path/to/your/file.ged", &GedcomConfig::new()) {
        Ok(gedcom) => {
            println!("Parsed {} individuals", gedcom.individuals.len());
            for individual in &gedcom.individuals {
                if let Some(name) = individual.names.first() {
                    if let Some(value) = &name.name.value {
                        println!("  {}", value);
                    }
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Verbose Mode (Encoding Diagnostics)

For detailed encoding warnings, especially useful for ANSEL files:

```rust
use gedcom_rs::parse::{parse_gedcom, GedcomConfig};

fn main() {
    // Enable verbose mode for detailed encoding warnings
    let config = GedcomConfig::new().verbose();
    
    match parse_gedcom("path/to/file.ged", &config) {
        Ok(gedcom) => println!("Parsed successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Command Line Tool

```bash
# Basic usage
cargo run --bin gedcom-rs path/to/file.ged

# With verbose encoding warnings
cargo run --bin gedcom-rs --verbose path/to/file.ged
```

### Examples

The library includes several examples demonstrating different features:

```bash
# Basic parsing and statistics
cargo run --example basic_parse data/complete.ged

# Search for individuals by name
cargo run --example find_person data/complete.ged "Smith"

# Configuration options
cargo run --example config
```

See the `examples/` directory for more detailed usage patterns.

## Known Limitations

### ANSEL Character Encoding

The library currently approximates ANSEL encoding (ANSI/NISO Z39.47-1993) using Windows-1252, which may cause:

- Accented characters (é, ñ, ü, etc.) to display incorrectly
- Loss of combining diacritical marks
- Incorrect representation of special genealogical symbols

**Workaround:** If you have control over the GEDCOM file, consider converting it to UTF-8 using a GEDCOM editor.

**Technical Details:** See [docs/ENCODING.md](docs/ENCODING.md) for a comprehensive explanation of ANSEL encoding and its limitations.

**Tracking:** Full ANSEL support is tracked in issue [#TBD](https://github.com/adamgiacomelli/gedcom-rs/issues/TBD)

### Incomplete Parsing

The following record types are recognized but not yet fully parsed:
- Family records (FAM)
- Source records (SOUR)
- Repository records (REPO)
- Multimedia records (OBJE)
- Note records (NOTE)

These records are silently skipped during parsing. Contributions to implement these are welcome!

## Supported Character Encodings

| Encoding | Support Level | Notes |
|----------|--------------|-------|
| UTF-8 | ✅ Full | Recommended for new files |
| ASCII | ✅ Full | Subset of UTF-8 |
| ANSI (Windows-1252) | ✅ Full | Common in Western genealogy software |
| ANSEL | ⚠️ Partial | Approximated with Windows-1252; see limitations above |
| UTF-16 | ⚠️ Untested | May work but not thoroughly tested |

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Benchmarks

```bash
cargo bench
```

### Linting and Formatting

```bash
# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Run all CI checks
make test
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Areas where help is especially appreciated:
- Full ANSEL encoding support
- Parsing FAM, SOUR, REPO, OBJE, and NOTE records
- Additional test cases and GEDCOM sample files
- Documentation improvements

For a detailed breakdown of planned features and implementation status, see [docs/ROADMAP.md](docs/ROADMAP.md).

## Copyright

While this library is open source under the MIT license, `data/complete.ged`, used for testing, is © 1997 by H. Eichmann, parts © 1999-2000 by J. A. Nairn.
