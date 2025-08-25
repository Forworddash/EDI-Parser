use crate::{models::Segment, error::EdiError};
use super::SegmentParser;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// ISA - Interchange Control Header
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct IsaSegment {
    pub authorization_info_qualifier: String,
    pub authorization_information: String,
    pub security_info_qualifier: String,
    pub security_information: String,
    pub interchange_id_qualifier_sender: String,
    pub interchange_sender_id: String,
    pub interchange_id_qualifier_receiver: String,
    pub interchange_receiver_id: String,
    pub interchange_date: String,
    pub interchange_time: String,
    pub interchange_control_standards_id: String,
    pub interchange_control_version_number: String,
    pub interchange_control_number: String,
    pub acknowledgment_requested: String,
    pub usage_indicator: String,
    pub component_element_separator: String,
}

impl SegmentParser for IsaSegment {
    type Output = Self;
    
    fn parse_segment(segment: &Segment) -> Result<Self::Output, EdiError> {
        if segment.id != "ISA" {
            return Err(EdiError::InvalidSegmentFormat(format!("Expected ISA, got {}", segment.id)));
        }
        
        if segment.elements.len() < 16 {
            return Err(EdiError::InvalidSegmentFormat("ISA segment requires 16 elements".to_string()));
        }
        
        Ok(IsaSegment {
            authorization_info_qualifier: segment.elements[0].clone(),
            authorization_information: segment.elements[1].clone(),
            security_info_qualifier: segment.elements[2].clone(),
            security_information: segment.elements[3].clone(),
            interchange_id_qualifier_sender: segment.elements[4].clone(),
            interchange_sender_id: segment.elements[5].clone(),
            interchange_id_qualifier_receiver: segment.elements[6].clone(),
            interchange_receiver_id: segment.elements[7].clone(),
            interchange_date: segment.elements[8].clone(),
            interchange_time: segment.elements[9].clone(),
            interchange_control_standards_id: segment.elements[10].clone(),
            interchange_control_version_number: segment.elements[11].clone(),
            interchange_control_number: segment.elements[12].clone(),
            acknowledgment_requested: segment.elements[13].clone(),
            usage_indicator: segment.elements[14].clone(),
            component_element_separator: segment.elements[15].clone(),
        })
    }
    
    fn segment_id() -> &'static str {
        "ISA"
    }
    
    fn validate(&self) -> Result<(), EdiError> {
        if self.interchange_control_number.trim().is_empty() {
            return Err(EdiError::ValidationError("ISA control number cannot be empty".to_string()));
        }
        Ok(())
    }
}

/// GS - Functional Group Header
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct GsSegment {
    pub functional_identifier_code: String,
    pub application_sender_code: String,
    pub application_receiver_code: String,
    pub date: String,
    pub time: String,
    pub group_control_number: String,
    pub responsible_agency_code: String,
    pub version_release_id: String,
}

impl SegmentParser for GsSegment {
    type Output = Self;
    
    fn parse_segment(segment: &Segment) -> Result<Self::Output, EdiError> {
        if segment.id != "GS" {
            return Err(EdiError::InvalidSegmentFormat(format!("Expected GS, got {}", segment.id)));
        }
        
        if segment.elements.len() < 8 {
            return Err(EdiError::InvalidSegmentFormat("GS segment requires 8 elements".to_string()));
        }
        
        Ok(GsSegment {
            functional_identifier_code: segment.elements[0].clone(),
            application_sender_code: segment.elements[1].clone(),
            application_receiver_code: segment.elements[2].clone(),
            date: segment.elements[3].clone(),
            time: segment.elements[4].clone(),
            group_control_number: segment.elements[5].clone(),
            responsible_agency_code: segment.elements[6].clone(),
            version_release_id: segment.elements[7].clone(),
        })
    }
    
    fn segment_id() -> &'static str {
        "GS"
    }
    
    fn validate(&self) -> Result<(), EdiError> {
        if self.group_control_number.trim().is_empty() {
            return Err(EdiError::ValidationError("GS control number cannot be empty".to_string()));
        }
        Ok(())
    }
}

/// ST - Transaction Set Header
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct StSegment {
    pub transaction_set_identifier_code: String,
    pub transaction_set_control_number: String,
    pub implementation_convention_reference: Option<String>,
}

impl SegmentParser for StSegment {
    type Output = Self;
    
    fn parse_segment(segment: &Segment) -> Result<Self::Output, EdiError> {
        if segment.id != "ST" {
            return Err(EdiError::InvalidSegmentFormat(format!("Expected ST, got {}", segment.id)));
        }
        
        if segment.elements.len() < 2 {
            return Err(EdiError::InvalidSegmentFormat("ST segment requires at least 2 elements".to_string()));
        }
        
        Ok(StSegment {
            transaction_set_identifier_code: segment.elements[0].clone(),
            transaction_set_control_number: segment.elements[1].clone(),
            implementation_convention_reference: segment.elements.get(2).cloned(),
        })
    }
    
    fn segment_id() -> &'static str {
        "ST"
    }
    
    fn validate(&self) -> Result<(), EdiError> {
        if self.transaction_set_control_number.trim().is_empty() {
            return Err(EdiError::ValidationError("ST control number cannot be empty".to_string()));
        }
        Ok(())
    }
}
