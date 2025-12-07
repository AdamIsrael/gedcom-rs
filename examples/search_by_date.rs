//! Search for individuals by event date
//!
//! This example demonstrates the find_individuals_by_event_date function
//! which allows searching for individuals based on specific life events and dates.
//!
//! Usage:
//!   cargo run --example search_by_date <gedcom-file> <event-type> <date-pattern>
//!
//! Event types: Birth, Death, Christening, Marriage
//! Date pattern: Any partial date string (e.g., "1965", "MAR 1999", "DEC")

use gedcom_rs::parse::{parse_gedcom, GedcomConfig};
use gedcom_rs::types::EventType;
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

fn parse_event_type(s: &str) -> Option<EventType> {
    match s.to_lowercase().as_str() {
        "birth" => Some(EventType::Birth),
        "death" => Some(EventType::Death),
        "christening" => Some(EventType::Christening),
        "baptism" => Some(EventType::Baptism),
        "burial" => Some(EventType::Burial),
        "adoption" => Some(EventType::Adoption),
        "census" => Some(EventType::Census),
        "emigration" => Some(EventType::Emigration),
        "immigration" => Some(EventType::Immigration),
        _ => None,
    }
}

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!(
            "Usage: {} <gedcom-file> <event-type> <date-pattern>",
            args[0]
        );
        eprintln!("\nSearches for individuals by event date");
        eprintln!("\nEvent types:");
        eprintln!("  Birth, Death, Christening, Baptism, Burial");
        eprintln!("  Adoption, Census, Emigration, Immigration");
        eprintln!("\nExamples:");
        eprintln!("  cargo run --example search_by_date data/complete.ged Birth 1965");
        eprintln!("  cargo run --example search_by_date data/complete.ged Death \"DEC 1997\"");
        eprintln!("  cargo run --example search_by_date data/complete.ged Christening MAR");
        process::exit(1);
    }

    let filename = &args[1];
    let event_type_str = &args[2];
    let date_pattern = &args[3];

    // Parse event type
    let event_type = match parse_event_type(event_type_str) {
        Some(et) => et,
        None => {
            eprintln!("Error: Unknown event type '{}'", event_type_str);
            eprintln!("Supported: Birth, Death, Christening, Baptism, Burial, Adoption, Census, Emigration, Immigration");
            process::exit(1);
        }
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
        "Successfully parsed {} individuals\n",
        gedcom.individuals.len()
    );

    // Search for individuals
    println!(
        "=== Searching for {:?} events matching '{}' ===\n",
        event_type, date_pattern
    );
    let results = gedcom.find_individuals_by_event_date(event_type, date_pattern);

    if results.is_empty() {
        println!(
            "No individuals found with {:?} events matching '{}'",
            event_type, date_pattern
        );
    } else {
        println!("Found {} individual(s):\n", results.len());
        for (i, person) in results.iter().enumerate() {
            println!("{}. {} ({})", i + 1, get_name(person), get_xref_str(person));

            // Show the matching event date
            match event_type {
                EventType::Birth => {
                    for birth in &person.birth {
                        if let Some(date) = &birth.event.detail.date {
                            if date.to_lowercase().contains(&date_pattern.to_lowercase()) {
                                println!("   Birth: {}", date);
                                if let Some(place) = &birth.event.detail.place {
                                    if let Some(place_name) = &place.name {
                                        println!("   Place: {}", place_name);
                                    }
                                }
                            }
                        }
                    }
                }
                EventType::Death => {
                    for death in &person.death {
                        if let Some(event) = &death.event {
                            if let Some(date) = &event.date {
                                if date.to_lowercase().contains(&date_pattern.to_lowercase()) {
                                    println!("   Death: {}", date);
                                    if let Some(place) = &event.place {
                                        if let Some(place_name) = &place.name {
                                            println!("   Place: {}", place_name);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                EventType::Christening => {
                    for christening in &person.christening {
                        if let Some(date) = &christening.event.detail.date {
                            if date.to_lowercase().contains(&date_pattern.to_lowercase()) {
                                println!("   Christening: {}", date);
                                if let Some(place) = &christening.event.detail.place {
                                    if let Some(place_name) = &place.name {
                                        println!("   Place: {}", place_name);
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {} // Handle other event types as needed
            }
            println!();
        }
    }
}
