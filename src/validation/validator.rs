use super::{ValidationResult, ValidationError, ValidationWarning, ValidationErrorType};
use super::schema::*;
use crate::models::{Transaction, Segment};
use std::collections::HashMap;

/// Validates EDI transactions against schema definitions
pub struct EdiValidator {
    schema: EdiSchema,
}

impl EdiValidator {
    /// Create new validator with schema
    pub fn new(schema: EdiSchema) -> Self {
        Self { schema }
    }
    
    /// Validate a complete transaction
    pub fn validate_transaction(&self, transaction: &Transaction) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Track segment occurrences for repetition validation
        let mut segment_counts: HashMap<String, u32> = HashMap::new();
        let mut segment_positions: HashMap<String, Vec<usize>> = HashMap::new();
        
        // Validate each segment
        for (index, segment) in transaction.segments.iter().enumerate() {
            // Count segment occurrences
            *segment_counts.entry(segment.id.clone()).or_insert(0) += 1;
            segment_positions.entry(segment.id.clone()).or_default().push(index);
            
            // Validate individual segment
            let segment_result = self.validate_segment(segment);
            result.merge(segment_result);
        }
        
        // Validate segment repetitions
        for (segment_id, count) in segment_counts.iter() {
            if let Some(def) = self.schema.get_segment(segment_id) {
                if let Some(max_reps) = def.max_repetitions {
                    if *count > max_reps {
                        result.add_error(ValidationError {
                            segment_id: segment_id.clone(),
                            element_position: None,
                            error_type: ValidationErrorType::TooManyRepetitions,
                            message: format!("Segment {} appears {} times, maximum allowed {}", 
                                segment_id, count, max_reps),
                        });
                    }
                }
            }
        }
        
        // Validate required segments are present
        for required_seg in self.schema.required_segments() {
            if !segment_counts.contains_key(&required_seg.id) {
                result.add_error(ValidationError {
                    segment_id: required_seg.id.clone(),
                    element_position: None,
                    error_type: ValidationErrorType::MissingRequiredSegment,
                    message: format!("Required segment {} is missing", required_seg.id),
                });
            }
        }
        
        // Validate segment sequence (if specified)
        result.merge(self.validate_segment_sequence(&transaction.segments));
        
        result
    }
    
    /// Validate a single segment
    pub fn validate_segment(&self, segment: &Segment) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        let segment_def = match self.schema.get_segment(&segment.id) {
            Some(def) => def,
            None => {
                result.add_warning(ValidationWarning {
                    segment_id: segment.id.clone(),
                    element_position: None,
                    message: format!("Unknown segment {} not in schema", segment.id),
                });
                return result;
            }
        };
        
        // Validate required elements are present
        for required_elem in segment_def.required_elements() {
            let element_index = (required_elem.position - 1) as usize; // Convert to 0-based index
            
            if element_index >= segment.elements.len() || segment.elements[element_index].trim().is_empty() {
                result.add_error(ValidationError {
                    segment_id: segment.id.clone(),
                    element_position: Some(required_elem.position as usize),
                    error_type: ValidationErrorType::MissingRequiredElement,
                    message: format!("Required element at position {} is missing or empty", 
                        required_elem.position),
                });
                continue;
            }
        }
        
        // Validate each present element
        for (index, element_value) in segment.elements.iter().enumerate() {
            let position = (index + 1) as u32; // Convert to 1-based position
            
            if let Some(element_def) = segment_def.get_element(position) {
                if !element_value.trim().is_empty() {
                    let validation_errors = element_def.validate_value(element_value);
                    for error_msg in validation_errors {
                        result.add_error(ValidationError {
                            segment_id: segment.id.clone(),
                            element_position: Some(position as usize),
                            error_type: ValidationErrorType::InvalidElementFormat,
                            message: error_msg,
                        });
                    }
                }
            } else if !element_value.trim().is_empty() {
                result.add_warning(ValidationWarning {
                    segment_id: segment.id.clone(),
                    element_position: Some(position as usize),
                    message: format!("Element at position {} not defined in schema", position),
                });
            }
        }
        
        result
    }
    
    /// Validate segment sequence according to schema
    fn validate_segment_sequence(&self, segments: &[Segment]) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Create a map of expected sequences
        let mut sequence_map: HashMap<String, u32> = HashMap::new();
        for seg_def in &self.schema.segments {
            if let Some(seq) = seg_def.sequence {
                sequence_map.insert(seg_def.id.clone(), seq);
            }
        }
        
        // Check if segments appear in correct sequence
        let mut last_sequence = 0;
        for segment in segments {
            if let Some(&expected_seq) = sequence_map.get(&segment.id) {
                if expected_seq < last_sequence {
                    result.add_error(ValidationError {
                        segment_id: segment.id.clone(),
                        element_position: None,
                        error_type: ValidationErrorType::SegmentOutOfSequence,
                        message: format!("Segment {} appears out of sequence", segment.id),
                    });
                } else {
                    last_sequence = expected_seq;
                }
            }
        }
        
        result
    }
}

/// Builder for creating validation schemas programmatically
pub struct SchemaBuilder {
    version: String,
    transaction_set: String,
    description: String,
    segments: Vec<SegmentDefinition>,
    loops: Vec<LoopDefinition>,
}

impl SchemaBuilder {
    pub fn new(version: &str, transaction_set: &str, description: &str) -> Self {
        Self {
            version: version.to_string(),
            transaction_set: transaction_set.to_string(),
            description: description.to_string(),
            segments: Vec::new(),
            loops: Vec::new(),
        }
    }
    
    pub fn add_segment(mut self, segment: SegmentDefinition) -> Self {
        self.segments.push(segment);
        self
    }
    
    pub fn add_loop(mut self, loop_def: LoopDefinition) -> Self {
        self.loops.push(loop_def);
        self
    }
    
    pub fn build(self) -> EdiSchema {
        EdiSchema {
            version: self.version,
            transaction_set: self.transaction_set,
            description: self.description,
            segments: self.segments,
            loops: self.loops,
        }
    }
}

/// Builder for creating segment definitions
pub struct SegmentBuilder {
    id: String,
    name: String,
    usage: SegmentUsage,
    max_repetitions: Option<u32>,
    elements: Vec<ElementDefinition>,
    position: u32,
    sequence: Option<u32>,
}

impl SegmentBuilder {
    pub fn new(id: &str, name: &str, usage: SegmentUsage, position: u32) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            usage,
            max_repetitions: None,
            elements: Vec::new(),
            position,
            sequence: None,
        }
    }
    
    pub fn max_repetitions(mut self, max: u32) -> Self {
        self.max_repetitions = Some(max);
        self
    }
    
    pub fn sequence(mut self, seq: u32) -> Self {
        self.sequence = Some(seq);
        self
    }
    
    pub fn add_element(mut self, element: ElementDefinition) -> Self {
        self.elements.push(element);
        self
    }
    
    pub fn build(self) -> SegmentDefinition {
        SegmentDefinition {
            id: self.id,
            name: self.name,
            usage: self.usage,
            max_repetitions: self.max_repetitions,
            elements: self.elements,
            position: self.position,
            sequence: self.sequence,
        }
    }
}

/// Builder for creating element definitions
pub struct ElementBuilder {
    position: u32,
    element_reference: String,
    name: String,
    usage: ElementUsage,
    data_type: DataType,
    min_length: Option<u32>,
    max_length: Option<u32>,
    valid_codes: Option<Vec<String>>,
    format: Option<String>,
}

impl ElementBuilder {
    pub fn new(position: u32, element_ref: &str, name: &str, usage: ElementUsage, data_type: DataType) -> Self {
        Self {
            position,
            element_reference: element_ref.to_string(),
            name: name.to_string(),
            usage,
            data_type,
            min_length: None,
            max_length: None,
            valid_codes: None,
            format: None,
        }
    }
    
    pub fn length(mut self, min: u32, max: u32) -> Self {
        self.min_length = Some(min);
        self.max_length = Some(max);
        self
    }
    
    pub fn max_length(mut self, max: u32) -> Self {
        self.max_length = Some(max);
        self
    }
    
    pub fn valid_codes(mut self, codes: Vec<String>) -> Self {
        self.valid_codes = Some(codes);
        self
    }
    
    pub fn format(mut self, format: &str) -> Self {
        self.format = Some(format.to_string());
        self
    }
    
    pub fn build(self) -> ElementDefinition {
        ElementDefinition {
            position: self.position,
            element_reference: self.element_reference,
            name: self.name,
            usage: self.usage,
            data_type: self.data_type,
            min_length: self.min_length,
            max_length: self.max_length,
            valid_codes: self.valid_codes,
            format: self.format,
        }
    }
}
