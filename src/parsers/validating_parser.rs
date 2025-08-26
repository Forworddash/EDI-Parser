use crate::{
    models::{
        segment::Segment, 
        validated_element::{ValidatedSegment, ValidatedElement}, 
        segment_definition::{SegmentRegistry, X12Version, ElementDefinition}
    },
    error::EdiError,
};

/// Enhanced segment parser that validates against element definitions
pub struct ValidatingSegmentParser {
    registry: SegmentRegistry,
    version: X12Version,
}

impl ValidatingSegmentParser {
    /// Create a new validating parser for a specific X12 version
    pub fn new(version: X12Version) -> Self {
        Self {
            registry: SegmentRegistry::new(),
            version,
        }
    }

    /// Parse and validate a segment according to its definition
    pub fn parse_and_validate(&self, segment: &Segment) -> Result<ValidatedSegment, EdiError> {
        // Get the segment definition
        let definition = self.registry
            .get_definition(&self.version, &segment.id)
            .ok_or_else(|| EdiError::ValidationError(
                format!("Unknown segment '{}' for version {:?}", segment.id, self.version)
            ))?;

        let mut validated_segment = ValidatedSegment::new(segment.id.clone());

        // Process each element definition
        for (pos, element_def) in definition.elements.iter().enumerate() {
            let position = pos + 1; // 1-based positioning
            let value = segment.elements.get(pos).cloned();

            let validated_element = ValidatedElement::new(
                element_def.clone(),
                value,
                position,
            );

            validated_segment.add_element(validated_element);
        }

        // Validate the entire segment
        validated_segment.validate()?;

        Ok(validated_segment)
    }

    /// Get all available segment definitions for the current version
    pub fn get_available_segments(&self) -> Vec<String> {
        self.registry.get_registered_segments_for_version(&self.version)
    }

    /// Get element definition by ID across all segments
    pub fn get_element_definition_by_id(&self, element_id: u32) -> Vec<(String, &ElementDefinition)> {
        let segments = self.get_available_segments();
        let mut results = Vec::new();

        for segment_id in segments {
            if let Some(definition) = self.registry.get_definition(&self.version, &segment_id) {
                for element_def in &definition.elements {
                    if element_def.element_id == element_id {
                        results.push((segment_id.clone(), element_def));
                    }
                }
            }
        }

        results
    }

    /// Validate a specific element value against its definition
    pub fn validate_element_value(
        &self,
        segment_id: &str,
        position: usize,
        value: &str,
    ) -> Result<(), EdiError> {
        let definition = self.registry
            .get_definition(&self.version, segment_id)
            .ok_or_else(|| EdiError::ValidationError(
                format!("Unknown segment '{}' for version {:?}", segment_id, self.version)
            ))?;

        let element_def = definition.elements.get(position - 1) // Convert to 0-based
            .ok_or_else(|| EdiError::ValidationError(
                format!("Position {} does not exist in segment '{}'", position, segment_id)
            ))?;

        let validated_element = ValidatedElement::new(
            element_def.clone(),
            Some(value.to_string()),
            position,
        );

        validated_element.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Segment;

    #[test]
    fn test_validate_beg_segment() {
        let parser = ValidatingSegmentParser::new(X12Version::V4010);
        
        // Valid BEG segment
        let segment = Segment::new(
            "BEG".to_string(),
            vec![
                "00".to_string(),     // Transaction Set Purpose Code (353)
                "SA".to_string(),     // Purchase Order Type Code (92)
                "PO12345".to_string(), // Purchase Order Number (324)
                "".to_string(),       // Release Number (328) - optional
                "20240101".to_string(), // Date (373)
            ],
        );

        let result = parser.parse_and_validate(&segment);
        if let Err(e) = &result {
            println!("Validation error: {:?}", e);
        }
        assert!(result.is_ok());

        let validated = result.unwrap();
        assert_eq!(validated.segment_id, "BEG");
        assert_eq!(validated.elements.len(), 5);

        // Check element IDs
        assert_eq!(validated.elements[0].element_id(), 353);
        assert_eq!(validated.elements[1].element_id(), 92);
        assert_eq!(validated.elements[2].element_id(), 324);
    }

    #[test]
    fn test_validate_beg_segment_invalid_purpose_code() {
        let parser = ValidatingSegmentParser::new(X12Version::V4010);
        
        // Invalid BEG segment - bad purpose code
        let segment = Segment::new(
            "BEG".to_string(),
            vec![
                "99".to_string(),     // Invalid Transaction Set Purpose Code
                "SA".to_string(),
                "PO12345".to_string(),
                "".to_string(),
                "20240101".to_string(),
            ],
        );

        let result = parser.parse_and_validate(&segment);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("not a valid code"));
    }

    #[test]
    fn test_validate_cur_segment() {
        let parser = ValidatingSegmentParser::new(X12Version::V4010);
        
        // Valid CUR segment
        let segment = Segment::new(
            "CUR".to_string(),
            vec![
                "BY".to_string(),     // Entity Identifier Code (98)
                "USD".to_string(),    // Currency Code (100)
                "1.0".to_string(),    // Exchange Rate (280)
            ],
        );

        let result = parser.parse_and_validate(&segment);
        assert!(result.is_ok());

        let validated = result.unwrap();
        
        // Test element access by ID
        let entity_element = validated.get_element_by_id(98).unwrap();
        assert_eq!(entity_element.value().unwrap(), "BY");
        
        let currency_element = validated.get_element_by_id(100).unwrap();
        assert_eq!(currency_element.value().unwrap(), "USD");
    }

    #[test]
    fn test_get_element_definition_by_id() {
        let parser = ValidatingSegmentParser::new(X12Version::V4010);
        
        // Element ID 98 should appear in multiple segments
        let definitions = parser.get_element_definition_by_id(98);
        assert!(!definitions.is_empty());
        
        // Should find it in CUR segment
        let cur_def = definitions.iter().find(|(seg, _)| *seg == "CUR");
        assert!(cur_def.is_some());
    }
}
