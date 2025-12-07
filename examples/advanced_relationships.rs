//! Explore ancestral and descendant relationships in a GEDCOM file
//!
//! This example demonstrates the advanced relationship functions:
//! - get_ancestors: Find all ancestors up to N generations
//! - get_descendants: Find all descendants up to N generations
//! - find_relationship_path: Find the connection between two individuals
//!
//! Usage:
//!   cargo run --example advanced_relationships path/to/file.ged [xref] [max_generations]
//!
//! If no xref is provided, it will use the first individual in the file.
//! If no max_generations is provided, it will find all ancestors/descendants.

use gedcom_rs::parse::{parse_gedcom, GedcomConfig};
use std::env;
use std::process;

fn get_name(individual: &gedcom_rs::types::Individual) -> String {
    individual
        .names
        .first()
        .and_then(|n| n.name.value.as_ref())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "(unnamed)".to_string())
}

fn get_xref_str(individual: &gedcom_rs::types::Individual) -> String {
    individual
        .xref
        .as_ref()
        .map(|x| x.to_string())
        .unwrap_or_else(|| "(no ID)".to_string())
}

fn get_birth_year(individual: &gedcom_rs::types::Individual) -> Option<String> {
    individual
        .birth
        .first()
        .and_then(|b| b.event.detail.date.as_ref())
        .map(|d| {
            // Try to extract year from date string
            d.split_whitespace().last().unwrap_or(d).to_string()
        })
}

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <gedcom-file> [xref] [max_generations]", args[0]);
        eprintln!("\nExplores ancestors and descendants of an individual");
        eprintln!("\nExamples:");
        eprintln!("  cargo run --example advanced_relationships data/TGC551.ged");
        eprintln!("  cargo run --example advanced_relationships data/TGC551.ged @I1@");
        eprintln!("  cargo run --example advanced_relationships data/TGC551.ged @I1@ 5");
        process::exit(1);
    }

    let filename = &args[1];
    let max_generations = if args.len() >= 4 {
        match args[3].parse::<usize>() {
            Ok(n) => Some(n),
            Err(_) => {
                eprintln!("Error: max_generations must be a positive number");
                process::exit(1);
            }
        }
    } else {
        None
    };

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

    // Determine which individual to examine
    let person = if args.len() >= 3 {
        let xref = &args[2];
        match gedcom.find_individual_by_xref(xref) {
            Some(p) => p,
            None => {
                eprintln!("Error: Individual with ID {} not found", xref);
                process::exit(1);
            }
        }
    } else {
        // Use first individual with family connections
        gedcom
            .individuals
            .iter()
            .find(|i| !i.famc.is_empty() || !i.fams.is_empty())
            .or_else(|| gedcom.individuals.first())
            .expect("No individuals found in GEDCOM file")
    };

    println!("=== Examining Relationships for ===");
    println!("Name: {}", get_name(person));
    println!("ID: {}", get_xref_str(person));
    if let Some(year) = get_birth_year(person) {
        println!("Birth: {}", year);
    }
    if let Some(max_gen) = max_generations {
        println!("Max generations: {}", max_gen);
    } else {
        println!("Max generations: unlimited");
    }
    println!();

    // Get ancestors
    println!("=== Ancestors ===");
    let ancestors = gedcom.get_ancestors(person, max_generations);
    if ancestors.is_empty() {
        println!("No ancestors found");
    } else {
        println!("Found {} ancestor(s):", ancestors.len());
        for (i, ancestor) in ancestors.iter().enumerate() {
            print!(
                "{}. {} ({})",
                i + 1,
                get_name(ancestor),
                get_xref_str(ancestor)
            );
            if let Some(year) = get_birth_year(ancestor) {
                print!(" - b. {}", year);
            }
            println!();
        }
    }
    println!();

    // Get descendants
    println!("=== Descendants ===");
    let descendants = gedcom.get_descendants(person, max_generations);
    if descendants.is_empty() {
        println!("No descendants found");
    } else {
        println!("Found {} descendant(s):", descendants.len());
        for (i, descendant) in descendants.iter().enumerate() {
            print!(
                "{}. {} ({})",
                i + 1,
                get_name(descendant),
                get_xref_str(descendant)
            );
            if let Some(year) = get_birth_year(descendant) {
                print!(" - b. {}", year);
            }
            println!();
        }
    }
    println!();

    // Demonstrate relationship path finding
    println!("=== Relationship Path ===");
    if ancestors.is_empty() && descendants.is_empty() {
        println!("No relationships to explore");
    } else {
        // Try to find path between this person and an ancestor or descendant
        let target = ancestors
            .first()
            .copied()
            .or_else(|| descendants.first().copied())
            .or_else(|| {
                // Try to find any other individual to demonstrate the path finding
                gedcom
                    .individuals
                    .iter()
                    .find(|i| i.xref.as_ref() != person.xref.as_ref())
            });

        if let Some(target_person) = target {
            println!(
                "Finding path from {} to {}",
                get_name(person),
                get_name(target_person)
            );

            match gedcom.find_relationship_path(person, target_person) {
                Some(path) => {
                    println!("Found path with {} step(s):", path.len());
                    for (i, step) in path.iter().enumerate() {
                        let arrow = if i < path.len() - 1 { " → " } else { "" };
                        print!("{}{}", get_name(step), arrow);
                    }
                    println!();
                }
                None => {
                    println!("No relationship path found between these individuals");
                }
            }
        }
    }
    println!();

    // Summary statistics
    println!("=== Summary ===");
    println!("Total ancestors found: {}", ancestors.len());
    println!("Total descendants found: {}", descendants.len());

    if max_generations.is_none() {
        println!("\nNote: These counts represent ALL known ancestors and descendants in the GEDCOM file.");
    } else {
        println!(
            "\nNote: These counts are limited to {} generation(s).",
            max_generations.unwrap()
        );
    }

    // Additional statistics
    if !ancestors.is_empty() {
        let oldest_ancestor = ancestors
            .iter()
            .filter_map(|a| {
                get_birth_year(a)
                    .and_then(|y| y.parse::<i32>().ok())
                    .map(|year| (a, year))
            })
            .min_by_key(|(_, year)| *year);

        if let Some((ancestor, year)) = oldest_ancestor {
            println!(
                "\nOldest known ancestor: {} (b. {})",
                get_name(ancestor),
                year
            );
        }
    }

    if !descendants.is_empty() {
        let youngest_descendant = descendants
            .iter()
            .filter_map(|d| {
                get_birth_year(d)
                    .and_then(|y| y.parse::<i32>().ok())
                    .map(|year| (d, year))
            })
            .max_by_key(|(_, year)| *year);

        if let Some((descendant, year)) = youngest_descendant {
            println!(
                "Youngest known descendant: {} (b. {})",
                get_name(descendant),
                year
            );
        }
    }
}
