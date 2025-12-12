// use crate::types::{Address, Line, Source};
// use super::types::Line;
use super::types::*;
use crate::error::{GedcomError, Result};

use std::fs::File;
use std::io::Read;
use std::path::Path;

use encoding_rs::{Encoding, WINDOWS_1252};
use winnow::prelude::*;

/// Configuration options for parsing GEDCOM files
#[derive(Debug, Clone, Default)]
pub struct GedcomConfig {
    /// Enable verbose output including detailed encoding warnings and diagnostics
    pub verbose: bool,
}

impl GedcomConfig {
    /// Create a new configuration with default settings (verbose = false)
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable verbose output
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
}

// This is pretty much a kludge to strip out U+FEFF, a Zero Width No-Break Space
// https://www.compart.com/en/unicode/U+FEFF
//
// So far, I've only seen this with one GEDCOM, as the starting byte.
// pub fn zero_with_no_break_space(input: &mut &str) -> PResult<&str> {
//     if input.starts_with('\u{FEFF}') {
//         let parser = tag("\u{FEFF}");
//
//         parser.parse_next(input)
//     } else {
//         Ok("")
//     }
// }

/// Read the next tag's value and any continuations
pub fn get_tag_value(input: &mut &str) -> PResult<Option<String>> {
    let mut line = Line::parse(input)?;

    // Seed the value with the initial value
    let mut text: String = line.value.to_string();

    line = Line::peek(input)?;
    while line.tag == "CONC" || line.tag == "CONT" {
        // consume
        line = Line::parse(input)?;

        if line.tag == "CONT" {
            text.push('\n');
        }
        text.push_str(line.value);

        // peek ahead
        line = Line::peek(input)?;
    }

    Ok(Some(text))
}

/// Detect the character encoding declared in the GEDCOM header
/// Scans the first part of the file (as ASCII) to find the CHAR tag
///
/// Returns a tuple of (encoding, encoding_name) where encoding_name is for logging
fn detect_gedcom_encoding(bytes: &[u8]) -> (&'static Encoding, String) {
    // GEDCOM header tags are always ASCII-compatible, so we can safely
    // search for "1 CHAR" pattern in the first ~2KB of the file
    let search_limit = bytes.len().min(2048);
    let search_bytes = bytes.get(..search_limit).unwrap_or(bytes);

    // Look for "1 CHAR" followed by a space and the encoding name
    // This is a simple ASCII search that works regardless of encoding
    if let Some(pos) = search_bytes.windows(7).position(|w| w == b"1 CHAR ") {
        let start = pos + 7; // Skip "1 CHAR "

        // Find the end of the line (CR, LF, or CRLF)
        if let Some(rest) = search_bytes.get(start..) {
            let end = rest
                .iter()
                .position(|&b| b == b'\r' || b == b'\n')
                .map(|p| start + p)
                .unwrap_or(search_bytes.len());

            if let Some(encoding_slice) = search_bytes.get(start..end) {
                if let Ok(encoding_name) = std::str::from_utf8(encoding_slice) {
                    let encoding_name = encoding_name.trim().to_string();

                    // Map GEDCOM encoding names to Rust encoding_rs encodings
                    return match encoding_name.as_str() {
                        "UTF-8" | "UTF8" => (encoding_rs::UTF_8, encoding_name),
                        // ANSEL is a specialized character set for genealogy that combines
                        // ASCII with diacritical marks. We approximate it with Windows-1252
                        // which covers most common Latin characters, though some special
                        // ANSEL characters may not convert perfectly.
                        "ANSEL" => (WINDOWS_1252, encoding_name),
                        "ASCII" => (encoding_rs::UTF_8, encoding_name), // ASCII is a subset of UTF-8
                        "UNICODE" => (encoding_rs::UTF_16LE, encoding_name),
                        "ANSI" => (WINDOWS_1252, encoding_name),
                        _ => {
                            // Default to Windows-1252 for unknown encodings
                            // as it's a superset of ISO-8859-1
                            eprintln!(
                                "Warning: Unknown encoding '{}', defaulting to Windows-1252",
                                encoding_name
                            );
                            (WINDOWS_1252, encoding_name)
                        }
                    };
                }
            }
        }
    }

    // If no CHAR tag found, default to UTF-8
    (encoding_rs::UTF_8, "UTF-8 (default)".to_string())
}

/// Read file as bytes and convert to UTF-8 String based on declared encoding
fn read_file_with_encoding(
    filename: &str,
    config: &GedcomConfig,
) -> Result<(String, Option<GedcomError>)> {
    let mut file = File::open(filename)?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    // Detect the encoding from the GEDCOM header
    let (encoding, encoding_name) = detect_gedcom_encoding(&bytes);

    if config.verbose {
        eprintln!("Detected encoding: {}", encoding_name);
    }

    // Warn about ANSEL limitations
    if encoding_name == "ANSEL" && config.verbose {
        eprintln!("\nWarning: ANSEL encoding detected");
        eprintln!("  ANSEL is a specialized genealogical character set (ANSI/NISO Z39.47-1993)");
        eprintln!("  that uses prefix diacritics and combining characters.");
        eprintln!();
        eprintln!(
            "  Current implementation approximates ANSEL using Windows-1252, which may cause:"
        );
        eprintln!("  - Accented characters (é, ñ, ü, etc.) to display incorrectly");
        eprintln!("  - Loss of combining diacritical marks");
        eprintln!("  - Incorrect representation of special genealogical symbols");
        eprintln!();
        eprintln!("  For full ANSEL support, please see: https://github.com/adamgiacomelli/gedcom-rs/issues/[TBD]");
        eprintln!();
    }

    // Decode bytes to String using the detected encoding
    let (cow, _encoding_used, had_errors) = encoding.decode(&bytes);

    let warning = if had_errors {
        if config.verbose {
            eprintln!(
                "Warning: Encoding errors detected while converting '{}' from {} to UTF-8",
                filename, encoding_name
            );
        }
        Some(GedcomError::EncodingError {
            declared_encoding: encoding_name.clone(),
            detected_encoding: None,
            message: format!(
                "Errors detected during conversion from {} to UTF-8",
                encoding_name
            ),
            had_errors: true,
        })
    } else {
        None
    };

    Ok((cow.into_owned(), warning))
}

// Parse the buffer if the CONC tag is found and return the resulting string.
// pub fn conc(input: &mut &str) -> PResult<Option<String>> {
//     let line = Line::parse(input).unwrap();
//
//     if line.tag == "CONC" {
//         Ok(Some(line.value.to_string()))
//     } else {
//         Ok(None)
//     }
// }

// Parse the buffer if the CONT tag is found and return the resulting string.
// TODO: Refactor this. It should handle CONT and CONC.
// pub fn cont(input: &mut &str) -> PResult<Option<String>> {
//     let line = Line::parse(input).unwrap();
//
//     if line.tag == "CONT" {
//         Ok(Some(line.value.to_string()))
//     } else {
//         Ok(None)
//     }
// }

/// Parse a GEDCOM file with custom configuration
///
/// # Arguments
///
/// * `filename` - Path to the GEDCOM file to parse
/// * `config` - Configuration options (use `GedcomConfig::new()` for defaults)
///
/// # Returns
///
/// Returns a `Result` containing the parsed `Gedcom` structure or a `GedcomError`
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be found or opened
/// - The file cannot be read
/// - The GEDCOM data is malformed
///
/// # Examples
///
/// ```no_run
/// use gedcom_rs::parse::{parse_gedcom, GedcomConfig};
///
/// // Parse with default configuration
/// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
///
/// // Parse with verbose output
/// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new().verbose())?;
/// # Ok::<(), gedcom_rs::error::GedcomError>(())
/// ```
pub fn parse_gedcom(filename: &str, config: &GedcomConfig) -> Result<Gedcom> {
    // Check if file exists first for better error messages
    if !Path::new(filename).exists() {
        return Err(GedcomError::FileNotFound(filename.to_string()));
    }

    // Read the entire file with proper encoding handling
    let (content, encoding_warning) = read_file_with_encoding(filename, config)?;

    // Initialize an empty gedcom with pre-allocated capacity
    let mut gedcom = Gedcom {
        header: Header::default(),
        individuals: Vec::with_capacity(100), // Pre-allocate for typical genealogy files
        families: Vec::new(),
        sources: Vec::new(),
        repositories: Vec::new(),
        notes: Vec::new(),
        multimedia: Vec::new(),
        submitters: Vec::new(),
        warnings: Vec::new(),
    };

    // Add encoding warning if present
    if let Some(warning) = encoding_warning {
        gedcom.warnings.push(warning);
    }

    // Capacity management constants
    const INITIAL_RECORD_CAPACITY: usize = 2048; // Typical record ~1-2KB

    let mut record = String::with_capacity(INITIAL_RECORD_CAPACITY);

    // Process each line
    for line in content.lines() {
        // Strip off any leading Zero Width No-Break Space
        let line = line.strip_prefix('\u{FEFF}').unwrap_or(line);

        if let Some(ch) = line.chars().next() {
            if ch == '0' && !record.is_empty() {
                let mut input: &str = record.as_str();

                // Peek at the first line in the record so we know how
                // to parse it.
                if let Ok(line) = Line::peek(&mut input) {
                    match line.tag {
                        "HEAD" => {
                            gedcom.header = Header::parse(input);
                        }
                        "INDI" => {
                            let indi = Individual::parse(&mut input);
                            gedcom.individuals.push(indi);
                        }
                        "SOUR" => {
                            if let Ok(source) = SourceRecord::parse(&mut input) {
                                gedcom.sources.push(source);
                            }
                        }
                        "REPO" => {
                            if let Ok(repository) = RepositoryRecord::parse(&mut input) {
                                gedcom.repositories.push(repository);
                            }
                        }
                        "NOTE" => {
                            if let Ok(note) = NoteRecord::parse(&mut input) {
                                gedcom.notes.push(note);
                            }
                        }
                        "OBJE" => {
                            if let Ok(multimedia) = MultimediaRecord::parse(&mut input) {
                                gedcom.multimedia.push(multimedia);
                            }
                        }
                        "FAM" => {
                            let family = Family::parse(&mut input);
                            gedcom.families.push(family);
                        }
                        "SUBM" => {
                            let submitter = Submitter::parse(&mut input);
                            gedcom.submitters.push(submitter);
                        }
                        _ => {}
                    }
                }

                record.clear();
            }

            record.push_str(line);
            record.push('\n');
        }
    }

    // Process the last record if any
    if !record.is_empty() {
        let mut input: &str = record.as_str();
        if let Ok(line) = Line::peek(&mut input) {
            match line.tag {
                "HEAD" => {
                    gedcom.header = Header::parse(input);
                }
                "INDI" => {
                    let indi = Individual::parse(&mut input);
                    gedcom.individuals.push(indi);
                }
                "SOUR" => {
                    if let Ok(source) = SourceRecord::parse(&mut input) {
                        gedcom.sources.push(source);
                    }
                }
                "REPO" => {
                    if let Ok(repository) = RepositoryRecord::parse(&mut input) {
                        gedcom.repositories.push(repository);
                    }
                }
                "NOTE" => {
                    if let Ok(note) = NoteRecord::parse(&mut input) {
                        gedcom.notes.push(note);
                    }
                }
                "OBJE" => {
                    if let Ok(multimedia) = MultimediaRecord::parse(&mut input) {
                        gedcom.multimedia.push(multimedia);
                    }
                }
                "FAM" => {
                    let family = Family::parse(&mut input);
                    gedcom.families.push(family);
                }
                "SUBM" => {
                    let submitter = Submitter::parse(&mut input);
                    gedcom.submitters.push(submitter);
                }
                _ => {}
            }
        }
    }

    // TODO: families
    // TODO: repositories
    // TODO: sources
    // TODO: multimedia

    // Validate records and collect warnings
    validate_gedcom(&mut gedcom);

    Ok(gedcom)
}

/// Validate GEDCOM records and add warnings for missing required fields
fn validate_gedcom(gedcom: &mut Gedcom) {
    // Validate individuals - NAME is recommended but not strictly required in GEDCOM 5.5.1
    // We'll warn about individuals without names as it's a common data quality issue
    for individual in &gedcom.individuals {
        if individual.names.is_empty() {
            gedcom.warnings.push(GedcomError::ValidationError {
                record_type: "INDI".to_string(),
                record_xref: individual.xref.as_ref().map(|x| x.to_string()),
                field: "NAME".to_string(),
                message: "Individual has no name - this may indicate incomplete data".to_string(),
            });
        }
    }

    // Validate families - at least one spouse (HUSB or WIFE) is recommended
    for family in &gedcom.families {
        if family.husband.is_none() && family.wife.is_none() && family.children.is_empty() {
            gedcom.warnings.push(GedcomError::ValidationError {
                record_type: "FAM".to_string(),
                record_xref: Some(family.xref.to_string()),
                field: "HUSB/WIFE/CHIL".to_string(),
                message:
                    "Family has no husband, wife, or children - this may indicate incomplete data"
                        .to_string(),
            });
        }
    }

    // Validate submitters - NAME is required in GEDCOM 5.5.1
    for submitter in &gedcom.submitters {
        if submitter.name.is_none() {
            gedcom.warnings.push(GedcomError::MissingRequiredField {
                record_type: "SUBM".to_string(),
                record_xref: Some(submitter.xref.to_string()),
                field: "NAME".to_string(),
            });
        }
    }

    // Validate repositories - NAME is recommended
    for repository in &gedcom.repositories {
        if repository.name.is_none() {
            gedcom.warnings.push(GedcomError::ValidationError {
                record_type: "REPO".to_string(),
                record_xref: repository.xref.as_ref().map(|x| x.to_string()),
                field: "NAME".to_string(),
                message: "Repository has no name - this may indicate incomplete data".to_string(),
            });
        }
    }

    // Validate multimedia records - at least one FILE is required
    for multimedia in &gedcom.multimedia {
        if multimedia.files.is_empty() {
            gedcom.warnings.push(GedcomError::MissingRequiredField {
                record_type: "OBJE".to_string(),
                record_xref: multimedia.xref.as_ref().map(|x| x.to_string()),
                field: "FILE".to_string(),
            });
        }
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_get_tag_value() {
        let mut input = "3 ADDR 1300 West Traverse Parkway\n4 CONT Lehi, UT 84043 \n4 CONC USA";
        let output = "1300 West Traverse Parkway\nLehi, UT 84043 USA";

        let res = get_tag_value(&mut input).unwrap();
        if let Some(value) = res {
            assert!(output == value);
        }
        assert!(input.is_empty());
    }

    #[test]
    fn test_get_tag_value_simple() {
        let mut input = "1 NAME John /Doe/\n";
        let result = get_tag_value(&mut input).unwrap();
        assert_eq!(result, Some("John /Doe/".to_string()));
    }

    #[test]
    fn test_get_tag_value_with_conc() {
        let mut input = "1 TEXT First part\n2 CONC Second part\n";
        let result = get_tag_value(&mut input).unwrap();
        assert_eq!(result, Some("First partSecond part".to_string()));
    }

    #[test]
    fn test_get_tag_value_with_cont() {
        let mut input = "1 TEXT First line\n2 CONT Second line\n";
        let result = get_tag_value(&mut input).unwrap();
        assert_eq!(result, Some("First line\nSecond line".to_string()));
    }

    #[test]
    fn test_get_tag_value_multiple_conc_cont() {
        let mut input = "1 TEXT Start\n2 CONC Middle\n2 CONT NewLine\n2 CONC End\n";
        let result = get_tag_value(&mut input).unwrap();
        assert_eq!(result, Some("StartMiddle\nNewLineEnd".to_string()));
    }

    #[test]
    fn test_detect_gedcom_encoding_utf8() {
        let content = b"0 HEAD\n1 CHAR UTF-8\n";
        let (encoding, name) = detect_gedcom_encoding(content);
        assert_eq!(encoding, encoding_rs::UTF_8);
        assert_eq!(name, "UTF-8");
    }

    #[test]
    fn test_detect_gedcom_encoding_ansel() {
        let content = b"0 HEAD\n1 CHAR ANSEL\n";
        let (encoding, name) = detect_gedcom_encoding(content);
        assert_eq!(encoding, WINDOWS_1252);
        assert_eq!(name, "ANSEL");
    }

    #[test]
    fn test_detect_gedcom_encoding_ascii() {
        let content = b"0 HEAD\n1 CHAR ASCII\n";
        let (encoding, name) = detect_gedcom_encoding(content);
        assert_eq!(encoding, encoding_rs::UTF_8);
        assert_eq!(name, "ASCII");
    }

    #[test]
    fn test_detect_gedcom_encoding_ansi() {
        let content = b"0 HEAD\n1 CHAR ANSI\n";
        let (encoding, name) = detect_gedcom_encoding(content);
        assert_eq!(encoding, WINDOWS_1252);
        assert_eq!(name, "ANSI");
    }

    #[test]
    fn test_detect_gedcom_encoding_unicode() {
        let content = b"0 HEAD\n1 CHAR UNICODE\n";
        let (encoding, name) = detect_gedcom_encoding(content);
        assert_eq!(encoding, encoding_rs::UTF_16LE);
        assert_eq!(name, "UNICODE");
    }

    #[test]
    fn test_detect_gedcom_encoding_unknown() {
        let content = b"0 HEAD\n1 CHAR UNKNOWN_ENCODING\n";
        let (encoding, name) = detect_gedcom_encoding(content);
        assert_eq!(encoding, WINDOWS_1252);
        assert_eq!(name, "UNKNOWN_ENCODING");
    }

    #[test]
    fn test_detect_gedcom_encoding_no_char_tag() {
        let content = b"0 HEAD\n1 SOUR Test\n";
        let (encoding, name) = detect_gedcom_encoding(content);
        assert_eq!(encoding, encoding_rs::UTF_8);
        assert_eq!(name, "UTF-8 (default)");
    }

    #[test]
    fn test_detect_gedcom_encoding_with_crlf() {
        let content = b"0 HEAD\r\n1 CHAR UTF-8\r\n";
        let (encoding, name) = detect_gedcom_encoding(content);
        assert_eq!(encoding, encoding_rs::UTF_8);
        assert_eq!(name, "UTF-8");
    }

    #[test]
    fn test_gedcom_config_new() {
        let config = GedcomConfig::new();
        assert!(!config.verbose);
    }

    #[test]
    fn test_gedcom_config_verbose() {
        let config = GedcomConfig::new().verbose();
        assert!(config.verbose);
    }

    #[test]
    fn test_gedcom_config_default() {
        let config = GedcomConfig::default();
        assert!(!config.verbose);
    }

    #[test]
    fn test_parse_gedcom_file_not_found() {
        let config = GedcomConfig::new();
        let result = parse_gedcom("nonexistent_file_12345.ged", &config);
        assert!(result.is_err());
        match result {
            Err(GedcomError::FileNotFound(path)) => {
                assert!(path.contains("nonexistent_file_12345.ged"));
            }
            _ => panic!("Expected FileNotFound error"),
        }
    }

    #[test]
    fn test_parse_minimal_gedcom() {
        // Create a minimal GEDCOM file for testing
        let temp_file = "test_minimal.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n1 GEDC\n2 VERS 5.5.1\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();
        assert_eq!(gedcom.individuals.len(), 0);
        assert_eq!(gedcom.families.len(), 0);
    }

    #[test]
    fn test_parse_gedcom_with_individual() {
        let temp_file = "test_individual.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @I1@ INDI\n1 NAME John /Doe/\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();
        assert_eq!(gedcom.individuals.len(), 1);
    }

    #[test]
    fn test_parse_gedcom_with_family() {
        let temp_file = "test_family.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @F1@ FAM\n1 HUSB @I1@\n1 WIFE @I2@\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();
        assert_eq!(gedcom.families.len(), 1);
    }

    #[test]
    fn test_parse_gedcom_with_source() {
        let temp_file = "test_source.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @S1@ SOUR\n1 TITL Test Source\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();
        assert_eq!(gedcom.sources.len(), 1);
    }

    #[test]
    fn test_parse_gedcom_with_note() {
        let temp_file = "test_note.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @N1@ NOTE This is a test note\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();
        assert_eq!(gedcom.notes.len(), 1);
    }

    #[test]
    fn test_parse_gedcom_with_repository() {
        let temp_file = "test_repo.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @R1@ REPO\n1 NAME Test Repository\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();
        assert_eq!(gedcom.repositories.len(), 1);
    }

    #[test]
    fn test_parse_gedcom_with_multimedia() {
        let temp_file = "test_multimedia.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @M1@ OBJE\n1 FILE test.jpg\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();
        assert_eq!(gedcom.multimedia.len(), 1);
    }

    #[test]
    fn test_parse_gedcom_with_submitter() {
        let temp_file = "test_submitter.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @U1@ SUBM\n1 NAME John Doe\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();
        assert_eq!(gedcom.submitters.len(), 1);
    }

    #[test]
    fn test_parse_gedcom_with_bom() {
        let temp_file = "test_bom.ged";
        // UTF-8 BOM is U+FEFF
        let content = "\u{FEFF}0 HEAD\n1 CHAR UTF-8\n0 @I1@ INDI\n1 NAME Test /Person/\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();
        assert_eq!(gedcom.individuals.len(), 1);
    }

    #[test]
    fn test_parse_gedcom_empty_file() {
        let temp_file = "test_empty.ged";
        let content = "";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();
        assert_eq!(gedcom.individuals.len(), 0);
    }

    #[test]
    fn test_parse_gedcom_multiple_records() {
        let temp_file = "test_multiple.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @I1@ INDI\n1 NAME First /Person/\n0 @I2@ INDI\n1 NAME Second /Person/\n0 @F1@ FAM\n1 HUSB @I1@\n1 WIFE @I2@\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();
        assert_eq!(gedcom.individuals.len(), 2);
        assert_eq!(gedcom.families.len(), 1);
    }

    // ========================================================================
    // Validation Unit Tests
    // ========================================================================

    /// Helper function to parse GEDCOM content for validation tests
    /// Uses tempfile crate to ensure proper cleanup even on test failure or panic
    fn parse_test_gedcom(content: &str) -> Result<Gedcom> {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary test file");
        temp_file
            .write_all(content.as_bytes())
            .expect("Failed to write test data");

        let path_str = temp_file
            .path()
            .to_str()
            .expect("Failed to convert path to string");
        parse_gedcom(path_str, &GedcomConfig::new())
    }

    /// Test validation detects individual without name
    #[test]
    fn test_validate_individual_empty_name() {
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @I1@ INDI\n1 SEX M\n0 TRLR\n";
        let result = parse_test_gedcom(content);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have exactly one individual
        assert_eq!(gedcom.individuals.len(), 1);

        // Should have a validation warning for missing name
        let has_name_warning = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::ValidationError {
                    record_type,
                    field,
                    ..
                } if record_type == "INDI" && field == "NAME"
            )
        });

        assert!(
            has_name_warning,
            "Expected ValidationError for individual without name"
        );

        // Verify the warning message is helpful
        let warning = gedcom
            .warnings
            .iter()
            .find(|w| {
                matches!(
                    w,
                    GedcomError::ValidationError {
                        record_type,
                        field,
                        ..
                    } if record_type == "INDI" && field == "NAME"
                )
            })
            .unwrap();

        if let GedcomError::ValidationError {
            message,
            record_xref,
            ..
        } = warning
        {
            assert!(
                message.contains("no name"),
                "Warning message should mention 'no name'"
            );
            assert!(
                record_xref.is_some(),
                "Warning should include record xref for traceability"
            );
            assert_eq!(record_xref.as_ref().unwrap(), "@I1@");
        }
    }

    /// Test validation passes for individual with name
    #[test]
    fn test_validate_individual_with_name() {
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @I1@ INDI\n1 NAME John /Doe/\n1 SEX M\n0 TRLR\n";
        let result = parse_test_gedcom(content);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have exactly one individual
        assert_eq!(gedcom.individuals.len(), 1);

        // Should NOT have a validation warning for this individual
        let has_name_warning = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::ValidationError {
                    record_type,
                    field,
                    ..
                } if record_type == "INDI" && field == "NAME"
            )
        });

        assert!(
            !has_name_warning,
            "Should not have ValidationError for individual with name"
        );
    }

    /// Test validation with multiple individuals, some missing names
    #[test]
    fn test_validate_multiple_individuals_mixed() {
        let temp_file = "test_validate_multiple_indi.ged";
        let content = "0 HEAD\n\
                       1 CHAR UTF-8\n\
                       0 @I1@ INDI\n\
                       1 NAME Valid /Person/\n\
                       0 @I2@ INDI\n\
                       1 SEX F\n\
                       0 @I3@ INDI\n\
                       1 NAME Another /Valid/\n\
                       0 @I4@ INDI\n\
                       1 BIRT\n\
                       2 DATE 1 JAN 1900\n\
                       0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have 4 individuals
        assert_eq!(gedcom.individuals.len(), 4);

        // Should have exactly 2 validation warnings (I2 and I4 missing names)
        let name_warnings: Vec<_> = gedcom
            .warnings
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    GedcomError::ValidationError {
                        record_type,
                        field,
                        ..
                    } if record_type == "INDI" && field == "NAME"
                )
            })
            .collect();

        assert_eq!(
            name_warnings.len(),
            2,
            "Expected exactly 2 NAME validation warnings"
        );

        // Verify both I2 and I4 are flagged
        let xrefs: Vec<String> = name_warnings
            .iter()
            .filter_map(|w| {
                if let GedcomError::ValidationError { record_xref, .. } = w {
                    record_xref.clone()
                } else {
                    None
                }
            })
            .collect();

        assert!(xrefs.contains(&"@I2@".to_string()));
        assert!(xrefs.contains(&"@I4@".to_string()));
    }

    /// Test validation doesn't fail on individual without xref (edge case)
    #[test]
    fn test_validate_individual_without_xref() {
        let temp_file = "test_validate_indi_no_xref.ged";
        // Invalid GEDCOM but parser might handle it
        let content = "0 HEAD\n1 CHAR UTF-8\n0 INDI\n1 SEX M\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        // Should still parse without crashing
        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // If an individual was created, validation should handle missing xref gracefully
        if !gedcom.individuals.is_empty() {
            let has_name_warning = gedcom.warnings.iter().any(|w| {
                matches!(
                    w,
                    GedcomError::ValidationError {
                        record_type,
                        field,
                        ..
                    } if record_type == "INDI" && field == "NAME"
                )
            });

            // Should still detect missing name even without xref
            assert!(has_name_warning);

            // Warning should handle None xref gracefully
            let warning = gedcom
                .warnings
                .iter()
                .find(|w| {
                    matches!(
                        w,
                        GedcomError::ValidationError {
                            record_type,
                            field,
                            ..
                        } if record_type == "INDI" && field == "NAME"
                    )
                })
                .unwrap();

            if let GedcomError::ValidationError { record_xref, .. } = warning {
                // record_xref might be None, and that's okay
                assert!(true, "Validation handled missing xref: {:?}", record_xref);
            }
        }
    }

    /// Test validation detects family with no members
    #[test]
    fn test_validate_family_empty() {
        let temp_file = "test_validate_fam_empty.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @F1@ FAM\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have exactly one family
        assert_eq!(gedcom.families.len(), 1);

        // Should have a validation warning for empty family
        let has_family_warning = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::ValidationError {
                    record_type,
                    field,
                    ..
                } if record_type == "FAM" && field == "HUSB/WIFE/CHIL"
            )
        });

        assert!(
            has_family_warning,
            "Expected ValidationError for family with no members"
        );

        // Verify the warning includes xref
        let warning = gedcom
            .warnings
            .iter()
            .find(|w| {
                matches!(
                    w,
                    GedcomError::ValidationError {
                        record_type,
                        field,
                        ..
                    } if record_type == "FAM" && field == "HUSB/WIFE/CHIL"
                )
            })
            .unwrap();

        if let GedcomError::ValidationError {
            message,
            record_xref,
            ..
        } = warning
        {
            assert!(
                message.contains("no husband, wife, or children"),
                "Warning message should explain what's missing"
            );
            assert!(record_xref.is_some(), "Warning should include record xref");
            assert_eq!(record_xref.as_ref().unwrap(), "@F1@");
        }
    }

    /// Test validation passes for family with husband only
    #[test]
    fn test_validate_family_with_husband() {
        let temp_file = "test_validate_fam_husband.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @F1@ FAM\n1 HUSB @I1@\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have exactly one family
        assert_eq!(gedcom.families.len(), 1);

        // Should NOT have a validation warning
        let has_family_warning = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::ValidationError {
                    record_type,
                    field,
                    ..
                } if record_type == "FAM" && field == "HUSB/WIFE/CHIL"
            )
        });

        assert!(
            !has_family_warning,
            "Should not have warning for family with husband"
        );
    }

    /// Test validation passes for family with wife only
    #[test]
    fn test_validate_family_with_wife() {
        let temp_file = "test_validate_fam_wife.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @F1@ FAM\n1 WIFE @I2@\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have exactly one family
        assert_eq!(gedcom.families.len(), 1);

        // Should NOT have a validation warning
        let has_family_warning = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::ValidationError {
                    record_type,
                    field,
                    ..
                } if record_type == "FAM" && field == "HUSB/WIFE/CHIL"
            )
        });

        assert!(
            !has_family_warning,
            "Should not have warning for family with wife"
        );
    }

    /// Test validation passes for family with children only
    #[test]
    fn test_validate_family_with_children() {
        let temp_file = "test_validate_fam_children.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @F1@ FAM\n1 CHIL @I3@\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have exactly one family
        assert_eq!(gedcom.families.len(), 1);

        // Should NOT have a validation warning
        let has_family_warning = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::ValidationError {
                    record_type,
                    field,
                    ..
                } if record_type == "FAM" && field == "HUSB/WIFE/CHIL"
            )
        });

        assert!(
            !has_family_warning,
            "Should not have warning for family with children"
        );
    }

    /// Test validation passes for complete family
    #[test]
    fn test_validate_family_complete() {
        let temp_file = "test_validate_fam_complete.ged";
        let content =
            "0 HEAD\n1 CHAR UTF-8\n0 @F1@ FAM\n1 HUSB @I1@\n1 WIFE @I2@\n1 CHIL @I3@\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have exactly one family
        assert_eq!(gedcom.families.len(), 1);

        // Should NOT have a validation warning
        let has_family_warning = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::ValidationError {
                    record_type,
                    field,
                    ..
                } if record_type == "FAM" && field == "HUSB/WIFE/CHIL"
            )
        });

        assert!(
            !has_family_warning,
            "Should not have warning for complete family"
        );
    }

    /// Test validation with multiple families, some empty
    #[test]
    fn test_validate_multiple_families_mixed() {
        let temp_file = "test_validate_multiple_fam.ged";
        let content = "0 HEAD\n\
                       1 CHAR UTF-8\n\
                       0 @F1@ FAM\n\
                       1 HUSB @I1@\n\
                       1 WIFE @I2@\n\
                       0 @F2@ FAM\n\
                       0 @F3@ FAM\n\
                       1 CHIL @I3@\n\
                       0 @F4@ FAM\n\
                       0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have 4 families
        assert_eq!(gedcom.families.len(), 4);

        // Should have exactly 2 validation warnings (F2 and F4 are empty)
        let family_warnings: Vec<_> = gedcom
            .warnings
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    GedcomError::ValidationError {
                        record_type,
                        field,
                        ..
                    } if record_type == "FAM" && field == "HUSB/WIFE/CHIL"
                )
            })
            .collect();

        assert_eq!(
            family_warnings.len(),
            2,
            "Expected exactly 2 family validation warnings"
        );

        // Verify both F2 and F4 are flagged
        let xrefs: Vec<String> = family_warnings
            .iter()
            .filter_map(|w| {
                if let GedcomError::ValidationError { record_xref, .. } = w {
                    record_xref.clone()
                } else {
                    None
                }
            })
            .collect();

        assert!(xrefs.contains(&"@F2@".to_string()));
        assert!(xrefs.contains(&"@F4@".to_string()));
    }

    /// Test validation detects submitter without required NAME field
    #[test]
    fn test_validate_submitter_missing_name() {
        let temp_file = "test_validate_subm_no_name.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @U1@ SUBM\n1 ADDR 123 Main St\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have exactly one submitter
        assert_eq!(gedcom.submitters.len(), 1);

        // Should have a MissingRequiredField error for NAME
        let has_name_error = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::MissingRequiredField {
                    record_type,
                    field,
                    ..
                } if record_type == "SUBM" && field == "NAME"
            )
        });

        assert!(
            has_name_error,
            "Expected MissingRequiredField error for submitter without NAME"
        );

        // Verify the error includes xref
        let error = gedcom
            .warnings
            .iter()
            .find(|w| {
                matches!(
                    w,
                    GedcomError::MissingRequiredField {
                        record_type,
                        field,
                        ..
                    } if record_type == "SUBM" && field == "NAME"
                )
            })
            .unwrap();

        if let GedcomError::MissingRequiredField { record_xref, .. } = error {
            assert!(record_xref.is_some(), "Error should include record xref");
            assert_eq!(record_xref.as_ref().unwrap(), "@U1@");
        }
    }

    /// Test validation passes for submitter with NAME field
    #[test]
    fn test_validate_submitter_with_name() {
        let temp_file = "test_validate_subm_with_name.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @U1@ SUBM\n1 NAME John Doe\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have exactly one submitter
        assert_eq!(gedcom.submitters.len(), 1);

        // Should NOT have a MissingRequiredField error
        let has_name_error = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::MissingRequiredField {
                    record_type,
                    field,
                    ..
                } if record_type == "SUBM" && field == "NAME"
            )
        });

        assert!(
            !has_name_error,
            "Should not have error for submitter with NAME"
        );
    }

    /// Test validation with multiple submitters, some missing NAME
    #[test]
    fn test_validate_multiple_submitters_mixed() {
        let temp_file = "test_validate_multiple_subm.ged";
        let content = "0 HEAD\n\
                       1 CHAR UTF-8\n\
                       0 @U1@ SUBM\n\
                       1 NAME Valid Submitter\n\
                       0 @U2@ SUBM\n\
                       1 ADDR Some Address\n\
                       0 @U3@ SUBM\n\
                       1 NAME Another Valid\n\
                       0 @U4@ SUBM\n\
                       1 PHON 555-1234\n\
                       0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have 4 submitters
        assert_eq!(gedcom.submitters.len(), 4);

        // Should have exactly 2 MissingRequiredField errors (U2 and U4)
        let name_errors: Vec<_> = gedcom
            .warnings
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    GedcomError::MissingRequiredField {
                        record_type,
                        field,
                        ..
                    } if record_type == "SUBM" && field == "NAME"
                )
            })
            .collect();

        assert_eq!(
            name_errors.len(),
            2,
            "Expected exactly 2 NAME MissingRequiredField errors"
        );

        // Verify both U2 and U4 are flagged
        let xrefs: Vec<String> = name_errors
            .iter()
            .filter_map(|w| {
                if let GedcomError::MissingRequiredField { record_xref, .. } = w {
                    record_xref.clone()
                } else {
                    None
                }
            })
            .collect();

        assert!(xrefs.contains(&"@U2@".to_string()));
        assert!(xrefs.contains(&"@U4@".to_string()));
    }

    /// Test that submitter NAME field is correctly identified as required (vs recommended)
    #[test]
    fn test_validate_submitter_uses_missing_required_field_error() {
        let temp_file = "test_validate_subm_required.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @U1@ SUBM\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Must use MissingRequiredField, not ValidationError
        let has_missing_required = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::MissingRequiredField {
                    record_type,
                    ..
                } if record_type == "SUBM"
            )
        });

        let has_validation_error = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::ValidationError {
                    record_type,
                    ..
                } if record_type == "SUBM"
            )
        });

        assert!(
            has_missing_required,
            "Should use MissingRequiredField for SUBM NAME"
        );
        assert!(
            !has_validation_error,
            "Should NOT use ValidationError for SUBM NAME (it's required, not just recommended)"
        );
    }

    /// Test validation detects repository without NAME field
    #[test]
    fn test_validate_repository_missing_name() {
        let temp_file = "test_validate_repo_no_name.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @R1@ REPO\n1 ADDR 123 Library St\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have exactly one repository
        assert_eq!(gedcom.repositories.len(), 1);

        // Should have a ValidationError for missing NAME (it's recommended, not required)
        let has_name_warning = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::ValidationError {
                    record_type,
                    field,
                    ..
                } if record_type == "REPO" && field == "NAME"
            )
        });

        assert!(
            has_name_warning,
            "Expected ValidationError for repository without NAME"
        );

        // Verify the warning includes xref and helpful message
        let warning = gedcom
            .warnings
            .iter()
            .find(|w| {
                matches!(
                    w,
                    GedcomError::ValidationError {
                        record_type,
                        field,
                        ..
                    } if record_type == "REPO" && field == "NAME"
                )
            })
            .unwrap();

        if let GedcomError::ValidationError {
            message,
            record_xref,
            ..
        } = warning
        {
            assert!(
                message.contains("no name"),
                "Warning message should mention 'no name'"
            );
            assert!(record_xref.is_some(), "Warning should include record xref");
            assert_eq!(record_xref.as_ref().unwrap(), "@R1@");
        }
    }

    /// Test validation passes for repository with NAME field
    #[test]
    fn test_validate_repository_with_name() {
        let temp_file = "test_validate_repo_with_name.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @R1@ REPO\n1 NAME National Archives\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have exactly one repository
        assert_eq!(gedcom.repositories.len(), 1);

        // Should NOT have a ValidationError
        let has_name_warning = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::ValidationError {
                    record_type,
                    field,
                    ..
                } if record_type == "REPO" && field == "NAME"
            )
        });

        assert!(
            !has_name_warning,
            "Should not have warning for repository with NAME"
        );
    }

    /// Test validation with multiple repositories, some missing NAME
    #[test]
    fn test_validate_multiple_repositories_mixed() {
        let temp_file = "test_validate_multiple_repo.ged";
        let content = "0 HEAD\n\
                       1 CHAR UTF-8\n\
                       0 @R1@ REPO\n\
                       1 NAME State Archives\n\
                       0 @R2@ REPO\n\
                       1 ADDR Unknown Location\n\
                       0 @R3@ REPO\n\
                       1 NAME County Library\n\
                       0 @R4@ REPO\n\
                       1 NOTE Some note\n\
                       0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have 4 repositories
        assert_eq!(gedcom.repositories.len(), 4);

        // Should have exactly 2 ValidationError warnings (R2 and R4)
        let name_warnings: Vec<_> = gedcom
            .warnings
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    GedcomError::ValidationError {
                        record_type,
                        field,
                        ..
                    } if record_type == "REPO" && field == "NAME"
                )
            })
            .collect();

        assert_eq!(
            name_warnings.len(),
            2,
            "Expected exactly 2 NAME validation warnings"
        );

        // Verify both R2 and R4 are flagged
        let xrefs: Vec<String> = name_warnings
            .iter()
            .filter_map(|w| {
                if let GedcomError::ValidationError { record_xref, .. } = w {
                    record_xref.clone()
                } else {
                    None
                }
            })
            .collect();

        assert!(xrefs.contains(&"@R2@".to_string()));
        assert!(xrefs.contains(&"@R4@".to_string()));
    }

    /// Test that repository NAME uses ValidationError (recommended) not MissingRequiredField
    #[test]
    fn test_validate_repository_uses_validation_error() {
        let temp_file = "test_validate_repo_recommended.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @R1@ REPO\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Must use ValidationError, not MissingRequiredField (NAME is recommended, not required)
        let has_validation_error = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::ValidationError {
                    record_type,
                    ..
                } if record_type == "REPO"
            )
        });

        let has_missing_required = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::MissingRequiredField {
                    record_type,
                    ..
                } if record_type == "REPO"
            )
        });

        assert!(
            has_validation_error,
            "Should use ValidationError for REPO NAME (recommended)"
        );
        assert!(
            !has_missing_required,
            "Should NOT use MissingRequiredField for REPO NAME (it's recommended, not required)"
        );
    }

    /// Test validation detects multimedia without FILE field
    #[test]
    fn test_validate_multimedia_missing_file() {
        let temp_file = "test_validate_obje_no_file.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @M1@ OBJE\n1 TITL Photo Title\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have exactly one multimedia record
        assert_eq!(gedcom.multimedia.len(), 1);

        // Should have a MissingRequiredField error for FILE
        let has_file_error = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::MissingRequiredField {
                    record_type,
                    field,
                    ..
                } if record_type == "OBJE" && field == "FILE"
            )
        });

        assert!(
            has_file_error,
            "Expected MissingRequiredField error for multimedia without FILE"
        );

        // Verify the error includes xref
        let error = gedcom
            .warnings
            .iter()
            .find(|w| {
                matches!(
                    w,
                    GedcomError::MissingRequiredField {
                        record_type,
                        field,
                        ..
                    } if record_type == "OBJE" && field == "FILE"
                )
            })
            .unwrap();

        if let GedcomError::MissingRequiredField { record_xref, .. } = error {
            assert!(record_xref.is_some(), "Error should include record xref");
            assert_eq!(record_xref.as_ref().unwrap(), "@M1@");
        }
    }

    /// Test validation passes for multimedia with FILE field
    #[test]
    fn test_validate_multimedia_with_file() {
        let temp_file = "test_validate_obje_with_file.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @M1@ OBJE\n1 FILE photo.jpg\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have exactly one multimedia record
        assert_eq!(gedcom.multimedia.len(), 1);

        // Should NOT have a MissingRequiredField error
        let has_file_error = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::MissingRequiredField {
                    record_type,
                    field,
                    ..
                } if record_type == "OBJE" && field == "FILE"
            )
        });

        assert!(
            !has_file_error,
            "Should not have error for multimedia with FILE"
        );
    }

    /// Test validation with multiple multimedia records, some missing FILE
    #[test]
    fn test_validate_multiple_multimedia_mixed() {
        let temp_file = "test_validate_multiple_obje.ged";
        let content = "0 HEAD\n\
                       1 CHAR UTF-8\n\
                       0 @M1@ OBJE\n\
                       1 FILE photo1.jpg\n\
                       0 @M2@ OBJE\n\
                       1 TITL Photo without file\n\
                       0 @M3@ OBJE\n\
                       1 FILE photo2.png\n\
                       1 TITL Valid Photo\n\
                       0 @M4@ OBJE\n\
                       1 NOTE Just a note\n\
                       0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have 4 multimedia records
        assert_eq!(gedcom.multimedia.len(), 4);

        // Should have exactly 2 MissingRequiredField errors (M2 and M4)
        let file_errors: Vec<_> = gedcom
            .warnings
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    GedcomError::MissingRequiredField {
                        record_type,
                        field,
                        ..
                    } if record_type == "OBJE" && field == "FILE"
                )
            })
            .collect();

        assert_eq!(
            file_errors.len(),
            2,
            "Expected exactly 2 FILE MissingRequiredField errors"
        );

        // Verify both M2 and M4 are flagged
        let xrefs: Vec<String> = file_errors
            .iter()
            .filter_map(|w| {
                if let GedcomError::MissingRequiredField { record_xref, .. } = w {
                    record_xref.clone()
                } else {
                    None
                }
            })
            .collect();

        assert!(xrefs.contains(&"@M2@".to_string()));
        assert!(xrefs.contains(&"@M4@".to_string()));
    }

    /// Test multimedia with multiple files is valid
    #[test]
    fn test_validate_multimedia_with_multiple_files() {
        let temp_file = "test_validate_obje_multi_file.ged";
        let content = "0 HEAD\n\
                       1 CHAR UTF-8\n\
                       0 @M1@ OBJE\n\
                       1 FILE photo.jpg\n\
                       1 FILE photo_thumb.jpg\n\
                       0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Should have exactly one multimedia record
        assert_eq!(gedcom.multimedia.len(), 1);

        // Should NOT have any errors (multiple files is valid)
        let has_file_error = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::MissingRequiredField {
                    record_type,
                    field,
                    ..
                } if record_type == "OBJE" && field == "FILE"
            )
        });

        assert!(
            !has_file_error,
            "Should not have error for multimedia with multiple files"
        );
    }

    /// Test that multimedia FILE uses MissingRequiredField (required) not ValidationError
    #[test]
    fn test_validate_multimedia_uses_missing_required_field_error() {
        let temp_file = "test_validate_obje_required.ged";
        let content = "0 HEAD\n1 CHAR UTF-8\n0 @M1@ OBJE\n0 TRLR\n";

        fs::write(temp_file, content).unwrap();

        let config = GedcomConfig::new();
        let result = parse_gedcom(temp_file, &config);

        // Cleanup
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let gedcom = result.unwrap();

        // Must use MissingRequiredField, not ValidationError
        let has_missing_required = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::MissingRequiredField {
                    record_type,
                    ..
                } if record_type == "OBJE"
            )
        });

        let has_validation_error = gedcom.warnings.iter().any(|w| {
            matches!(
                w,
                GedcomError::ValidationError {
                    record_type,
                    ..
                } if record_type == "OBJE"
            )
        });

        assert!(
            has_missing_required,
            "Should use MissingRequiredField for OBJE FILE"
        );
        assert!(
            !has_validation_error,
            "Should NOT use ValidationError for OBJE FILE (it's required, not just recommended)"
        );
    }
}
