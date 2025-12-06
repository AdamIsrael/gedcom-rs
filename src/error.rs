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
        message: String,
    },

    /// Invalid GEDCOM structure (e.g., missing required fields)
    InvalidStructure(String),
}

impl fmt::Display for GedcomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GedcomError::Io(err) => write!(f, "I/O error: {}", err),
            GedcomError::FileNotFound(path) => write!(f, "File not found: {}", path),
            GedcomError::ParseError {
                line_number,
                message,
            } => {
                if let Some(line) = line_number {
                    write!(f, "Parse error at line {}: {}", line, message)
                } else {
                    write!(f, "Parse error: {}", message)
                }
            }
            GedcomError::InvalidStructure(msg) => write!(f, "Invalid GEDCOM structure: {}", msg),
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
        assert_eq!(format!("{}", err), "File not found: test.ged");
    }

    #[test]
    fn test_parse_error_with_line_number() {
        let err = GedcomError::ParseError {
            line_number: Some(42),
            message: "Invalid syntax".to_string(),
        };
        assert_eq!(format!("{}", err), "Parse error at line 42: Invalid syntax");
    }

    #[test]
    fn test_parse_error_without_line_number() {
        let err = GedcomError::ParseError {
            line_number: None,
            message: "Invalid syntax".to_string(),
        };
        assert_eq!(format!("{}", err), "Parse error: Invalid syntax");
    }

    #[test]
    fn test_invalid_structure_display() {
        let err = GedcomError::InvalidStructure("Missing required field".to_string());
        assert_eq!(
            format!("{}", err),
            "Invalid GEDCOM structure: Missing required field"
        );
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
            message: "test".to_string(),
        };
        assert!(err.source().is_none());

        let err = GedcomError::InvalidStructure("test".to_string());
        assert!(err.source().is_none());
    }

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
        let gedcom_err: GedcomError = io_err.into();
        assert!(matches!(gedcom_err, GedcomError::Io(_)));
    }
}
