pub mod rules;
pub mod schema;
pub mod validator;

use crate::error::EdiError;

// Re-export main types
pub use rules::*;
pub use schema::*;
pub use validator::*;

/// Validation result with detailed information
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub segment_id: String,
    pub element_position: Option<usize>,
    pub error_type: ValidationErrorType,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ValidationWarning {
    pub segment_id: String,
    pub element_position: Option<usize>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationErrorType {
    MissingRequiredSegment,
    MissingRequiredElement,
    ElementTooLong,
    ElementTooShort,
    InvalidElementFormat,
    TooManyRepetitions,
    InvalidDataType,
    InvalidCodeValue,
    SegmentOutOfSequence,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, error: ValidationError) {
        self.is_valid = false;
        self.errors.push(error);
    }
    
    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }
    
    pub fn merge(&mut self, other: ValidationResult) {
        if !other.is_valid {
            self.is_valid = false;
        }
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}
