use gedcom_rs::error::GedcomError;
use gedcom_rs::parse::{parse_gedcom, GedcomConfig};
use std::fs;

/// Helper function to create a temporary GEDCOM file for testing
fn create_temp_file(name: &str, content: &str) -> String {
    let filename = format!("test_{}.ged", name);
    fs::write(&filename, content).expect("Failed to write test file");
    filename
}

/// Helper function to clean up temporary files
fn cleanup_file(filename: &str) {
    let _ = fs::remove_file(filename);
}

#[test]
fn test_parse_complete_gedcom_file() {
    // Test with actual complete.ged file if it exists
    let result = parse_gedcom("./data/complete.ged", &GedcomConfig::new());
    assert!(result.is_ok(), "Failed to parse complete.ged");

    let gedcom = result.unwrap();
    // Basic sanity checks
    assert!(gedcom.individuals.len() > 0, "Should have individuals");
}

#[test]
fn test_parse_minimal_valid_gedcom() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   1 GEDC\n\
                   2 VERS 5.5.1\n\
                   0 TRLR\n";

    let filename = create_temp_file("minimal_valid", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_parse_gedcom_with_multiple_individuals() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME John /Doe/\n\
                   1 SEX M\n\
                   1 BIRT\n\
                   2 DATE 1 JAN 1950\n\
                   0 @I2@ INDI\n\
                   1 NAME Jane /Smith/\n\
                   1 SEX F\n\
                   0 TRLR\n";

    let filename = create_temp_file("multiple_individuals", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
    let gedcom = result.unwrap();
    assert_eq!(gedcom.individuals.len(), 2);
}

#[test]
fn test_parse_gedcom_with_family_relationships() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME John /Doe/\n\
                   1 FAMS @F1@\n\
                   0 @I2@ INDI\n\
                   1 NAME Jane /Smith/\n\
                   1 FAMS @F1@\n\
                   0 @F1@ FAM\n\
                   1 HUSB @I1@\n\
                   1 WIFE @I2@\n\
                   1 MARR\n\
                   2 DATE 1 JAN 1970\n\
                   0 TRLR\n";

    let filename = create_temp_file("family_relationships", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
    let gedcom = result.unwrap();
    assert_eq!(gedcom.individuals.len(), 2);
    assert_eq!(gedcom.families.len(), 1);
}

#[test]
fn test_parse_gedcom_with_sources() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @S1@ SOUR\n\
                   1 TITL Test Source Document\n\
                   1 AUTH John Historian\n\
                   1 PUBL Published by Test Press\n\
                   0 @I1@ INDI\n\
                   1 NAME Test /Person/\n\
                   1 BIRT\n\
                   2 DATE 1 JAN 2000\n\
                   2 SOUR @S1@\n\
                   0 TRLR\n";

    let filename = create_temp_file("with_sources", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
    let gedcom = result.unwrap();
    assert_eq!(gedcom.sources.len(), 1);
    assert_eq!(gedcom.individuals.len(), 1);
}

#[test]
fn test_parse_gedcom_with_notes() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @N1@ NOTE This is a test note\n\
                   1 CONT with multiple lines\n\
                   1 CONC concatenated together\n\
                   0 @I1@ INDI\n\
                   1 NAME Test /Person/\n\
                   1 NOTE @N1@\n\
                   0 TRLR\n";

    let filename = create_temp_file("with_notes", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
    let gedcom = result.unwrap();
    assert_eq!(gedcom.notes.len(), 1);
}

#[test]
fn test_parse_gedcom_with_repositories() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @R1@ REPO\n\
                   1 NAME National Archives\n\
                   1 ADDR\n\
                   2 ADR1 123 Main St\n\
                   2 CITY Washington\n\
                   2 STAE DC\n\
                   0 TRLR\n";

    let filename = create_temp_file("with_repositories", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
    let gedcom = result.unwrap();
    assert_eq!(gedcom.repositories.len(), 1);
}

#[test]
fn test_parse_gedcom_with_multimedia() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @M1@ OBJE\n\
                   1 FILE photo.jpg\n\
                   2 FORM jpeg\n\
                   2 TITL Family Photo\n\
                   0 TRLR\n";

    let filename = create_temp_file("with_multimedia", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
    let gedcom = result.unwrap();
    assert_eq!(gedcom.multimedia.len(), 1);
}

#[test]
fn test_parse_gedcom_different_encodings() {
    // Test UTF-8
    let content_utf8 = "0 HEAD\n1 CHAR UTF-8\n0 TRLR\n";
    let filename = create_temp_file("encoding_utf8", content_utf8);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);
    assert!(result.is_ok());

    // Test ASCII
    let content_ascii = "0 HEAD\n1 CHAR ASCII\n0 TRLR\n";
    let filename = create_temp_file("encoding_ascii", content_ascii);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);
    assert!(result.is_ok());

    // Test ANSEL (with approximation)
    let content_ansel = "0 HEAD\n1 CHAR ANSEL\n0 TRLR\n";
    let filename = create_temp_file("encoding_ansel", content_ansel);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);
    assert!(result.is_ok());
}

#[test]
fn test_parse_gedcom_with_conc_cont() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME Very Long Name That Needs\n\
                   2 CONC To Be Concatenated\n\
                   2 CONT And Also Continued\n\
                   2 CONT On Multiple Lines\n\
                   0 TRLR\n";

    let filename = create_temp_file("conc_cont", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_parse_gedcom_with_custom_tags() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   1 _CUSTOM Custom Value\n\
                   0 @I1@ INDI\n\
                   1 NAME Test /Person/\n\
                   1 _MYID 12345\n\
                   0 TRLR\n";

    let filename = create_temp_file("custom_tags", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_parse_gedcom_with_utf8_bom() {
    let content = "\u{FEFF}0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME Test /Person/\n\
                   0 TRLR\n";

    let filename = create_temp_file("utf8_bom", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
    let gedcom = result.unwrap();
    assert_eq!(gedcom.individuals.len(), 1);
}

#[test]
fn test_parse_gedcom_verbose_mode() {
    let content = "0 HEAD\n1 CHAR ANSEL\n0 TRLR\n";

    let filename = create_temp_file("verbose_mode", content);
    let config = GedcomConfig::new().verbose();
    let result = parse_gedcom(&filename, &config);
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_parse_file_not_found() {
    let result = parse_gedcom("nonexistent_file_xyz.ged", &GedcomConfig::new());
    assert!(result.is_err());
    match result {
        Err(GedcomError::FileNotFound(_)) => (),
        _ => panic!("Expected FileNotFound error"),
    }
}

#[test]
fn test_parse_empty_file() {
    let filename = create_temp_file("empty", "");
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
    let gedcom = result.unwrap();
    assert_eq!(gedcom.individuals.len(), 0);
}

#[test]
fn test_parse_gedcom_complex_names() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME John Paul /Doe-Smith/\n\
                   2 GIVN John Paul\n\
                   2 SURN Doe-Smith\n\
                   2 NICK Johnny\n\
                   1 NAME /Doe/\n\
                   2 TYPE birth\n\
                   0 TRLR\n";

    let filename = create_temp_file("complex_names", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
    let gedcom = result.unwrap();
    assert_eq!(gedcom.individuals.len(), 1);
}

#[test]
fn test_parse_gedcom_complex_dates() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @I1@ INDI\n\
                   1 NAME Test /Person/\n\
                   1 BIRT\n\
                   2 DATE ABT 1950\n\
                   1 DEAT\n\
                   2 DATE BET 2000 AND 2010\n\
                   0 TRLR\n";

    let filename = create_temp_file("complex_dates", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_parse_gedcom_with_crlf_line_endings() {
    let content = "0 HEAD\r\n1 CHAR UTF-8\r\n0 @I1@ INDI\r\n1 NAME Test /Person/\r\n0 TRLR\r\n";

    let filename = create_temp_file("crlf_endings", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
}

#[test]
fn test_parse_gedcom_with_submitters() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   1 SUBM @U1@\n\
                   0 @U1@ SUBM\n\
                   1 NAME John Smith\n\
                   1 ADDR\n\
                   2 ADR1 123 Main St\n\
                   0 TRLR\n";

    let filename = create_temp_file("with_submitters", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
    let gedcom = result.unwrap();
    assert_eq!(gedcom.submitters.len(), 1);
}

#[test]
fn test_parse_gedcom_large_file() {
    // Create a GEDCOM with many individuals
    let mut content = String::from("0 HEAD\n1 CHAR UTF-8\n");

    for i in 1..=100 {
        content.push_str(&format!("0 @I{}@ INDI\n", i));
        content.push_str(&format!("1 NAME Person{} /Test/\n", i));
    }

    content.push_str("0 TRLR\n");

    let filename = create_temp_file("large_file", &content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
    let gedcom = result.unwrap();
    assert_eq!(gedcom.individuals.len(), 100);
}

#[test]
fn test_parse_gedcom_with_all_record_types() {
    let content = "0 HEAD\n\
                   1 CHAR UTF-8\n\
                   0 @U1@ SUBM\n\
                   1 NAME Submitter\n\
                   0 @I1@ INDI\n\
                   1 NAME Individual\n\
                   0 @F1@ FAM\n\
                   1 HUSB @I1@\n\
                   0 @S1@ SOUR\n\
                   1 TITL Source\n\
                   0 @R1@ REPO\n\
                   1 NAME Repository\n\
                   0 @N1@ NOTE Note text\n\
                   0 @M1@ OBJE\n\
                   1 FILE file.jpg\n\
                   0 TRLR\n";

    let filename = create_temp_file("all_record_types", content);
    let result = parse_gedcom(&filename, &GedcomConfig::new());
    cleanup_file(&filename);

    assert!(result.is_ok());
    let gedcom = result.unwrap();
    assert_eq!(gedcom.submitters.len(), 1);
    assert_eq!(gedcom.individuals.len(), 1);
    assert_eq!(gedcom.families.len(), 1);
    assert_eq!(gedcom.sources.len(), 1);
    assert_eq!(gedcom.repositories.len(), 1);
    assert_eq!(gedcom.notes.len(), 1);
    assert_eq!(gedcom.multimedia.len(), 1);
}

#[test]
fn test_parse_gedcom_with_validation_warnings() {
    use std::fs;
    let temp_file = "test_validation_warnings.ged";
    let content = "0 HEAD\n1 CHAR UTF-8\n0 @I1@ INDI\n0 @F1@ FAM\n0 @U1@ SUBM\n0 TRLR\n";

    fs::write(temp_file, content).unwrap();

    let config = GedcomConfig::new();
    let result = parse_gedcom(temp_file, &config);

    // Cleanup
    let _ = fs::remove_file(temp_file);

    assert!(result.is_ok());
    let gedcom = result.unwrap();

    // Should have validation warnings
    assert!(!gedcom.warnings.is_empty(), "Expected validation warnings");

    // Check for individual without name warning
    let has_indi_warning = gedcom.warnings.iter().any(|w| {
        matches!(w, gedcom_rs::error::GedcomError::ValidationError {
            record_type, field, ..
        } if record_type == "INDI" && field == "NAME")
    });
    assert!(
        has_indi_warning,
        "Expected warning for individual without name"
    );

    // Check for family without members warning
    let has_fam_warning = gedcom.warnings.iter().any(|w| {
        matches!(w, gedcom_rs::error::GedcomError::ValidationError {
            record_type, field, ..
        } if record_type == "FAM" && field == "HUSB/WIFE/CHIL")
    });
    assert!(has_fam_warning, "Expected warning for empty family");

    // Check for submitter without name error
    let has_subm_error = gedcom.warnings.iter().any(|w| {
        matches!(w, gedcom_rs::error::GedcomError::MissingRequiredField {
            record_type, field, ..
        } if record_type == "SUBM" && field == "NAME")
    });
    assert!(
        has_subm_error,
        "Expected error for submitter without required name"
    );
}

#[test]
fn test_parse_gedcom_warnings_do_not_prevent_parsing() {
    use std::fs;
    let temp_file = "test_warnings_nonfatal.ged";
    // File with warnings but still parseable
    let content =
        "0 HEAD\n1 CHAR UTF-8\n0 @I1@ INDI\n1 SEX M\n0 @I2@ INDI\n1 NAME Test /Person/\n0 TRLR\n";

    fs::write(temp_file, content).unwrap();

    let config = GedcomConfig::new();
    let result = parse_gedcom(temp_file, &config);

    // Cleanup
    let _ = fs::remove_file(temp_file);

    assert!(result.is_ok(), "Parsing should succeed despite warnings");
    let gedcom = result.unwrap();

    // Should have 2 individuals
    assert_eq!(gedcom.individuals.len(), 2);

    // But should have warning for one without name
    let name_warnings = gedcom.warnings.iter().filter(|w| {
        matches!(w, gedcom_rs::error::GedcomError::ValidationError { field, .. } if field == "NAME")
    }).count();
    assert_eq!(name_warnings, 1, "Expected exactly one NAME warning");
}
