use std::fmt;
use std::io;

/// Errors that can occur during GEDCOM parsing
#[derive(Debug)]
pub enum GedcomError {
    /// Error reading the GEDCOM file
    Io(io::Error),

    /// The file path provided does not exist or cannot be accessed
    FileNotFound(String),

    /// Error parsing a specific line or record
    ParseError {
        line_number: Option<usize>,
        record_type: Option<String>,
        field: Option<String>,
        message: String,
        context: Option<String>,
    },

    /// Invalid GEDCOM structure (e.g., missing required fields)
    InvalidStructure {
        record_xref: Option<String>,
        message: String,
    },

    /// Validation error for a specific field
    ValidationError {
        record_type: String,
        record_xref: Option<String>,
        field: String,
        message: String,
    },

    /// Character encoding issues during file reading
    EncodingError {
        declared_encoding: String,
        detected_encoding: Option<String>,
        message: String,
        had_errors: bool,
    },

    /// Required field is missing from a record
    MissingRequiredField {
        record_type: String,
        record_xref: Option<String>,
        field: String,
    },
}

impl fmt::Display for GedcomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GedcomError::Io(err) => write!(f, "I/O error: {}", err),
            GedcomError::FileNotFound(path) => {
                writeln!(f, "File not found: '{}'", path)?;
                write!(
                    f,
                    "  Hint: Check that the file path is correct and the file exists"
                )
            }
            GedcomError::ParseError {
                line_number,
                record_type,
                field,
                message,
                context,
            } => {
                write!(f, "Parse error")?;
                if let Some(line) = line_number {
                    write!(f, " at line {}", line)?;
                }
                if let Some(rec_type) = record_type {
                    write!(f, " in {} record", rec_type)?;
                }
                if let Some(fld) = field {
                    write!(f, ", field '{}'", fld)?;
                }
                write!(f, ": {}", message)?;
                if let Some(ctx) = context {
                    write!(f, "\n  Context: {}", ctx)?;
                }
                Ok(())
            }
            GedcomError::InvalidStructure {
                record_xref,
                message,
            } => {
                write!(f, "Invalid GEDCOM structure")?;
                if let Some(xref) = record_xref {
                    write!(f, " in record {}", xref)?;
                }
                write!(f, ": {}", message)
            }
            GedcomError::ValidationError {
                record_type,
                record_xref,
                field,
                message,
            } => {
                write!(f, "Validation error in {} record", record_type)?;
                if let Some(xref) = record_xref {
                    write!(f, " ({})", xref)?;
                }
                write!(f, ", field '{}': {}", field, message)
            }
            GedcomError::EncodingError {
                declared_encoding,
                detected_encoding,
                message,
                had_errors,
            } => {
                write!(
                    f,
                    "Character encoding error: declared as '{}', ",
                    declared_encoding
                )?;
                if let Some(detected) = detected_encoding {
                    write!(f, "detected as '{}', ", detected)?;
                }
                write!(f, "{}", message)?;
                if *had_errors {
                    write!(
                        f,
                        "\n  Warning: Some characters could not be converted correctly"
                    )?;
                }
                Ok(())
            }
            GedcomError::MissingRequiredField {
                record_type,
                record_xref,
                field,
            } => {
                write!(
                    f,
                    "Missing required field '{}' in {} record",
                    field, record_type
                )?;
                if let Some(xref) = record_xref {
                    write!(f, " ({})", xref)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for GedcomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GedcomError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for GedcomError {
    fn from(err: io::Error) -> Self {
        GedcomError::Io(err)
    }
}

/// Type alias for Results in GEDCOM parsing
pub type Result<T> = std::result::Result<T, GedcomError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_io_error_display() {
        let err = GedcomError::Io(io::Error::new(io::ErrorKind::NotFound, "file not found"));
        let msg = format!("{}", err);
        assert!(msg.contains("I/O error"));
    }

    #[test]
    fn test_file_not_found_display() {
        let err = GedcomError::FileNotFound("test.ged".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("File not found"));
        assert!(msg.contains("test.ged"));
    }

    #[test]
    fn test_parse_error_with_line_number() {
        let err = GedcomError::ParseError {
            line_number: Some(42),
            record_type: None,
            field: None,
            message: "Invalid syntax".to_string(),
            context: None,
        };
        assert!(format!("{}", err).contains("line 42"));
        assert!(format!("{}", err).contains("Invalid syntax"));
    }

    #[test]
    fn test_parse_error_without_line_number() {
        let err = GedcomError::ParseError {
            line_number: None,
            record_type: None,
            field: None,
            message: "Invalid syntax".to_string(),
            context: None,
        };
        assert!(format!("{}", err).contains("Parse error"));
        assert!(format!("{}", err).contains("Invalid syntax"));
    }

    #[test]
    fn test_parse_error_with_full_context() {
        let err = GedcomError::ParseError {
            line_number: Some(42),
            record_type: Some("INDI".to_string()),
            field: Some("NAME".to_string()),
            message: "Invalid name format".to_string(),
            context: Some("1 NAME /Invalid/Name/".to_string()),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("line 42"));
        assert!(msg.contains("INDI"));
        assert!(msg.contains("NAME"));
        assert!(msg.contains("Invalid name format"));
        assert!(msg.contains("Context"));
    }

    #[test]
    fn test_invalid_structure_display() {
        let err = GedcomError::InvalidStructure {
            record_xref: Some("@I1@".to_string()),
            message: "Missing required field".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid GEDCOM structure"));
        assert!(msg.contains("@I1@"));
        assert!(msg.contains("Missing required field"));
    }

    #[test]
    fn test_validation_error_display() {
        let err = GedcomError::ValidationError {
            record_type: "INDI".to_string(),
            record_xref: Some("@I1@".to_string()),
            field: "NAME".to_string(),
            message: "Name cannot be empty".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("Validation error"));
        assert!(msg.contains("INDI"));
        assert!(msg.contains("@I1@"));
        assert!(msg.contains("NAME"));
        assert!(msg.contains("Name cannot be empty"));
    }

    #[test]
    fn test_encoding_error_display() {
        let err = GedcomError::EncodingError {
            declared_encoding: "ANSEL".to_string(),
            detected_encoding: Some("UTF-8".to_string()),
            message: "Using UTF-8 fallback".to_string(),
            had_errors: true,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("encoding error"));
        assert!(msg.contains("ANSEL"));
        assert!(msg.contains("UTF-8"));
        assert!(msg.contains("Warning"));
    }

    #[test]
    fn test_missing_required_field_display() {
        let err = GedcomError::MissingRequiredField {
            record_type: "INDI".to_string(),
            record_xref: Some("@I1@".to_string()),
            field: "NAME".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("Missing required field"));
        assert!(msg.contains("NAME"));
        assert!(msg.contains("INDI"));
        assert!(msg.contains("@I1@"));
    }

    #[test]
    fn test_file_not_found_includes_hint() {
        let err = GedcomError::FileNotFound("missing.ged".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("File not found"));
        assert!(msg.contains("missing.ged"));
        assert!(msg.contains("Hint"));
    }

    #[test]
    fn test_io_error_source() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "test");
        let err = GedcomError::Io(io_err);
        assert!(err.source().is_some());
    }

    #[test]
    fn test_other_errors_no_source() {
        let err = GedcomError::FileNotFound("test.ged".to_string());
        assert!(err.source().is_none());

        let err = GedcomError::ParseError {
            line_number: None,
            record_type: None,
            field: None,
            message: "test".to_string(),
            context: None,
        };
        assert!(err.source().is_none());

        let err = GedcomError::InvalidStructure {
            record_xref: None,
            message: "test".to_string(),
        };
        assert!(err.source().is_none());
    }

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
        let gedcom_err: GedcomError = io_err.into();
        assert!(matches!(gedcom_err, GedcomError::Io(_)));
    }
}
