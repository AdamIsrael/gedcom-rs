//! Determine genealogical relationships between individuals
//!
//! This example demonstrates the find_relationship function which:
//! - Identifies the relationship type (parent, sibling, cousin, etc.)
//! - Finds the Most Recent Common Ancestor (MRCA)
//! - Calculates generational distances
//!
//! Usage:
//!   cargo run --example find_relationship path/to/file.ged [xref1] [xref2]
//!
//! If no xrefs are provided, it will demonstrate relationships between
//! various individuals in the file.

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

fn print_relationship_details(
    gedcom: &gedcom_rs::types::Gedcom,
    person1: &gedcom_rs::types::Individual,
    person2: &gedcom_rs::types::Individual,
) {
    println!("=== Analyzing Relationship ===");
    println!(
        "Person 1: {} ({})",
        get_name(person1),
        get_xref_str(person1)
    );
    println!(
        "Person 2: {} ({})",
        get_name(person2),
        get_xref_str(person2)
    );
    println!();

    let relationship = gedcom.find_relationship(person1, person2);

    println!("Relationship: {}", relationship.description);

    if let Some(gen1) = relationship.generations_to_mrca_1 {
        println!("Generations from {} to MRCA: {}", get_name(person1), gen1);
    }
    if let Some(gen2) = relationship.generations_to_mrca_2 {
        println!("Generations from {} to MRCA: {}", get_name(person2), gen2);
    }

    if !relationship.mrca.is_empty() {
        println!("\nMost Recent Common Ancestor(s):");
        for (i, ancestor) in relationship.mrca.iter().enumerate() {
            println!(
                "  {}. {} ({})",
                i + 1,
                get_name(ancestor),
                get_xref_str(ancestor)
            );
        }
    } else if relationship.description != "Not related" && relationship.description != "Self" {
        println!("\nNo common ancestor (direct relationship)");
    }
    println!();
}

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <gedcom-file> [xref1] [xref2]", args[0]);
        eprintln!("\nDetermines the genealogical relationship between two individuals");
        eprintln!("\nExamples:");
        eprintln!("  cargo run --example find_relationship data/complete.ged");
        eprintln!("  cargo run --example find_relationship data/complete.ged @I1@ @I2@");
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

    if gedcom.individuals.is_empty() {
        eprintln!("No individuals found in GEDCOM file");
        process::exit(1);
    }

    // If specific xrefs provided, use those
    if args.len() >= 4 {
        let xref1 = &args[2];
        let xref2 = &args[3];

        let person1 = match gedcom.find_individual_by_xref(xref1) {
            Some(p) => p,
            None => {
                eprintln!("Error: Individual with ID {} not found", xref1);
                process::exit(1);
            }
        };

        let person2 = match gedcom.find_individual_by_xref(xref2) {
            Some(p) => p,
            None => {
                eprintln!("Error: Individual with ID {} not found", xref2);
                process::exit(1);
            }
        };

        print_relationship_details(&gedcom, person1, person2);
    } else {
        // Demonstrate various relationships in the file
        println!("=== Demonstrating Various Relationships ===\n");

        // Find some interesting relationships to demonstrate
        let mut demonstrated = 0;

        // Try to find parent-child relationships
        for person in gedcom.individuals.iter().take(10) {
            let children = gedcom.get_children(person);
            if !children.is_empty() {
                println!(
                    "--- Example {}: Parent-Child Relationship ---",
                    demonstrated + 1
                );
                print_relationship_details(&gedcom, person, children[0]);
                demonstrated += 1;
                if demonstrated >= 5 {
                    break;
                }
            }
        }

        // Try to find sibling relationships
        if demonstrated < 5 {
            for person in gedcom.individuals.iter().take(10) {
                let siblings = gedcom.get_siblings(person);
                if !siblings.is_empty() {
                    println!("--- Example {}: Sibling Relationship ---", demonstrated + 1);
                    print_relationship_details(&gedcom, person, siblings[0]);
                    demonstrated += 1;
                    if demonstrated >= 5 {
                        break;
                    }
                }
            }
        }

        // Try to find cousin relationships
        if demonstrated < 5 {
            for person in gedcom.individuals.iter().take(20) {
                // Get all descendants of grandparents to find cousins
                for (gf, gm) in gedcom.get_parents(person) {
                    for grandparent in [gf, gm].iter().filter_map(|x| *x) {
                        let descendants = gedcom.get_descendants(grandparent, Some(3));
                        for descendant in descendants.iter() {
                            let relationship = gedcom.find_relationship(person, descendant);
                            if relationship.description.contains("Cousin")
                                || relationship.description.contains("Aunt")
                                || relationship.description.contains("Uncle")
                            {
                                println!(
                                    "--- Example {}: {} ---",
                                    demonstrated + 1,
                                    relationship.description
                                );
                                print_relationship_details(&gedcom, person, descendant);
                                demonstrated += 1;
                                if demonstrated >= 5 {
                                    break;
                                }
                            }
                        }
                        if demonstrated >= 5 {
                            break;
                        }
                    }
                    if demonstrated >= 5 {
                        break;
                    }
                }
                if demonstrated >= 5 {
                    break;
                }
            }
        }

        // Try to find spouse relationships
        if demonstrated < 5 {
            for person in gedcom.individuals.iter().take(10) {
                let spouses = gedcom.get_spouses(person);
                if !spouses.is_empty() {
                    println!("--- Example {}: Spouse Relationship ---", demonstrated + 1);
                    print_relationship_details(&gedcom, person, spouses[0]);
                    demonstrated += 1;
                    if demonstrated >= 5 {
                        break;
                    }
                }
            }
        }

        if demonstrated == 0 {
            println!("Could not find interesting relationships to demonstrate.");
            println!("Try specifying two individual IDs to compare.");
        } else {
            println!("=== Summary ===");
            println!("Demonstrated {} different relationship types", demonstrated);
            println!("\nTip: Run with two specific XREFs to analyze any relationship:");
            println!(
                "  cargo run --example find_relationship {} @I1@ @I2@",
                filename
            );
        }
    }
}
