//! Configuration example for gedcom-rs
//!
//! This example demonstrates various ways to configure the GEDCOM parser.
//!
//! Usage:
//!   cargo run --example config

use gedcom_rs::parse::{parse_gedcom, GedcomConfig};

fn main() {
    let filename = "data/complete.ged";

    // Example 1: Default configuration (no verbose output)
    println!("=== Example 1: Default Configuration ===");
    match parse_gedcom(filename, &GedcomConfig::new()) {
        Ok(gedcom) => {
            println!(
                "Parsed {} individuals (quiet mode)",
                gedcom.individuals.len()
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    println!();

    // Example 2: Verbose configuration (shows encoding details)
    println!("=== Example 2: Verbose Configuration ===");
    let config = GedcomConfig::new().verbose();
    match parse_gedcom(filename, &config) {
        Ok(gedcom) => {
            println!(
                "Parsed {} individuals (verbose mode)",
                gedcom.individuals.len()
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    println!();

    // Example 3: Builder pattern for more complex configurations (future extensibility)
    println!("=== Example 3: Builder Pattern ===");
    let config = GedcomConfig::new().verbose(); // Can chain additional configuration options in the future

    match parse_gedcom(filename, &config) {
        Ok(gedcom) => {
            println!("Configuration applied successfully");
            println!("Total individuals: {}", gedcom.individuals.len());
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    println!();

    // Example 4: Manual configuration
    println!("=== Example 4: Manual Configuration ===");
    let mut config = GedcomConfig::new();
    config.verbose = true;

    match parse_gedcom(filename, &config) {
        Ok(gedcom) => {
            println!("Manually configured parser");
            println!("Total individuals: {}", gedcom.individuals.len());
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
