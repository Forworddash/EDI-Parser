use crate::{models::Segment, error::EdiError};
use super::SegmentParser;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// N1 - Name
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct N1Segment {
    pub entity_identifier_code: String,
    pub name: Option<String>,
    pub identification_code_qualifier: Option<String>,
    pub identification_code: Option<String>,
    pub entity_relationship_code: Option<String>,
    pub entity_identifier_code_2: Option<String>,
}

impl SegmentParser for N1Segment {
    type Output = Self;
    
    fn parse_segment(segment: &Segment) -> Result<Self::Output, EdiError> {
        if segment.id != "N1" {
            return Err(EdiError::InvalidSegmentFormat(format!("Expected N1, got {}", segment.id)));
        }
        
        if segment.elements.is_empty() {
            return Err(EdiError::InvalidSegmentFormat("N1 segment requires at least 1 element".to_string()));
        }
        
        Ok(N1Segment {
            entity_identifier_code: segment.elements[0].clone(),
            name: segment.elements.get(1).cloned(),
            identification_code_qualifier: segment.elements.get(2).cloned(),
            identification_code: segment.elements.get(3).cloned(),
            entity_relationship_code: segment.elements.get(4).cloned(),
            entity_identifier_code_2: segment.elements.get(5).cloned(),
        })
    }
    
    fn segment_id() -> &'static str {
        "N1"
    }
    
    fn validate(&self) -> Result<(), EdiError> {
        if self.entity_identifier_code.trim().is_empty() {
            return Err(EdiError::ValidationError("Entity identifier code cannot be empty".to_string()));
        }
        Ok(())
    }
}

/// REF - Reference Identification
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct RefSegment {
    pub reference_identification_qualifier: String,
    pub reference_identification: Option<String>,
    pub description: Option<String>,
    pub reference_identifier: Option<String>,
}

impl SegmentParser for RefSegment {
    type Output = Self;
    
    fn parse_segment(segment: &Segment) -> Result<Self::Output, EdiError> {
        if segment.id != "REF" {
            return Err(EdiError::InvalidSegmentFormat(format!("Expected REF, got {}", segment.id)));
        }
        
        if segment.elements.is_empty() {
            return Err(EdiError::InvalidSegmentFormat("REF segment requires at least 1 element".to_string()));
        }
        
        Ok(RefSegment {
            reference_identification_qualifier: segment.elements[0].clone(),
            reference_identification: segment.elements.get(1).cloned(),
            description: segment.elements.get(2).cloned(),
            reference_identifier: segment.elements.get(3).cloned(),
        })
    }
    
    fn segment_id() -> &'static str {
        "REF"
    }
    
    fn validate(&self) -> Result<(), EdiError> {
        Ok(())
    }
}

/// DTM - Date/Time Reference
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct DtmSegment {
    pub date_time_qualifier: String,
    pub date: Option<String>,
    pub time: Option<String>,
    pub time_code: Option<String>,
    pub date_time_period_format_qualifier: Option<String>,
    pub date_time_period: Option<String>,
}

impl SegmentParser for DtmSegment {
    type Output = Self;
    
    fn parse_segment(segment: &Segment) -> Result<Self::Output, EdiError> {
        if segment.id != "DTM" {
            return Err(EdiError::InvalidSegmentFormat(format!("Expected DTM, got {}", segment.id)));
        }
        
        if segment.elements.is_empty() {
            return Err(EdiError::InvalidSegmentFormat("DTM segment requires at least 1 element".to_string()));
        }
        
        Ok(DtmSegment {
            date_time_qualifier: segment.elements[0].clone(),
            date: segment.elements.get(1).cloned(),
            time: segment.elements.get(2).cloned(),
            time_code: segment.elements.get(3).cloned(),
            date_time_period_format_qualifier: segment.elements.get(4).cloned(),
            date_time_period: segment.elements.get(5).cloned(),
        })
    }
    
    fn segment_id() -> &'static str {
        "DTM"
    }
    
    fn validate(&self) -> Result<(), EdiError> {
        // Could add date format validation here
        Ok(())
    }
}
