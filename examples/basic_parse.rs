//! Basic GEDCOM parsing example
//!
//! This example demonstrates how to parse a GEDCOM file and access
//! basic information about the genealogy data.
//!
//! Usage:
//!   cargo run --example basic_parse path/to/file.ged

use gedcom_rs::parse::{parse_gedcom, GedcomConfig};
use std::env;
use std::process;

fn main() {
    // Get filename from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <gedcom-file>", args[0]);
        eprintln!("\nExample:");
        eprintln!("  cargo run --example basic_parse data/complete.ged");
        process::exit(1);
    }

    let filename = &args[1];

    // Parse the GEDCOM file with default configuration
    match parse_gedcom(filename, &GedcomConfig::new()) {
        Ok(gedcom) => {
            println!("Successfully parsed: {}", filename);
            println!();

            // Display header information
            println!("=== Header Information ===");

            if let Some(source) = &gedcom.header.source {
                println!("Source: {}", source.name.as_deref().unwrap_or("Unknown"));
                if let Some(version) = &source.version {
                    println!("Version: {}", version);
                }
                if let Some(corp) = &source.corporation {
                    if let Some(name) = &corp.name {
                        println!("Corporation: {}", name);
                    }
                }
            }

            if let Some(date) = &gedcom.header.date {
                println!("Date: {:?}", date);
            }

            if let Some(char_set) = &gedcom.header.character_set {
                println!("Character Set: {:?}", char_set);
            }

            if let Some(copyright) = &gedcom.header.copyright {
                println!("Copyright: {}", copyright);
            }

            println!();

            // Display statistics
            println!("=== Statistics ===");
            println!("Total individuals: {}", gedcom.individuals.len());

            // Count individuals with names
            let named_count = gedcom
                .individuals
                .iter()
                .filter(|i| !i.names.is_empty())
                .count();
            println!("Named individuals: {}", named_count);

            // Count individuals with birth dates
            let birth_count = gedcom
                .individuals
                .iter()
                .filter(|i| !i.birth.is_empty())
                .count();
            println!("With birth information: {}", birth_count);

            // Count individuals with death dates
            let death_count = gedcom
                .individuals
                .iter()
                .filter(|i| !i.death.is_empty())
                .count();
            println!("With death information: {}", death_count);

            println!();

            // Display first 10 individuals
            println!("=== Sample Individuals (first 10) ===");
            for (i, individual) in gedcom.individuals.iter().take(10).enumerate() {
                print!("{}. ", i + 1);

                if let Some(name) = individual.names.first() {
                    if let Some(value) = &name.name.value {
                        print!("{}", value);
                    } else {
                        print!("(no name value)");
                    }
                } else {
                    print!("(unnamed)");
                }

                if let Some(xref) = &individual.xref {
                    print!(" [{:?}]", xref);
                }

                // Add birth year if available
                if let Some(birth) = individual.birth.first() {
                    if let Some(date) = &birth.event.detail.date {
                        print!(" b.{}", date);
                    }
                }

                // Add death year if available
                if let Some(death) = individual.death.first() {
                    if let Some(event) = &death.event {
                        if let Some(date) = &event.date {
                            print!(" d.{}", date);
                        }
                    }
                }

                println!();
            }

            if gedcom.individuals.len() > 10 {
                println!("... and {} more", gedcom.individuals.len() - 10);
            }
        }
        Err(e) => {
            eprintln!("Error parsing GEDCOM file: {}", e);
            process::exit(1);
        }
    }
}
