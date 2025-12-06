//! Example: Exploring Family Relationships
//!
//! This example demonstrates how to parse family (FAM) records and navigate
//! relationships between individuals.
//!
//! Usage:
//!   cargo run --example family_tree path/to/file.ged

use gedcom_rs::parse::{parse_gedcom, GedcomConfig};
use std::collections::HashMap;
use std::env;
use std::process;

fn main() {
    // Get filename from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <gedcom-file>", args[0]);
        eprintln!("\nExample: {} data/TGC551LF.ged", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    // Parse the GEDCOM file
    let gedcom = match parse_gedcom(filename, &GedcomConfig::new()) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error parsing GEDCOM file: {}", e);
            process::exit(1);
        }
    };

    println!("GEDCOM Family Tree Analysis");
    println!("============================\n");

    // Build a lookup map for individuals by xref
    let mut individual_map: HashMap<String, String> = HashMap::new();
    for individual in &gedcom.individuals {
        if let Some(ref xref) = individual.xref {
            if let Some(name) = individual.names.first() {
                if let Some(value) = &name.name.value {
                    individual_map.insert(xref.to_string(), value.clone());
                }
            }
        }
    }

    // Summary statistics
    println!("Summary:");
    println!("  Individuals: {}", gedcom.individuals.len());
    println!("  Families: {}\n", gedcom.families.len());

    // Analyze families
    println!("Family Relationships:");
    println!("--------------------\n");

    for (i, family) in gedcom.families.iter().enumerate().take(10) {
        println!("{}. Family {} ", i + 1, family.xref);

        // Display husband
        if let Some(ref husband_xref) = family.husband {
            let husband_name = individual_map
                .get(&husband_xref.to_string())
                .map(|s| s.as_str())
                .unwrap_or("Unknown");
            println!("   Husband: {} ({})", husband_name, husband_xref);
        }

        // Display wife
        if let Some(ref wife_xref) = family.wife {
            let wife_name = individual_map
                .get(&wife_xref.to_string())
                .map(|s| s.as_str())
                .unwrap_or("Unknown");
            println!("   Wife: {} ({})", wife_name, wife_xref);
        }

        // Display children
        if !family.children.is_empty() {
            println!("   Children ({}):", family.children.len());
            for child_xref in &family.children {
                let child_name = individual_map
                    .get(&child_xref.to_string())
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");
                println!("     - {} ({})", child_name, child_xref);
            }
        }

        // Display child count if specified
        if let Some(count) = family.child_count {
            println!("   Recorded child count: {}", count);
        }

        // Display events
        if !family.events.is_empty() {
            println!("   Events: {}", family.events.len());
            for event in &family.events {
                if let Some(ref detail) = event.detail {
                    if let Some(ref date) = detail.date {
                        print!("     - Date: {}", date);
                    }
                    if let Some(ref place) = detail.place {
                        if let Some(ref name) = place.name {
                            print!(" at {}", name);
                        }
                    }
                    println!();
                }
            }
        }

        // Display notes
        if !family.notes.is_empty() {
            println!("   Notes: {}", family.notes.len());
        }

        println!();
    }

    if gedcom.families.len() > 10 {
        println!("... and {} more families\n", gedcom.families.len() - 10);
    }

    // Calculate statistics
    let families_with_marriage = gedcom
        .families
        .iter()
        .filter(|f| !f.events.is_empty())
        .count();

    let families_with_children = gedcom
        .families
        .iter()
        .filter(|f| !f.children.is_empty())
        .count();

    println!("\nStatistics:");
    println!("  Families with events: {}", families_with_marriage);
    println!("  Families with children: {}", families_with_children);

    let total_children: usize = gedcom.families.iter().map(|f| f.children.len()).sum();
    let avg_children = if gedcom.families.is_empty() {
        0.0
    } else {
        total_children as f64 / gedcom.families.len() as f64
    };
    println!("  Average children per family: {:.2}", avg_children);
}
