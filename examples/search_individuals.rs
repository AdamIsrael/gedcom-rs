//! Search for individuals in a GEDCOM file using various methods
//!
//! This example demonstrates the search functions available on the Gedcom struct:
//! - find_individual_by_xref: Find a specific individual by their ID
//! - find_individuals_by_name: Search for individuals by name (partial match)
//! - find_family_by_xref: Find a specific family by its ID
//!
//! Usage:
//!   cargo run --example search_individuals path/to/file.ged

use gedcom_rs::parse::{parse_gedcom, GedcomConfig};
use std::env;
use std::process;

fn print_individual_summary(individual: &gedcom_rs::types::Individual) {
    // Display name
    if let Some(name) = individual.names.first() {
        if let Some(value) = &name.name.value {
            println!("  Name: {}", value);
        }
    }

    // Display reference ID
    if let Some(xref) = &individual.xref {
        println!("  ID: {}", xref);
    }

    // Display gender
    println!("  Gender: {:?}", individual.gender);

    // Display birth date
    if let Some(birth) = individual.birth.first() {
        if let Some(date) = &birth.event.detail.date {
            println!("  Birth: {}", date);
        }
    }

    // Display death date
    if let Some(death) = individual.death.first() {
        if let Some(event) = &death.event {
            if let Some(date) = &event.date {
                println!("  Death: {}", date);
            }
        }
    }
}

fn main() {
    // Get filename from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <gedcom-file>", args[0]);
        eprintln!("\nDemonstrates various search functions on a GEDCOM file");
        eprintln!("\nExample:");
        eprintln!("  cargo run --example search_individuals data/TGC551.ged");
        process::exit(1);
    }

    let filename = &args[1];

    // Parse the GEDCOM file
    println!("Parsing GEDCOM file: {}", filename);
    let gedcom = match parse_gedcom(filename, &GedcomConfig::new()) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error parsing GEDCOM file: {}", e);
            process::exit(1);
        }
    };

    println!(
        "Successfully parsed {} individuals and {} families\n",
        gedcom.individuals.len(),
        gedcom.families.len()
    );

    // Example 1: Find individual by xref
    println!("=== Example 1: Find Individual by XREF ===");
    if let Some(first_person) = gedcom.individuals.first() {
        if let Some(xref) = &first_person.xref {
            println!("Searching for individual with ID: {}", xref);
            if let Some(person) = gedcom.find_individual_by_xref(xref.as_str()) {
                println!("Found:");
                print_individual_summary(person);
            }
        }
    }
    println!();

    // Example 2: Find individuals by name (partial match)
    println!("=== Example 2: Find Individuals by Name ===");
    // Try to search for a common name component
    let search_terms = vec!["Smith", "John", "Mary", "William"];

    for search_term in &search_terms {
        let results = gedcom.find_individuals_by_name(search_term);
        if !results.is_empty() {
            println!(
                "Found {} individual(s) with name containing '{}':",
                results.len(),
                search_term
            );
            for person in results.iter().take(3) {
                // Show first 3 matches
                print_individual_summary(person);
                println!();
            }
            if results.len() > 3 {
                println!("  ... and {} more\n", results.len() - 3);
            }
            break; // Found results, stop searching
        }
    }
    println!();

    // Example 3: Find family by xref
    println!("=== Example 3: Find Family by XREF ===");
    if let Some(first_family) = gedcom.families.first() {
        let family_xref = first_family.xref.as_str();
        println!("Searching for family with ID: {}", family_xref);
        if let Some(family) = gedcom.find_family_by_xref(family_xref) {
            println!("Found family:");
            if let Some(husband) = &family.husband {
                if let Some(h) = gedcom.find_individual_by_xref(husband.as_str()) {
                    if let Some(name) = h.names.first() {
                        if let Some(value) = &name.name.value {
                            println!("  Husband: {}", value);
                        }
                    }
                }
            }
            if let Some(wife) = &family.wife {
                if let Some(w) = gedcom.find_individual_by_xref(wife.as_str()) {
                    if let Some(name) = w.names.first() {
                        if let Some(value) = &name.name.value {
                            println!("  Wife: {}", value);
                        }
                    }
                }
            }
            println!("  Children: {}", family.children.len());
        }
    }
    println!();

    // Example 4: Case-insensitive name search
    println!("=== Example 4: Case-Insensitive Search ===");
    if let Some(first_person) = gedcom.individuals.first() {
        if let Some(name) = first_person.names.first() {
            if let Some(value) = &name.name.value {
                // Search with different cases
                let lower = value.to_lowercase();
                let upper = value.to_uppercase();

                println!("Original name: {}", value);
                println!("Searching with lowercase: {}", lower);
                let results_lower = gedcom.find_individuals_by_name(&lower);
                println!("  Found {} result(s)", results_lower.len());

                println!("Searching with uppercase: {}", upper);
                let results_upper = gedcom.find_individuals_by_name(&upper);
                println!("  Found {} result(s)", results_upper.len());
            }
        }
    }
    println!();

    println!("Search examples complete!");
}
