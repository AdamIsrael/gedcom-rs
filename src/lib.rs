//! # gedcom-rs
//!
//! A Rust library for parsing GEDCOM (Genealogical Data Communication) 5.5.1 files.
//!
//! GEDCOM is the most widely used file format for exchanging genealogical data between
//! different family history applications. This library provides a parser for reading
//! GEDCOM files and extracting structured data about individuals, families, and sources.
//!
//! ## Quick Start
//!
//! ```no_run
//! use gedcom_rs::parse::{parse_gedcom, GedcomConfig};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Parse a GEDCOM file with default configuration
//!     let gedcom = parse_gedcom("path/to/your/file.ged", &GedcomConfig::new())?;
//!     
//!     // Access individuals
//!     println!("Found {} individuals", gedcom.individuals.len());
//!     for individual in &gedcom.individuals {
//!         if let Some(name) = individual.names.first() {
//!             if let Some(value) = &name.name.value {
//!                 println!("  {}", value);
//!             }
//!         }
//!     }
//!     
//!     // Access families
//!     println!("Found {} families", gedcom.families.len());
//!     for family in &gedcom.families {
//!         println!("Family {}", family.xref);
//!         if let Some(ref husband) = family.husband {
//!             println!("  Husband: {}", husband);
//!         }
//!         if let Some(ref wife) = family.wife {
//!             println!("  Wife: {}", wife);
//!         }
//!         println!("  Children: {}", family.children.len());
//!     }
//!     
//!     // Access sources
//!     println!("Found {} sources", gedcom.sources.len());
//!     for source in &gedcom.sources {
//!         if let Some(ref xref) = source.xref {
//!             println!("Source {}", xref);
//!         }
//!         if let Some(ref title) = source.title {
//!             println!("  Title: {}", title);
//!         }
//!     }
//!     
//!     // Access notes
//!     println!("Found {} notes", gedcom.notes.len());
//!     for note in &gedcom.notes {
//!         if let Some(ref xref) = note.xref {
//!             println!("Note {}", xref);
//!         }
//!         println!("  {}", note.note);
//!     }
//!     
//!     // Access multimedia
//!     println!("Found {} multimedia records", gedcom.multimedia.len());
//!     for media in &gedcom.multimedia {
//!         if let Some(ref xref) = media.xref {
//!             println!("Multimedia {}", xref);
//!         }
//!         for file in &media.files {
//!             println!("  File: {}", file.file_reference);
//!         }
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Character Encoding Support
//!
//! The library automatically detects and handles multiple character encodings:
//!
//! - **UTF-8**: Full support (recommended for new files)
//! - **ASCII**: Full support (subset of UTF-8)
//! - **ANSI/Windows-1252**: Full support
//! - **ANSEL**: Partial support (approximated with Windows-1252)
//!
//! For files using ANSEL encoding, use verbose mode to see detailed warnings:
//!
//! ```no_run
//! use gedcom_rs::parse::{parse_gedcom, GedcomConfig};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Parse with verbose warnings about encoding issues
//!     let config = GedcomConfig::new().verbose();
//!     let gedcom = parse_gedcom("path/to/ansel.ged", &config)?;
//!     Ok(())
//! }
//! ```
//!
//! ## Current Limitations
//!
//! This library is a work in progress. Currently implemented:
//!
//! - ✅ Header (HEAD) record parsing
//! - ✅ Individual (INDI) record parsing
//! - ✅ Submitter (SUBM) record parsing
//! - ✅ Family (FAM) record parsing
//! - ✅ Source (SOUR) record parsing
//! - ✅ Note (NOTE) record parsing
//! - ✅ Multimedia (OBJE) record parsing
//! - ⚠️ Repository (REPO) records recognized but not parsed
//!
//! ### ANSEL Encoding Limitation
//!
//! ANSEL (ANSI/NISO Z39.47-1993) is a specialized character set used in genealogy that
//! uses prefix diacritics. The current implementation approximates ANSEL using Windows-1252,
//! which may cause accented characters and special symbols to display incorrectly.
//!
//! See the [README](https://github.com/adamgiacomelli/gedcom-rs#known-limitations) for more details.
//!
//! ## Modules
//!
//! - [`parse`] - Functions for parsing GEDCOM files and configuration options
//! - [`types`] - GEDCOM data structures (Header, Individual, Family, etc.)
//! - [`error`] - Error types returned by the parser
//!
//! ## GEDCOM Specification
//!
//! This library implements the [GEDCOM 5.5.1 specification](https://gedcom.io/specifications/ged551.pdf).

pub mod error;
pub mod parse;
pub mod types;
