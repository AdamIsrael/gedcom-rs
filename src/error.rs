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
