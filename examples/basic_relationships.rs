//! Explore basic family relationships in a GEDCOM file
//!
//! This example demonstrates the basic relationship functions:
//! - get_parents: Find the parents of an individual
//! - get_children: Find all children of an individual
//! - get_spouses: Find all spouses of an individual
//! - get_siblings: Find all siblings of an individual
//! - get_full_siblings: Find full siblings (same mother and father)
//! - get_half_siblings: Find half-siblings (one shared parent)
//!
//! Usage:
//!   cargo run --example basic_relationships path/to/file.ged [xref]
//!
//! If no xref is provided, it will use the first individual in the file.

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

fn main() {
    // Get filename from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <gedcom-file> [xref]", args[0]);
        eprintln!("\nExplores family relationships for an individual");
        eprintln!("\nExample:");
        eprintln!("  cargo run --example basic_relationships data/TGC551.ged");
        eprintln!("  cargo run --example basic_relationships data/TGC551.ged @I1@");
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
    println!("Gender: {:?}", person.gender);
    if let Some(birth) = person.birth.first() {
        if let Some(date) = &birth.event.detail.date {
            println!("Birth: {}", date);
        }
    }
    println!();

    // Get parents
    println!("=== Parents ===");
    let parents = gedcom.get_parents(person);
    if parents.is_empty() {
        println!("No parents found");
    } else {
        for (i, (father, mother)) in parents.iter().enumerate() {
            println!("Family {}:", i + 1);
            if let Some(dad) = father {
                println!("  Father: {} ({})", get_name(dad), get_xref_str(dad));
                if let Some(birth) = dad.birth.first() {
                    if let Some(date) = &birth.event.detail.date {
                        println!("    Birth: {}", date);
                    }
                }
            } else {
                println!("  Father: (unknown)");
            }

            if let Some(mom) = mother {
                println!("  Mother: {} ({})", get_name(mom), get_xref_str(mom));
                if let Some(birth) = mom.birth.first() {
                    if let Some(date) = &birth.event.detail.date {
                        println!("    Birth: {}", date);
                    }
                }
            } else {
                println!("  Mother: (unknown)");
            }
        }
    }
    println!();

    // Get spouses
    println!("=== Spouses ===");
    let spouses = gedcom.get_spouses(person);
    if spouses.is_empty() {
        println!("No spouses found");
    } else {
        for (i, spouse) in spouses.iter().enumerate() {
            println!("{}. {} ({})", i + 1, get_name(spouse), get_xref_str(spouse));
            if let Some(birth) = spouse.birth.first() {
                if let Some(date) = &birth.event.detail.date {
                    println!("   Birth: {}", date);
                }
            }
        }
    }
    println!();

    // Get children
    println!("=== Children ===");
    let children = gedcom.get_children(person);
    if children.is_empty() {
        println!("No children found");
    } else {
        println!("Found {} child(ren):", children.len());
        for (i, child) in children.iter().enumerate() {
            println!("{}. {} ({})", i + 1, get_name(child), get_xref_str(child));
            if let Some(birth) = child.birth.first() {
                if let Some(date) = &birth.event.detail.date {
                    println!("   Birth: {}", date);
                }
            }
        }
    }
    println!();

    // Get siblings
    println!("=== Siblings ===");
    let siblings = gedcom.get_siblings(person);
    let full_siblings = gedcom.get_full_siblings(person);
    let half_siblings = gedcom.get_half_siblings(person);

    if siblings.is_empty() {
        println!("No siblings found");
    } else {
        println!("Found {} total sibling(s):", siblings.len());
        println!("  - {} full sibling(s)", full_siblings.len());
        println!("  - {} half-sibling(s)", half_siblings.len());
        println!();

        if !full_siblings.is_empty() {
            println!("Full Siblings:");
            for (i, sibling) in full_siblings.iter().enumerate() {
                println!(
                    "  {}. {} ({})",
                    i + 1,
                    get_name(sibling),
                    get_xref_str(sibling)
                );
                if let Some(birth) = sibling.birth.first() {
                    if let Some(date) = &birth.event.detail.date {
                        println!("     Birth: {}", date);
                    }
                }
            }
            println!();
        }

        if !half_siblings.is_empty() {
            println!("Half-Siblings:");
            for (i, sibling) in half_siblings.iter().enumerate() {
                println!(
                    "  {}. {} ({})",
                    i + 1,
                    get_name(sibling),
                    get_xref_str(sibling)
                );
                if let Some(birth) = sibling.birth.first() {
                    if let Some(date) = &birth.event.detail.date {
                        println!("     Birth: {}", date);
                    }
                }
            }
        }
    }
    println!();

    // Summary
    println!("=== Summary ===");
    println!(
        "Total parents: {}",
        parents
            .iter()
            .filter_map(|(f, m)| if f.is_some() || m.is_some() {
                Some(1)
            } else {
                None
            })
            .count()
    );
    println!("Total spouses: {}", spouses.len());
    println!("Total children: {}", children.len());
    println!(
        "Total siblings: {} ({} full, {} half)",
        siblings.len(),
        full_siblings.len(),
        half_siblings.len()
    );
}
