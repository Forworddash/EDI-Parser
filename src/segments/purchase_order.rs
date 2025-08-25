use crate::{models::Segment, error::EdiError};
use super::SegmentParser;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// BEG - Beginning Segment for Purchase Order
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct BegSegment {
    pub transaction_set_purpose_code: String,
    pub purchase_order_type_code: String,
    pub purchase_order_number: String,
    pub release_number: Option<String>,
    pub date: String,
    pub contract_number: Option<String>,
    pub acknowledgment_type: Option<String>,
    pub invoice_type_code: Option<String>,
    pub contract_type_code: Option<String>,
    pub purchase_category: Option<String>,
    pub security_level_code: Option<String>,
}

impl SegmentParser for BegSegment {
    type Output = Self;
    
    fn parse_segment(segment: &Segment) -> Result<Self::Output, EdiError> {
        if segment.id != "BEG" {
            return Err(EdiError::InvalidSegmentFormat(format!("Expected BEG, got {}", segment.id)));
        }
        
        if segment.elements.len() < 5 {
            return Err(EdiError::InvalidSegmentFormat("BEG segment requires at least 5 elements".to_string()));
        }
        
        Ok(BegSegment {
            transaction_set_purpose_code: segment.elements[0].clone(),
            purchase_order_type_code: segment.elements[1].clone(),
            purchase_order_number: segment.elements[2].clone(),
            release_number: segment.elements.get(3).cloned(),
            date: segment.elements[4].clone(),
            contract_number: segment.elements.get(5).cloned(),
            acknowledgment_type: segment.elements.get(6).cloned(),
            invoice_type_code: segment.elements.get(7).cloned(),
            contract_type_code: segment.elements.get(8).cloned(),
            purchase_category: segment.elements.get(9).cloned(),
            security_level_code: segment.elements.get(10).cloned(),
        })
    }
    
    fn segment_id() -> &'static str {
        "BEG"
    }
    
    fn validate(&self) -> Result<(), EdiError> {
        if self.purchase_order_number.trim().is_empty() {
            return Err(EdiError::ValidationError("Purchase order number cannot be empty".to_string()));
        }
        Ok(())
    }
}

/// PO1 - Baseline Item Data
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Po1Segment {
    pub assigned_identification: Option<String>,
    pub quantity_ordered: Option<String>,
    pub unit_or_basis_for_measurement_code: Option<String>,
    pub unit_price: Option<String>,
    pub basis_of_unit_price_code: Option<String>,
    pub product_id_qualifier_1: Option<String>,
    pub product_id_1: Option<String>,
    pub product_id_qualifier_2: Option<String>,
    pub product_id_2: Option<String>,
    pub product_id_qualifier_3: Option<String>,
    pub product_id_3: Option<String>,
    pub product_id_qualifier_4: Option<String>,
    pub product_id_4: Option<String>,
    pub product_id_qualifier_5: Option<String>,
    pub product_id_5: Option<String>,
}

impl SegmentParser for Po1Segment {
    type Output = Self;
    
    fn parse_segment(segment: &Segment) -> Result<Self::Output, EdiError> {
        if segment.id != "PO1" {
            return Err(EdiError::InvalidSegmentFormat(format!("Expected PO1, got {}", segment.id)));
        }
        
        Ok(Po1Segment {
            assigned_identification: segment.elements.get(0).cloned(),
            quantity_ordered: segment.elements.get(1).cloned(),
            unit_or_basis_for_measurement_code: segment.elements.get(2).cloned(),
            unit_price: segment.elements.get(3).cloned(),
            basis_of_unit_price_code: segment.elements.get(4).cloned(),
            product_id_qualifier_1: segment.elements.get(5).cloned(),
            product_id_1: segment.elements.get(6).cloned(),
            product_id_qualifier_2: segment.elements.get(7).cloned(),
            product_id_2: segment.elements.get(8).cloned(),
            product_id_qualifier_3: segment.elements.get(9).cloned(),
            product_id_3: segment.elements.get(10).cloned(),
            product_id_qualifier_4: segment.elements.get(11).cloned(),
            product_id_4: segment.elements.get(12).cloned(),
            product_id_qualifier_5: segment.elements.get(13).cloned(),
            product_id_5: segment.elements.get(14).cloned(),
        })
    }
    
    fn segment_id() -> &'static str {
        "PO1"
    }
    
    fn validate(&self) -> Result<(), EdiError> {
        // PO1 is quite flexible, basic validation
        Ok(())
    }
}

/// CTT - Transaction Totals
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct CttSegment {
    pub number_of_line_items: String,
    pub hash_total: Option<String>,
    pub weight: Option<String>,
    pub unit_or_basis_for_measurement_code: Option<String>,
    pub volume: Option<String>,
    pub unit_or_basis_for_measurement_code_2: Option<String>,
    pub description: Option<String>,
}

impl SegmentParser for CttSegment {
    type Output = Self;
    
    fn parse_segment(segment: &Segment) -> Result<Self::Output, EdiError> {
        if segment.id != "CTT" {
            return Err(EdiError::InvalidSegmentFormat(format!("Expected CTT, got {}", segment.id)));
        }
        
        if segment.elements.is_empty() {
            return Err(EdiError::InvalidSegmentFormat("CTT segment requires at least 1 element".to_string()));
        }
        
        Ok(CttSegment {
            number_of_line_items: segment.elements[0].clone(),
            hash_total: segment.elements.get(1).cloned(),
            weight: segment.elements.get(2).cloned(),
            unit_or_basis_for_measurement_code: segment.elements.get(3).cloned(),
            volume: segment.elements.get(4).cloned(),
            unit_or_basis_for_measurement_code_2: segment.elements.get(5).cloned(),
            description: segment.elements.get(6).cloned(),
        })
    }
    
    fn segment_id() -> &'static str {
        "CTT"
    }
    
    fn validate(&self) -> Result<(), EdiError> {
        if let Ok(count) = self.number_of_line_items.parse::<u32>() {
            if count == 0 {
                return Err(EdiError::ValidationError("Line item count cannot be zero".to_string()));
            }
        } else {
            return Err(EdiError::ValidationError("Invalid line item count format".to_string()));
        }
        Ok(())
    }
}
