//! Find and search for individuals in a GEDCOM file
//!
//! This example demonstrates how to search for individuals by name,
//! date, or other criteria in a parsed GEDCOM file.
//!
//! Usage:
//!   cargo run --example find_person path/to/file.ged "search term"

use gedcom_rs::parse::{parse_gedcom, GedcomConfig};
use std::env;
use std::process;

fn main() {
    // Get filename and search term from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <gedcom-file> <search-term>", args[0]);
        eprintln!(
            "\nSearches for individuals whose name contains the search term (case-insensitive)"
        );
        eprintln!("\nExample:");
        eprintln!("  cargo run --example find_person data/complete.ged Smith");
        eprintln!("  cargo run --example find_person data/complete.ged \"Mary\"");
        process::exit(1);
    }

    let filename = &args[1];
    let search_term = args[2].to_lowercase();

    // Parse the GEDCOM file with default configuration
    match parse_gedcom(filename, &GedcomConfig::new()) {
        Ok(gedcom) => {
            println!("Searching for '{}' in {}", search_term, filename);
            println!();

            // Search for individuals
            let matches: Vec<_> = gedcom
                .individuals
                .iter()
                .filter(|individual| {
                    individual.names.iter().any(|name| {
                        if let Some(value) = &name.name.value {
                            value.to_lowercase().contains(&search_term)
                        } else {
                            false
                        }
                    })
                })
                .collect();

            if matches.is_empty() {
                println!("No individuals found matching '{}'", search_term);
                return;
            }

            println!("Found {} individual(s):", matches.len());
            println!();

            for individual in &matches {
                // Display name
                if let Some(name) = individual.names.first() {
                    if let Some(value) = &name.name.value {
                        println!("Name: {}", value);
                    } else {
                        println!("Name: (no name value)");
                    }
                } else {
                    println!("Name: (unnamed)");
                }

                // Display reference ID
                if let Some(xref) = &individual.xref {
                    println!("  ID: {:?}", xref);
                }

                // Display gender
                println!("  Gender: {:?}", individual.gender);

                // Display birth information
                if let Some(birth) = individual.birth.first() {
                    print!("  Birth: ");
                    if let Some(date) = &birth.event.detail.date {
                        print!("{}", date);
                    }
                    if let Some(place) = &birth.event.detail.place {
                        if let Some(name) = &place.name {
                            print!(" at {}", name);
                        }
                    }
                    println!();
                }

                // Display christening information
                if let Some(christening) = individual.christening.first() {
                    print!("  Christening: ");
                    if let Some(date) = &christening.event.detail.date {
                        print!("{}", date);
                    }
                    if let Some(place) = &christening.event.detail.place {
                        if let Some(name) = &place.name {
                            print!(" at {}", name);
                        }
                    }
                    println!();
                }

                // Display death information
                if let Some(death) = individual.death.first() {
                    print!("  Death: ");
                    if let Some(event) = &death.event {
                        if let Some(date) = &event.date {
                            print!("{}", date);
                        }
                        if let Some(place) = &event.place {
                            if let Some(name) = &place.name {
                                print!(" at {}", name);
                            }
                        }
                    }
                    println!();
                }

                // Display residence information
                if !individual.residences.is_empty() {
                    println!("  Residences:");
                    for residence in &individual.residences {
                        println!("    - {:?}", residence.detail);
                    }
                }

                println!();
            }

            println!("Search complete. Found {} match(es).", matches.len());
        }
        Err(e) => {
            eprintln!("Error parsing GEDCOM file: {}", e);
            process::exit(1);
        }
    }
}
