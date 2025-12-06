/// Edge case tests for the GEDCOM parser
///
/// These tests verify that the parser handles malformed or unusual input correctly
use gedcom_rs::parse::{parse_gedcom, GedcomConfig};
use std::fs;

fn create_temp_file(name: &str, content: &str) -> String {
    let filename = format!("test_edge_{}.ged", name);
    fs::write(&filename, content).expect("Failed to write test file");
    filename
}

fn cleanup_file(filename: &str) {
    let _ = fs::remove_file(filename);
}

#[test]
fn test_missing_header() {
    let content = "0 @I1@ INDI\n1 NAME Test /Person/\n0 TRLR\n";

    let filename = create_temp_file("missing_header", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    // Should still parse, just without header data
    assert!(result.is_ok());
}

#[test]
fn test_missing_trailer() {
    let content = "0 HEAD\n1 CHAR UTF-8\n0 @I1@ INDI\n1 NAME Test /Person/\n";

    let filename = create_temp_file("missing_trailer", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    // Should still parse successfully
    assert!(result.is_ok());
}

#[test]
fn test_empty_lines() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   \n\
                   0 @I1@ INDI\n\
                   1 NAME Test /Person/\n\
                   \n\
                   0 TRLR\n";

    let filename = create_temp_file("empty_lines", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_whitespace_only_lines() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                      \n\
                   0 @I1@ INDI\n\
                   1 NAME Test\n\
                   0 TRLR\n";

    let filename = create_temp_file("whitespace_lines", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_very_long_line() {
    let long_name = "A".repeat(1000);
    let content = format!(
        "0 HEAD\n1 CHAR UTF-8\n0 @I1@ INDI\n1 NAME {}\n0 TRLR\n",
        long_name
    );

    let filename = create_temp_file("long_line", &content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_deep_nesting() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME Test\n\
                   1 BIRT\n\
                   2 DATE 2000\n\
                   3 TIME 12:00\n\
                   2 PLAC Location\n\
                   3 MAP\n\
                   4 LATI N0\n\
                   4 LONG E0\n\
                   0 TRLR\n";

    let filename = create_temp_file("deep_nesting", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_unknown_record_type() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @X1@ UNKNOWN\n\
                   1 DATA Some data\n\
                   0 @I1@ INDI\n\
                   1 NAME Test\n\
                   0 TRLR\n";

    let filename = create_temp_file("unknown_record", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    // Should skip unknown records and continue
    assert!(result.is_ok());
}

#[test]
fn test_unknown_tags_in_record() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME Test\n\
                   1 UNKNOWN_TAG Some value\n\
                   1 SEX M\n\
                   0 TRLR\n";

    let filename = create_temp_file("unknown_tags", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_duplicate_xrefs() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME First Person\n\
                   0 @I1@ INDI\n\
                   1 NAME Second Person\n\
                   0 TRLR\n";

    let filename = create_temp_file("duplicate_xrefs", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    // Parser should accept this (validation is separate concern)
    assert!(result.is_ok());
    let gedcom = result.unwrap();
    assert_eq!(gedcom.individuals.len(), 2);
}

#[test]
fn test_invalid_xref_format() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @INVALID INDI\n\
                   1 NAME Test\n\
                   0 TRLR\n";

    let filename = create_temp_file("invalid_xref", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_missing_required_values() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME\n\
                   1 SEX\n\
                   0 TRLR\n";

    let filename = create_temp_file("missing_values", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_special_characters_in_values() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME Test <>&\"'/Person/\n\
                   1 NOTE Line with @special@ characters!\n\
                   0 TRLR\n";

    let filename = create_temp_file("special_chars", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_unicode_characters() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME José María /García/\n\
                   1 NOTE 中文测试 Тест Δοκιμή\n\
                   0 TRLR\n";

    let filename = create_temp_file("unicode", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_tabs_as_delimiters() {
    let content = "0\tHEAD\n1\tCHAR\tUTF-8\n0\t@I1@\tINDI\n1\tNAME\tTest\n0\tTRLR\n";

    let filename = create_temp_file("tabs", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    // Should handle tabs as whitespace
    assert!(result.is_ok());
}

#[test]
fn test_mixed_line_endings() {
    let content = "0 HEAD\r\n1 CHAR UTF-8\n0 @I1@ INDI\r\n1 NAME Test\n0 TRLR\r\n";

    let filename = create_temp_file("mixed_endings", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_level_zero_in_middle_of_record() {
    // Improperly nested records
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME Test\n\
                   0 @I2@ INDI\n\
                   1 NAME Another\n\
                   0 TRLR\n";

    let filename = create_temp_file("level_zero_middle", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_very_high_level_numbers() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME Test\n\
                   15 DATA Deep nested\n\
                   0 TRLR\n";

    let filename = create_temp_file("high_levels", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_conc_without_initial_value() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME\n\
                   2 CONC Test\n\
                   0 TRLR\n";

    let filename = create_temp_file("conc_no_initial", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_multiple_spaces_in_values() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME Test    With    Spaces\n\
                   0 TRLR\n";

    let filename = create_temp_file("multiple_spaces", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_trailing_whitespace() {
    let content = "0 HEAD   \n\
                   1 CHAR UTF-8   \n\
                   0 @I1@ INDI   \n\
                   1 NAME Test   \n\
                   0 TRLR   \n";

    let filename = create_temp_file("trailing_whitespace", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_numeric_only_values() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME 12345\n\
                   1 NOTE 67890\n\
                   0 TRLR\n";

    let filename = create_temp_file("numeric_values", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_extremely_nested_conc_cont() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NOTE Line 1\n\
                   2 CONT Line 2\n\
                   2 CONT Line 3\n\
                   2 CONT Line 4\n\
                   2 CONT Line 5\n\
                   2 CONC Part A\n\
                   2 CONC Part B\n\
                   2 CONC Part C\n\
                   0 TRLR\n";

    let filename = create_temp_file("nested_conc_cont", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_file_with_only_header() {
    let content = "0 HEAD\n1 CHAR UTF-8\n1 GEDC\n2 VERS 5.5.1\n";

    let filename = create_temp_file("only_header", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
    let gedcom = result.unwrap();
    assert_eq!(gedcom.individuals.len(), 0);
}

#[test]
fn test_records_without_xrefs() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 INDI\n\
                   1 NAME Test\n\
                   0 FAM\n\
                   1 HUSB @I1@\n\
                   0 TRLR\n";

    let filename = create_temp_file("no_xrefs", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    // Should handle missing xrefs
    assert!(result.is_ok());
}

#[test]
fn test_circular_references() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME Person 1\n\
                   1 FAMC @F1@\n\
                   0 @F1@ FAM\n\
                   1 CHIL @I1@\n\
                   0 TRLR\n";

    let filename = create_temp_file("circular_refs", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    // Parser should handle this (validation is separate)
    assert!(result.is_ok());
}
