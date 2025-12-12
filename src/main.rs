extern crate gedcom_rs;

use clap::Parser;
use gedcom_rs::parse::{parse_gedcom, GedcomConfig};
use gedcom_rs::types::Gedcom;
use std::process;
use tabled::{settings::Style, Table, Tabled};

/// GEDCOM 5.5.1 Parser and Analyzer
#[derive(Parser, Debug)]
#[command(name = "gedcom-rs")]
#[command(version)]
#[command(about = "Parse and analyze GEDCOM 5.5.1 genealogical data files", long_about = None)]
struct Args {
    /// Path to the GEDCOM file to parse
    #[arg(value_name = "FILE")]
    filename: String,

    /// Show detailed encoding warnings and diagnostics
    #[arg(short, long)]
    verbose: bool,

    /// Dump the entire GEDCOM structure (debug output)
    #[arg(short, long)]
    dump: bool,

    /// Show summary statistics (default behavior)
    #[arg(short, long, default_value_t = true)]
    summary: bool,

    /// XREF of the individual to use as the "home" person for genealogy analysis
    #[arg(long, value_name = "XREF")]
    home_xref: Option<String>,
}

/// Statistics row for the summary table
#[derive(Tabled)]
struct StatRow {
    #[tabled(rename = "Record Type")]
    record_type: String,
    #[tabled(rename = "Count")]
    count: usize,
}

fn main() {
    let args = Args::parse();

    // Configure parser
    let config = if args.verbose {
        GedcomConfig::new().verbose()
    } else {
        GedcomConfig::new()
    };

    // Parse the GEDCOM file
    let gedcom = match parse_gedcom(&args.filename, &config) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error parsing GEDCOM file: {}", e);
            process::exit(1);
        }
    };

    // Handle --dump flag
    if args.dump {
        println!("{:#?}", gedcom);
        return;
    }

    // Default behavior: show summary
    if args.summary {
        print_summary(&gedcom, args.home_xref.as_deref(), args.verbose);
    }
}

fn print_summary(gedcom: &Gedcom, home_xref: Option<&str>, verbose: bool) {
    // Get terminal width, default to 80 if unable to detect
    let term_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);

    // Use side-by-side layout if terminal is wide enough (>= 120 columns)
    if term_width >= 120 {
        print_summary_wide(gedcom, home_xref, verbose);
    } else {
        print_summary_narrow(gedcom, home_xref, verbose);
    }
}

/// Print summary in narrow (single column) format
fn print_summary_narrow(gedcom: &Gedcom, home_xref: Option<&str>, verbose: bool) {
    println!("═══════════════════════════════════════════════════════════");
    println!("                  GEDCOM FILE SUMMARY");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    print_file_info(gedcom);
    print_statistics_table(gedcom);
    print_warnings(gedcom, verbose);

    println!("───────────────────────────────────────────────────────────");
    println!("                  HOME INDIVIDUAL");
    println!("───────────────────────────────────────────────────────────");
    println!();

    print_home_individual(gedcom, home_xref);
    println!("═══════════════════════════════════════════════════════════");
}

/// Print summary in wide (two column) format
fn print_summary_wide(gedcom: &Gedcom, home_xref: Option<&str>, verbose: bool) {
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("                  GEDCOM FILE SUMMARY                     │                    HOME INDIVIDUAL");
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!();

    // Collect left column content
    let mut left_lines = Vec::new();
    collect_file_info(gedcom, &mut left_lines);
    collect_statistics(gedcom, &mut left_lines);
    collect_warnings(gedcom, verbose, &mut left_lines);

    // Collect right column content
    let mut right_lines = Vec::new();
    collect_home_individual(gedcom, home_xref, &mut right_lines);

    // Print side by side
    let max_lines = left_lines.len().max(right_lines.len());
    for i in 0..max_lines {
        let left = left_lines.get(i).map(|s| s.as_str()).unwrap_or("");
        let right = right_lines.get(i).map(|s| s.as_str()).unwrap_or("");

        // Left column is 58 chars wide, right column starts at position 60
        println!("{:<58} │ {}", left, right);
    }

    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
}

/// Print file information (narrow mode)
fn print_file_info(gedcom: &Gedcom) {
    if let Some(ref source) = gedcom.header.source {
        println!("Source System: {}", source.source);
        if let Some(ref name) = source.name {
            println!("Source Name:   {}", name);
        }
        if let Some(ref version) = source.version {
            println!("Version:       {}", version);
        }
        println!();
    }

    if let Some(ref gedc) = gedcom.header.gedcom_version {
        if let Some(ref version) = gedc.version {
            println!("GEDCOM Version: {}", version);
        }
        if let Some(ref form) = gedc.form {
            println!("GEDCOM Form:    {:?}", form);
        }
        println!();
    }
}

/// Collect file information into lines (wide mode)
fn collect_file_info(gedcom: &Gedcom, lines: &mut Vec<String>) {
    if let Some(ref source) = gedcom.header.source {
        lines.push(format!("Source System: {}", source.source));
        if let Some(ref name) = source.name {
            lines.push(format!("Source Name:   {}", name));
        }
        if let Some(ref version) = source.version {
            lines.push(format!("Version:       {}", version));
        }
        lines.push(String::new());
    }

    if let Some(ref gedc) = gedcom.header.gedcom_version {
        if let Some(ref version) = gedc.version {
            lines.push(format!("GEDCOM Version: {}", version));
        }
        if let Some(ref form) = gedc.form {
            lines.push(format!("GEDCOM Form:    {:?}", form));
        }
        lines.push(String::new());
    }
}

/// Print statistics table (narrow mode)
fn print_statistics_table(gedcom: &Gedcom) {
    let stats = build_stats_vec(gedcom);
    let mut table = Table::new(stats);
    table.with(Style::modern());
    println!("{}", table);
    println!();
}

/// Collect statistics into lines (wide mode)
fn collect_statistics(gedcom: &Gedcom, lines: &mut Vec<String>) {
    let stats = build_stats_vec(gedcom);
    let mut table = Table::new(stats);
    table.with(Style::modern());

    // Split table into lines
    for line in table.to_string().lines() {
        lines.push(line.to_string());
    }
    lines.push(String::new());
}

/// Build statistics vector
fn build_stats_vec(gedcom: &Gedcom) -> Vec<StatRow> {
    vec![
        StatRow {
            record_type: "Individuals".to_string(),
            count: gedcom.individuals.len(),
        },
        StatRow {
            record_type: "Families".to_string(),
            count: gedcom.families.len(),
        },
        StatRow {
            record_type: "Sources".to_string(),
            count: gedcom.sources.len(),
        },
        StatRow {
            record_type: "Repositories".to_string(),
            count: gedcom.repositories.len(),
        },
        StatRow {
            record_type: "Notes".to_string(),
            count: gedcom.notes.len(),
        },
        StatRow {
            record_type: "Multimedia".to_string(),
            count: gedcom.multimedia.len(),
        },
        StatRow {
            record_type: "Submitters".to_string(),
            count: gedcom.submitters.len(),
        },
    ]
}

/// Print warnings (narrow mode)
fn print_warnings(gedcom: &Gedcom, verbose: bool) {
    if gedcom.has_warnings() {
        println!("⚠ Warnings: {}", gedcom.warnings.len());
        if verbose {
            println!();
            println!("Warning Details:");
            for warning in &gedcom.warnings {
                println!("  • {}", warning);
            }
        }
        println!();
    }
}

/// Collect warnings into lines (wide mode)
fn collect_warnings(gedcom: &Gedcom, verbose: bool, lines: &mut Vec<String>) {
    if gedcom.has_warnings() {
        lines.push(format!("⚠ Warnings: {}", gedcom.warnings.len()));
        if verbose {
            lines.push(String::new());
            lines.push("Warning Details:".to_string());
            for warning in &gedcom.warnings {
                lines.push(format!("  • {}", warning));
            }
        }
        lines.push(String::new());
    }
}

/// Print home individual information (narrow mode)
fn print_home_individual(gedcom: &Gedcom, home_xref: Option<&str>) {
    let home_individual = if let Some(xref) = home_xref {
        gedcom.find_individual_by_xref(xref)
    } else {
        gedcom.individuals.first()
    };

    if let Some(individual) = home_individual {
        // Display name
        if let Some(name) = individual.names.first() {
            if let Some(ref name_value) = name.name.value {
                println!("Name: {}", name_value);
            }
            if let Some(ref given) = name.name.given {
                println!("Given Name: {}", given);
            }
            if let Some(ref surname) = name.name.surname {
                println!("Surname: {}", surname);
            }
        }

        if let Some(ref xref) = individual.xref {
            println!("XREF: {}", xref.as_str());
        }

        if let Some(birth) = individual.birth.first() {
            if let Some(ref date) = birth.event.detail.date {
                println!("Birth Date: {}", date);
            }
            if let Some(ref place) = birth.event.detail.place {
                if let Some(ref place_name) = place.name {
                    println!("Birth Place: {}", place_name);
                }
            }
        }

        if let Some(death) = individual.death.first() {
            if let Some(ref event) = death.event {
                if let Some(ref date) = event.date {
                    println!("Death Date: {}", date);
                }
                if let Some(ref place) = event.place {
                    if let Some(ref place_name) = place.name {
                        println!("Death Place: {}", place_name);
                    }
                }
            }
        }

        println!();

        let max_ancestor_gens = calculate_max_generations_ancestors(gedcom, individual);
        let max_descendant_gens = calculate_max_generations_descendants(gedcom, individual);

        println!("Genealogy Depth:");
        println!("  Ancestor Generations:   {}", max_ancestor_gens);
        println!("  Descendant Generations: {}", max_descendant_gens);
        println!(
            "  Total Generations:      {}",
            max_ancestor_gens + max_descendant_gens
        );
        println!();

        let parents = gedcom.get_parents(individual);
        let children = gedcom.get_children(individual);
        let siblings = gedcom.get_siblings(individual);
        let spouses = gedcom.get_spouses(individual);

        println!("Immediate Family:");
        println!("  Parents:   {}", if parents.is_empty() { 0 } else { 2 });
        println!("  Siblings:  {}", siblings.len());
        println!("  Spouses:   {}", spouses.len());
        println!("  Children:  {}", children.len());
        println!();

        let ancestors = gedcom.get_ancestors(individual, Some(10));
        let descendants = gedcom.get_descendants(individual, Some(10));

        println!("Extended Family (up to 10 generations):");
        println!("  Total Ancestors:   {}", ancestors.len());
        println!("  Total Descendants: {}", descendants.len());
        println!();
    } else {
        println!("No individuals found in GEDCOM file.");
        println!();
    }
}

/// Collect home individual information into lines (wide mode)
fn collect_home_individual(gedcom: &Gedcom, home_xref: Option<&str>, lines: &mut Vec<String>) {
    let home_individual = if let Some(xref) = home_xref {
        gedcom.find_individual_by_xref(xref)
    } else {
        gedcom.individuals.first()
    };

    if let Some(individual) = home_individual {
        if let Some(name) = individual.names.first() {
            if let Some(ref name_value) = name.name.value {
                lines.push(format!("Name: {}", name_value));
            }
            if let Some(ref given) = name.name.given {
                lines.push(format!("Given Name: {}", given));
            }
            if let Some(ref surname) = name.name.surname {
                lines.push(format!("Surname: {}", surname));
            }
        }

        if let Some(ref xref) = individual.xref {
            lines.push(format!("XREF: {}", xref.as_str()));
        }

        if let Some(birth) = individual.birth.first() {
            if let Some(ref date) = birth.event.detail.date {
                lines.push(format!("Birth Date: {}", date));
            }
            if let Some(ref place) = birth.event.detail.place {
                if let Some(ref place_name) = place.name {
                    lines.push(format!("Birth Place: {}", place_name));
                }
            }
        }

        if let Some(death) = individual.death.first() {
            if let Some(ref event) = death.event {
                if let Some(ref date) = event.date {
                    lines.push(format!("Death Date: {}", date));
                }
                if let Some(ref place) = event.place {
                    if let Some(ref place_name) = place.name {
                        lines.push(format!("Death Place: {}", place_name));
                    }
                }
            }
        }

        lines.push(String::new());

        let max_ancestor_gens = calculate_max_generations_ancestors(gedcom, individual);
        let max_descendant_gens = calculate_max_generations_descendants(gedcom, individual);

        lines.push("Genealogy Depth:".to_string());
        lines.push(format!("  Ancestors:   {} generations", max_ancestor_gens));
        lines.push(format!(
            "  Descendants: {} generations",
            max_descendant_gens
        ));
        lines.push(format!(
            "  Total:       {} generations",
            max_ancestor_gens + max_descendant_gens
        ));
        lines.push(String::new());

        let parents = gedcom.get_parents(individual);
        let children = gedcom.get_children(individual);
        let siblings = gedcom.get_siblings(individual);
        let spouses = gedcom.get_spouses(individual);

        lines.push("Immediate Family:".to_string());
        lines.push(format!(
            "  Parents:   {}",
            if parents.is_empty() { 0 } else { 2 }
        ));
        lines.push(format!("  Siblings:  {}", siblings.len()));
        lines.push(format!("  Spouses:   {}", spouses.len()));
        lines.push(format!("  Children:  {}", children.len()));
        lines.push(String::new());

        let ancestors = gedcom.get_ancestors(individual, Some(10));
        let descendants = gedcom.get_descendants(individual, Some(10));

        lines.push("Extended Family:".to_string());
        lines.push(format!("  Ancestors:   {}", ancestors.len()));
        lines.push(format!("  Descendants: {}", descendants.len()));
    } else {
        lines.push("No individuals found.".to_string());
    }
}

/// Calculate maximum number of ancestor generations from the given individual
fn calculate_max_generations_ancestors(
    gedcom: &Gedcom,
    individual: &gedcom_rs::types::Individual,
) -> usize {
    use std::collections::VecDeque;

    let mut max_depth = 0;
    let mut queue = VecDeque::new();
    queue.push_back((individual, 0));

    while let Some((current, depth)) = queue.pop_front() {
        if depth > max_depth {
            max_depth = depth;
        }

        for (father, mother) in gedcom.get_parents(current) {
            if let Some(dad) = father {
                queue.push_back((dad, depth + 1));
            }
            if let Some(mom) = mother {
                queue.push_back((mom, depth + 1));
            }
        }
    }

    max_depth
}

/// Calculate maximum number of descendant generations from the given individual
fn calculate_max_generations_descendants(
    gedcom: &Gedcom,
    individual: &gedcom_rs::types::Individual,
) -> usize {
    use std::collections::VecDeque;

    let mut max_depth = 0;
    let mut queue = VecDeque::new();
    queue.push_back((individual, 0));

    while let Some((current, depth)) = queue.pop_front() {
        if depth > max_depth {
            max_depth = depth;
        }

        for child in gedcom.get_children(current) {
            queue.push_back((child, depth + 1));
        }
    }

    max_depth
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use gedcom_rs::parse::{parse_gedcom, GedcomConfig};

    #[test]
    fn test_complete_gedcom() {
        let gedcom = parse_gedcom("./data/complete.ged", &GedcomConfig::new()).unwrap();

        // Test the header
        assert!(gedcom.header.copyright.is_some());
        let copyright = gedcom.header.copyright.unwrap();
        assert!(
            copyright == "© 1997 by H. Eichmann, parts © 1999-2000 by J. A. Nairn.".to_string()
        );

        // Test the note field
        assert!(gedcom.header.note.is_some());
        let note = gedcom.header.note.unwrap();
        assert!(note.starts_with("This file demonstrates all tags that are allowed in GEDCOM 5.5."));
        assert!(note.ends_with("GEDCOM 5.5 specs on the Internet at <http://homepages.rootsweb.com/~pmcbride/gedcom/55gctoc.htm>."));
    }

    #[test]
    fn test_file_not_found() {
        let result = parse_gedcom("./nonexistent.ged", &GedcomConfig::new());
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, gedcom_rs::error::GedcomError::FileNotFound(_)));
        }
    }

    #[test]
    fn test_source_parsing() {
        let gedcom = parse_gedcom("./data/complete.ged", &GedcomConfig::new()).unwrap();

        // complete.ged has 2 SOURCE records
        assert_eq!(gedcom.sources.len(), 2);

        // Test first source record
        let s1 = &gedcom.sources[0];
        assert!(s1.xref.is_some());
        assert_eq!(s1.xref.as_ref().unwrap().to_string(), "@S1@");
        assert!(s1.title.is_some());
        let title = s1.title.as_ref().unwrap();
        assert!(title.starts_with("Everything You Every Wanted to Know about GEDCOM Tags"));
        assert!(title.contains("Were Afraid to Ask!"));
        assert_eq!(s1.abbreviation.as_deref(), Some("All About GEDCOM Tags"));

        // Test second source record
        let s2 = &gedcom.sources[1];
        assert!(s2.xref.is_some());
        assert_eq!(s2.xref.as_ref().unwrap().to_string(), "@S2@");
        assert_eq!(
            s2.title.as_deref(),
            Some("All I Know About GEDCOM, I Learned on the Internet")
        );
        assert_eq!(s2.abbreviation.as_deref(), Some("What I Know About GEDCOM"));
        assert_eq!(s2.author.as_deref(), Some("Second Source Author"));
    }

    #[test]
    fn test_note_parsing() {
        let gedcom = parse_gedcom("./data/complete.ged", &GedcomConfig::new()).unwrap();

        // complete.ged has 27 NOTE records
        assert_eq!(gedcom.notes.len(), 27);

        // Test first note record
        let n1 = &gedcom.notes[0];
        assert!(n1.xref.is_some());
        assert_eq!(n1.xref.as_ref().unwrap().to_string(), "@N1@");
        assert_eq!(
            n1.note,
            "Test link to a graphics file about the main Submitter of this file."
        );

        // Test second note record (has source citation and multi-line content)
        let n2 = &gedcom.notes[1];
        assert_eq!(n2.xref.as_ref().unwrap().to_string(), "@N2@");
        assert!(n2.note.contains("Family History Library"));
        assert!(n2.note.contains("REPOSITORY Record"));
        assert!(n2.note.contains("Are they all imported?"));
        assert_eq!(n2.source_citations.len(), 1);
        assert_eq!(n2.source_citations[0].xref, Some("@S1@".to_string()));
    }

    #[test]
    fn test_multimedia_parsing() {
        let gedcom = parse_gedcom("./data/complete.ged", &GedcomConfig::new()).unwrap();

        // complete.ged has 16 MULTIMEDIA records
        assert_eq!(gedcom.multimedia.len(), 16);

        // Test first multimedia record
        let m1 = &gedcom.multimedia[0];
        assert!(m1.xref.is_some());
        assert_eq!(m1.xref.as_ref().unwrap().to_string(), "@M1@");
        assert_eq!(m1.files.len(), 1);
        assert_eq!(m1.files[0].file_reference, "photo.jpeg");
        assert_eq!(m1.files[0].format.as_deref(), Some("JPEG"));
        assert_eq!(m1.files[0].media_type.as_deref(), Some("photo"));
        assert_eq!(
            m1.files[0].title.as_deref(),
            Some("Picture of the book cover")
        );

        // Test second multimedia record
        let m2 = &gedcom.multimedia[1];
        assert_eq!(m2.xref.as_ref().unwrap().to_string(), "@M2@");
    }

    #[test]
    fn test_repository_parsing() {
        let gedcom = parse_gedcom("./data/complete.ged", &GedcomConfig::new()).unwrap();

        // complete.ged has 1 REPOSITORY record
        assert_eq!(gedcom.repositories.len(), 1);

        // Test the repository record
        let r1 = &gedcom.repositories[0];
        assert!(r1.xref.is_some());
        assert_eq!(r1.xref.as_ref().unwrap().to_string(), "@R1@");
        assert_eq!(r1.name, Some("Family History Library".to_string()));

        // Check address
        assert!(r1.address.is_some());
        let addr = r1.address.as_ref().unwrap();
        assert_eq!(addr.addr1.as_deref(), Some("35 North West Temple"));
        assert_eq!(addr.city.as_deref(), Some("Salt Lake City"));
        assert_eq!(addr.state.as_deref(), Some("UT"));

        // Check phones (part of address)
        assert_eq!(addr.phone.len(), 3);
        assert_eq!(addr.phone[0], "+1-801-240-2331");

        // Check note reference
        assert_eq!(r1.notes.len(), 1);
        assert_eq!(r1.notes[0].note, Some("@N2@".to_string()));
    }

    #[test]
    fn test_family_record_parsing() {
        let gedcom = parse_gedcom("./data/complete.ged", &GedcomConfig::new()).unwrap();

        // complete.ged has 6 FAMILY records
        assert_eq!(gedcom.families.len(), 6);

        // Test @F1@ - has marriage event with source citation
        let f1 = &gedcom.families[0];
        assert_eq!(f1.xref.to_string(), "@F1@");
        assert_eq!(f1.husband, Some("@I1@".into()));
        assert_eq!(f1.wife, Some("@I2@".into()));
        assert_eq!(f1.children.len(), 2);
        assert_eq!(f1.children[0].to_string(), "@I3@");
        assert_eq!(f1.children[1].to_string(), "@I4@");
        assert_eq!(f1.child_count, Some(42));
        // complete.ged has 7 family events: MARR, ENGA, DIV, DIVF, ANUL, CENS, EVEN
        assert_eq!(f1.events.len(), 7);

        // Test @F2@ - has CHANGE_DATE
        let f2 = &gedcom.families[1];
        assert_eq!(f2.xref.to_string(), "@F2@");
        assert!(f2.change_date.is_some());
        assert_eq!(
            f2.change_date.as_ref().unwrap().date,
            Some("13 JUN 2000".to_string())
        );
        assert_eq!(f2.automated_record_id, Some("2".to_string()));

        // Test @F3@ - has CHANGE_DATE
        let f3 = &gedcom.families[2];
        assert_eq!(f3.xref.to_string(), "@F3@");
        assert!(f3.change_date.is_some());
        assert_eq!(
            f3.change_date.as_ref().unwrap().date,
            Some("13 JUN 2000".to_string())
        );
        assert_eq!(f3.automated_record_id, Some("3".to_string()));
    }

    #[test]
    fn test_submitter_parsing() {
        let gedcom = parse_gedcom("./data/complete.ged", &GedcomConfig::new()).unwrap();

        // complete.ged has 1 SUBMITTER record
        assert_eq!(gedcom.submitters.len(), 1);

        let subm = &gedcom.submitters[0];
        assert_eq!(subm.xref.to_string(), "@U1@");
        assert_eq!(subm.name.as_deref(), Some("John A. Nairn"));

        // Check address
        assert!(subm.address.is_some());
        let addr = subm.address.as_ref().unwrap();
        assert_eq!(addr.addr1.as_deref(), Some("RSAC Software"));
        assert_eq!(addr.addr2.as_deref(), Some("7108 South Pine Cone Street"));
        assert_eq!(addr.addr3.as_deref(), Some("Ste 1"));
        assert_eq!(addr.city.as_deref(), Some("Salt Lake City"));
        assert_eq!(addr.state.as_deref(), Some("UT"));
        assert_eq!(addr.postal_code.as_deref(), Some("84121"));
        assert_eq!(addr.country.as_deref(), Some("USA"));

        // Check phone numbers (3 phones in file)
        assert_eq!(addr.phone.len(), 3);
        assert_eq!(addr.phone[0], "+1-801-942-7768");
        assert_eq!(addr.phone[1], "+1-801-555-1212");
        assert_eq!(addr.phone[2], "+1-801-942-1148");

        // Check languages (2 in file: English, German)
        assert_eq!(subm.languages.len(), 2);
        assert_eq!(subm.languages[0], "English");
        assert_eq!(subm.languages[1], "German");

        // Check multimedia links
        assert_eq!(subm.multimedia_links.len(), 1);

        // Check automated record id
        assert_eq!(subm.automated_record_id.as_deref(), Some("1"));

        // Check change date
        assert!(subm.change_date.is_some());
        assert_eq!(
            subm.change_date.as_ref().unwrap().date,
            Some("7 SEP 2000".to_string())
        );
    }

    #[test]
    fn test_max_generations_calculation() {
        let gedcom = parse_gedcom("./data/complete.ged", &GedcomConfig::new()).unwrap();

        if let Some(individual) = gedcom.individuals.first() {
            let _ancestor_gens = calculate_max_generations_ancestors(&gedcom, individual);
            let _descendant_gens = calculate_max_generations_descendants(&gedcom, individual);

            // Just verify the functions run without panicking
        }
    }
}
