use crate::{
    error::EdiError,
    models::{
        Segment,
        segment_definition::{SegmentRegistry, SegmentDefinition, ElementDefinition, ElementDataType, X12Version}
    }
};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DATE_REGEX: Regex = Regex::new(r"^\d{8}$").unwrap();
    static ref TIME_REGEX: Regex = Regex::new(r"^\d{4}(\d{2})?(\d{2})?$").unwrap();
    static ref NUMERIC_REGEX: Regex = Regex::new(r"^\d+$").unwrap();
    static ref DECIMAL_REGEX: Regex = Regex::new(r"^\d+(\.\d+)?$").unwrap();
}

/// Validation result for a segment
#[derive(Debug)]
pub struct SegmentValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

/// Specific validation error
#[derive(Debug)]
pub struct ValidationError {
    pub element_position: Option<usize>,
    pub error_type: ValidationErrorType,
    pub message: String,
}

/// Specific validation warning
#[derive(Debug)]
pub struct ValidationWarning {
    pub element_position: Option<usize>,
    pub message: String,
}

/// Types of validation errors
#[derive(Debug)]
pub enum ValidationErrorType {
    MissingRequiredElement,
    InvalidLength,
    InvalidDataType,
    InvalidFormat,
    TooManyElements,
    UnknownSegment,
}

/// Validator for EDI segments
pub struct SegmentValidator {
    registry: SegmentRegistry,
    version: X12Version,
}

impl SegmentValidator {
    pub fn new(version: X12Version) -> Self {
        Self {
            registry: SegmentRegistry::new(),
            version,
        }
    }

    pub fn with_registry(registry: SegmentRegistry, version: X12Version) -> Self {
        Self {
            registry,
            version,
        }
    }

    /// Validate a segment against its definition
    pub fn validate_segment(&self, segment: &Segment) -> SegmentValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Get segment definition
        let definition = match self.registry.get_definition(&self.version, &segment.id) {
            Some(def) => def,
            None => {
                errors.push(ValidationError {
                    element_position: None,
                    error_type: ValidationErrorType::UnknownSegment,
                    message: format!("Unknown segment type: {}", segment.id),
                });
                return SegmentValidationResult {
                    is_valid: false,
                    errors,
                    warnings,
                };
            }
        };

        // Check if we have too many elements
        if segment.elements.len() > definition.elements.len() {
            errors.push(ValidationError {
                element_position: Some(segment.elements.len()),
                error_type: ValidationErrorType::TooManyElements,
                message: format!(
                    "Segment {} has {} elements but definition only allows {}",
                    segment.id,
                    segment.elements.len(),
                    definition.elements.len()
                ),
            });
        }

        // Validate each element
        for (index, element_def) in definition.elements.iter().enumerate() {
            let element_value = segment.elements.get(index);

            // Check required elements
            if element_def.required && (element_value.is_none() || element_value.unwrap().is_empty()) {
                errors.push(ValidationError {
                    element_position: Some(index + 1),
                    error_type: ValidationErrorType::MissingRequiredElement,
                    message: format!(
                        "Required element '{}' is missing or empty at position {}",
                        element_def.name,
                        index + 1
                    ),
                });
                continue;
            }

            // Skip validation for optional empty elements
            if let Some(value) = element_value {
                if !value.is_empty() {
                    self.validate_element(value, element_def, index + 1, &mut errors, &mut warnings);
                }
            }
        }

        SegmentValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    /// Validate an individual element
    fn validate_element(
        &self,
        value: &str,
        definition: &ElementDefinition,
        position: usize,
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
    ) {
        // Check length constraints
        if let Some(min_length) = definition.min_length {
            if value.len() < min_length {
                errors.push(ValidationError {
                    element_position: Some(position),
                    error_type: ValidationErrorType::InvalidLength,
                    message: format!(
                        "Element '{}' at position {} is too short. Expected min {} characters, got {}",
                        definition.name,
                        position,
                        min_length,
                        value.len()
                    ),
                });
            }
        }

        if let Some(max_length) = definition.max_length {
            if value.len() > max_length {
                errors.push(ValidationError {
                    element_position: Some(position),
                    error_type: ValidationErrorType::InvalidLength,
                    message: format!(
                        "Element '{}' at position {} is too long. Expected max {} characters, got {}",
                        definition.name,
                        position,
                        max_length,
                        value.len()
                    ),
                });
            }
        }

        // Check data type constraints
        match definition.data_type {
            ElementDataType::N => {
                if !NUMERIC_REGEX.is_match(value) {
                    errors.push(ValidationError {
                        element_position: Some(position),
                        error_type: ValidationErrorType::InvalidDataType,
                        message: format!(
                            "Element '{}' at position {} must be numeric, got '{}'",
                            definition.name,
                            position,
                            value
                        ),
                    });
                }
            }
            ElementDataType::R => {
                if !DECIMAL_REGEX.is_match(value) {
                    errors.push(ValidationError {
                        element_position: Some(position),
                        error_type: ValidationErrorType::InvalidDataType,
                        message: format!(
                            "Element '{}' at position {} must be a decimal number, got '{}'",
                            definition.name,
                            position,
                            value
                        ),
                    });
                }
            }
            ElementDataType::DT => {
                if !DATE_REGEX.is_match(value) {
                    errors.push(ValidationError {
                        element_position: Some(position),
                        error_type: ValidationErrorType::InvalidFormat,
                        message: format!(
                            "Element '{}' at position {} must be a valid date (CCYYMMDD), got '{}'",
                            definition.name,
                            position,
                            value
                        ),
                    });
                }
            }
            ElementDataType::TM => {
                if !TIME_REGEX.is_match(value) {
                    errors.push(ValidationError {
                        element_position: Some(position),
                        error_type: ValidationErrorType::InvalidFormat,
                        message: format!(
                            "Element '{}' at position {} must be a valid time (HHMM, HHMMSS, or HHMMSSDD), got '{}'",
                            definition.name,
                            position,
                            value
                        ),
                    });
                }
            }
            ElementDataType::AN | ElementDataType::ID => {
                // For alphanumeric and ID types, just check printable ASCII
                if !value.chars().all(|c| c.is_ascii() && !c.is_control()) {
                    warnings.push(ValidationWarning {
                        element_position: Some(position),
                        message: format!(
                            "Element '{}' at position {} contains non-printable characters",
                            definition.name,
                            position
                        ),
                    });
                }
            }
        }
    }

    /// Validate that segment usage counts are within limits for a transaction
    pub fn validate_segment_usage(
        &self,
        segments: &[Segment],
        segment_id: &str,
    ) -> Result<(), EdiError> {
        let definition = self.registry.get_definition(&self.version, segment_id)
            .ok_or_else(|| EdiError::InvalidSegmentFormat(format!("Unknown segment: {}", segment_id)))?;

        let count = segments.iter().filter(|s| s.id == segment_id).count() as u32;

        if count < definition.min_usage {
            return Err(EdiError::InvalidControlStructure);
        }

        if let Some(max_usage) = definition.max_usage {
            if count > max_usage {
                return Err(EdiError::InvalidControlStructure);
            }
        }

        Ok(())
    }

    /// Get segment definition for introspection
    pub fn get_segment_definition(&self, segment_id: &str) -> Option<&SegmentDefinition> {
        self.registry.get_definition(&self.version, segment_id)
    }

    /// Get all registered segment IDs for the current version
    pub fn get_registered_segments(&self) -> Vec<String> {
        self.registry.get_registered_segments_for_version(&self.version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beg_segment_validation() {
        let validator = SegmentValidator::new(X12Version::V4010);
        
        // Valid BEG segment
        let valid_segment = Segment::new(
            "BEG".to_string(),
            vec![
                "00".to_string(),      // Transaction Set Purpose Code
                "SA".to_string(),      // Purchase Order Type Code
                "PO-001".to_string(),  // Purchase Order Number
                "".to_string(),        // Release Number (optional)
                "20230101".to_string(), // Date
            ],
        );

        let result = validator.validate_segment(&valid_segment);
        assert!(result.is_valid, "Valid BEG segment should pass validation");

        // Invalid BEG segment - missing required date
        let invalid_segment = Segment::new(
            "BEG".to_string(),
            vec![
                "00".to_string(),      // Transaction Set Purpose Code
                "SA".to_string(),      // Purchase Order Type Code
                "PO-001".to_string(),  // Purchase Order Number
            ],
        );

        let result = validator.validate_segment(&invalid_segment);
        assert!(!result.is_valid, "Invalid BEG segment should fail validation");
        assert!(!result.errors.is_empty(), "Should have validation errors");
    }

    #[test]
    fn test_element_length_validation() {
        let validator = SegmentValidator::new(X12Version::V4010);
        
        // BEG segment with purchase order number too long
        let segment = Segment::new(
            "BEG".to_string(),
            vec![
                "00".to_string(),
                "SA".to_string(),
                "A".repeat(25), // Too long - max 22 characters
                "".to_string(),
                "20230101".to_string(),
            ],
        );

        let result = validator.validate_segment(&segment);
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| matches!(e.error_type, ValidationErrorType::InvalidLength)));
    }

    #[test]
    fn test_date_format_validation() {
        let validator = SegmentValidator::new(X12Version::V4010);
        
        // BEG segment with invalid date format
        let segment = Segment::new(
            "BEG".to_string(),
            vec![
                "00".to_string(),
                "SA".to_string(),
                "PO-001".to_string(),
                "".to_string(),
                "2023-01-01".to_string(), // Invalid format - should be CCYYMMDD
            ],
        );

        let result = validator.validate_segment(&segment);
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| matches!(e.error_type, ValidationErrorType::InvalidFormat)));
    }
}
