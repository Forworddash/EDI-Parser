use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// EDI Implementation Guide Schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdiSchema {
    pub version: String,
    pub transaction_set: String,
    pub description: String,
    pub segments: Vec<SegmentDefinition>,
    pub loops: Vec<LoopDefinition>,
}

/// Definition of an EDI segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentDefinition {
    pub id: String,
    pub name: String,
    pub usage: SegmentUsage,
    pub max_repetitions: Option<u32>,
    pub elements: Vec<ElementDefinition>,
    pub position: u32,
    pub sequence: Option<u32>,
}

/// Definition of an EDI element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDefinition {
    pub position: u32,
    pub element_reference: String,
    pub name: String,
    pub usage: ElementUsage,
    pub data_type: DataType,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub valid_codes: Option<Vec<String>>,
    pub format: Option<String>,
}

/// Loop definition for hierarchical data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopDefinition {
    pub id: String,
    pub name: String,
    pub usage: SegmentUsage,
    pub max_repetitions: Option<u32>,
    pub segments: Vec<String>, // Segment IDs in this loop
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SegmentUsage {
    Required,    // M - Mandatory
    Optional,    // O - Optional
    Conditional, // C - Conditional
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ElementUsage {
    Required,    // M - Mandatory
    Optional,    // O - Optional
    Conditional, // C - Conditional
    NotUsed,     // N - Not Used
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    AN, // Alphanumeric
    N,  // Numeric
    R,  // Decimal
    ID, // Identifier
    DT, // Date
    TM, // Time
    B,  // Binary
}

impl EdiSchema {
    /// Load schema from JSON file
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
    
    /// Get segment definition by ID
    pub fn get_segment(&self, segment_id: &str) -> Option<&SegmentDefinition> {
        self.segments.iter().find(|s| s.id == segment_id)
    }
    
    /// Get all required segments
    pub fn required_segments(&self) -> Vec<&SegmentDefinition> {
        self.segments.iter()
            .filter(|s| s.usage == SegmentUsage::Required)
            .collect()
    }
}

impl SegmentDefinition {
    /// Get element definition by position
    pub fn get_element(&self, position: u32) -> Option<&ElementDefinition> {
        self.elements.iter().find(|e| e.position == position)
    }
    
    /// Get all required elements
    pub fn required_elements(&self) -> Vec<&ElementDefinition> {
        self.elements.iter()
            .filter(|e| e.usage == ElementUsage::Required)
            .collect()
    }
}

impl ElementDefinition {
    /// Validate element value against definition
    pub fn validate_value(&self, value: &str) -> Vec<String> {
        let mut errors = Vec::new();
        
        // Check length constraints
        if let Some(min_len) = self.min_length {
            if value.len() < min_len as usize {
                errors.push(format!("Element {} too short: {} chars, minimum {}", 
                    self.element_reference, value.len(), min_len));
            }
        }
        
        if let Some(max_len) = self.max_length {
            if value.len() > max_len as usize {
                errors.push(format!("Element {} too long: {} chars, maximum {}", 
                    self.element_reference, value.len(), max_len));
            }
        }
        
        // Check data type
        match self.data_type {
            DataType::N => {
                if !value.chars().all(|c| c.is_ascii_digit()) {
                    errors.push(format!("Element {} must be numeric", self.element_reference));
                }
            }
            DataType::R => {
                if value.parse::<f64>().is_err() {
                    errors.push(format!("Element {} must be a valid decimal", self.element_reference));
                }
            }
            DataType::DT => {
                // Validate date format (YYYYMMDD, YYMMDD, etc.)
                if let Some(format) = &self.format {
                    if !self.validate_date_format(value, format) {
                        errors.push(format!("Element {} invalid date format, expected {}", 
                            self.element_reference, format));
                    }
                }
            }
            _ => {} // AN, ID, TM, B validation can be added as needed
        }
        
        // Check valid codes
        if let Some(valid_codes) = &self.valid_codes {
            if !valid_codes.contains(&value.to_string()) {
                errors.push(format!("Element {} invalid code '{}', valid codes: {:?}", 
                    self.element_reference, value, valid_codes));
            }
        }
        
        errors
    }
    
    fn validate_date_format(&self, value: &str, format: &str) -> bool {
        match format {
            "YYYYMMDD" => value.len() == 8 && value.chars().all(|c| c.is_ascii_digit()),
            "YYMMDD" => value.len() == 6 && value.chars().all(|c| c.is_ascii_digit()),
            "CCYYMMDD" => value.len() == 8 && value.chars().all(|c| c.is_ascii_digit()),
            _ => true // Unknown format, assume valid
        }
    }
}
