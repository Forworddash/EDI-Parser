use crate::models::segment_definition::{ElementDefinition, ElementRequirement, ElementDataType};
use crate::error::EdiError;

/// Represents an EDI element with its value and metadata for validation
#[derive(Debug, Clone)]
pub struct ValidatedElement {
    /// The element definition with ID and rules
    pub definition: ElementDefinition,
    /// The actual value from the EDI segment
    pub value: Option<String>,
    /// Position in the segment (1-based)
    pub position: usize,
}

impl ValidatedElement {
    /// Create a new validated element
    pub fn new(definition: ElementDefinition, value: Option<String>, position: usize) -> Self {
        Self {
            definition,
            value,
            position,
        }
    }

    /// Validate this element according to its definition
    pub fn validate(&self) -> Result<(), EdiError> {
        // Check if required element is present
        if self.definition.requirement == ElementRequirement::Mandatory && self.value.is_none() {
            return Err(EdiError::ValidationError(format!(
                "Required element {} (ID: {}) at position {} is missing",
                self.definition.name, self.definition.element_id, self.position
            )));
        }

        // If no value, and it's optional, that's fine
        if self.value.is_none() {
            return Ok(());
        }

        let value = self.value.as_ref().unwrap();

        // For optional elements, empty values are allowed
        if value.is_empty() && self.definition.requirement != ElementRequirement::Mandatory {
            return Ok(());
        }

        // Validate length (only for non-empty values or mandatory elements)
        if let Some(min_len) = self.definition.min_length {
            if value.len() < min_len as usize {
                return Err(EdiError::ValidationError(format!(
                    "Element {} (ID: {}) at position {}: '{}' is too short (min: {})",
                    self.definition.name, self.definition.element_id, self.position, value, min_len
                )));
            }
        }

        if let Some(max_len) = self.definition.max_length {
            if value.len() > max_len as usize {
                return Err(EdiError::ValidationError(format!(
                    "Element {} (ID: {}) at position {}: '{}' is too long (max: {})",
                    self.definition.name, self.definition.element_id, self.position, value, max_len
                )));
            }
        }

        // Validate data type
        self.validate_data_type(value)?;

        // Validate against valid codes if applicable
        if let Some(valid_codes) = &self.definition.valid_codes {
            if !valid_codes.contains(value) {
                return Err(EdiError::ValidationError(format!(
                    "Element {} (ID: {}) at position {}: '{}' is not a valid code. Valid codes: {:?}",
                    self.definition.name, self.definition.element_id, self.position, value, valid_codes
                )));
            }
        }

        Ok(())
    }

    /// Validate the data type of the element value
    fn validate_data_type(&self, value: &str) -> Result<(), EdiError> {
        match self.definition.data_type {
            ElementDataType::N => {
                // Numeric - only digits
                if !value.chars().all(|c| c.is_ascii_digit()) {
                    return Err(EdiError::ValidationError(format!(
                        "Element {} (ID: {}) at position {}: '{}' must be numeric",
                        self.definition.name, self.definition.element_id, self.position, value
                    )));
                }
            }
            ElementDataType::R => {
                // Decimal - digits and optional decimal point
                if value.parse::<f64>().is_err() {
                    return Err(EdiError::ValidationError(format!(
                        "Element {} (ID: {}) at position {}: '{}' must be a valid decimal number",
                        self.definition.name, self.definition.element_id, self.position, value
                    )));
                }
            }
            ElementDataType::DT => {
                // Date - validate format (CCYYMMDD)
                if value.len() == 8 && value.chars().all(|c| c.is_ascii_digit()) {
                    // Basic format check - could be enhanced with actual date validation
                    let _year: i32 = value[0..4].parse().unwrap();
                    let month: u32 = value[4..6].parse().unwrap();
                    let day: u32 = value[6..8].parse().unwrap();
                    
                    if month < 1 || month > 12 {
                        return Err(EdiError::ValidationError(format!(
                            "Element {} (ID: {}) at position {}: '{}' has invalid month",
                            self.definition.name, self.definition.element_id, self.position, value
                        )));
                    }
                    
                    if day < 1 || day > 31 {
                        return Err(EdiError::ValidationError(format!(
                            "Element {} (ID: {}) at position {}: '{}' has invalid day",
                            self.definition.name, self.definition.element_id, self.position, value
                        )));
                    }
                } else {
                    return Err(EdiError::ValidationError(format!(
                        "Element {} (ID: {}) at position {}: '{}' must be a valid date (CCYYMMDD)",
                        self.definition.name, self.definition.element_id, self.position, value
                    )));
                }
            }
            ElementDataType::TM => {
                // Time - validate format (HHMM or HHMMSS)
                if (value.len() == 4 || value.len() == 6) && value.chars().all(|c| c.is_ascii_digit()) {
                    let hour: u32 = value[0..2].parse().unwrap();
                    let minute: u32 = value[2..4].parse().unwrap();
                    
                    if hour > 23 {
                        return Err(EdiError::ValidationError(format!(
                            "Element {} (ID: {}) at position {}: '{}' has invalid hour",
                            self.definition.name, self.definition.element_id, self.position, value
                        )));
                    }
                    
                    if minute > 59 {
                        return Err(EdiError::ValidationError(format!(
                            "Element {} (ID: {}) at position {}: '{}' has invalid minute",
                            self.definition.name, self.definition.element_id, self.position, value
                        )));
                    }

                    if value.len() == 6 {
                        let second: u32 = value[4..6].parse().unwrap();
                        if second > 59 {
                            return Err(EdiError::ValidationError(format!(
                                "Element {} (ID: {}) at position {}: '{}' has invalid second",
                                self.definition.name, self.definition.element_id, self.position, value
                            )));
                        }
                    }
                } else {
                    return Err(EdiError::ValidationError(format!(
                        "Element {} (ID: {}) at position {}: '{}' must be a valid time (HHMM or HHMMSS)",
                        self.definition.name, self.definition.element_id, self.position, value
                    )));
                }
            }
            ElementDataType::AN | ElementDataType::ID => {
                // Alphanumeric and ID types - any characters allowed
                // Additional validation for ID types is handled by valid_codes check above
            }
            ElementDataType::B => {
                // Binary data - typically base64 encoded
                // For now, just ensure it's not empty if required
                if value.is_empty() && self.definition.requirement == ElementRequirement::Mandatory {
                    return Err(EdiError::ValidationError(format!(
                        "Element {} (ID: {}) at position {}: Binary data cannot be empty",
                        self.definition.name, self.definition.element_id, self.position
                    )));
                }
            }
        }

        Ok(())
    }

    /// Get the element ID
    pub fn element_id(&self) -> u32 {
        self.definition.element_id
    }

    /// Get the element name
    pub fn element_name(&self) -> &str {
        &self.definition.name
    }

    /// Get the element value
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Check if this element is required
    pub fn is_required(&self) -> bool {
        self.definition.requirement == ElementRequirement::Mandatory
    }
}

/// Represents a validated EDI segment with element metadata
#[derive(Debug, Clone)]
pub struct ValidatedSegment {
    /// Segment ID (e.g., "BEG", "CUR")
    pub segment_id: String,
    /// All elements with their definitions and values
    pub elements: Vec<ValidatedElement>,
}

impl ValidatedSegment {
    /// Create a new validated segment
    pub fn new(segment_id: String) -> Self {
        Self {
            segment_id,
            elements: Vec::new(),
        }
    }

    /// Add an element to this segment
    pub fn add_element(&mut self, element: ValidatedElement) {
        self.elements.push(element);
    }

    /// Validate all elements in this segment
    pub fn validate(&self) -> Result<(), EdiError> {
        for element in &self.elements {
            element.validate()?;
        }
        Ok(())
    }

    /// Get element by ID
    pub fn get_element_by_id(&self, element_id: u32) -> Option<&ValidatedElement> {
        self.elements.iter().find(|e| e.element_id() == element_id)
    }

    /// Get element by position (1-based)
    pub fn get_element_by_position(&self, position: usize) -> Option<&ValidatedElement> {
        self.elements.iter().find(|e| e.position == position)
    }

    /// Get all required elements that are missing
    pub fn get_missing_required_elements(&self) -> Vec<&ValidatedElement> {
        self.elements
            .iter()
            .filter(|e| e.is_required() && e.value().is_none())
            .collect()
    }
}
