use crate::{
    models::Transaction,
    error::EdiError,
    segments::{StSegment, BegSegment, N1Segment, Po1Segment, CttSegment, SegmentParser},
};
use super::TransactionParser;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// Represents a complete 850 Purchase Order transaction
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct PurchaseOrder850 {
    pub header: StSegment,
    pub beginning_segment: BegSegment,
    pub parties: Vec<N1Segment>,
    pub line_items: Vec<Po1Segment>,
    pub totals: Option<CttSegment>,
    pub trailer: Option<SeSegment>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct SeSegment {
    pub number_of_included_segments: String,
    pub transaction_set_control_number: String,
}

impl SegmentParser for SeSegment {
    type Output = Self;
    
    fn parse_segment(segment: &crate::models::Segment) -> Result<Self::Output, EdiError> {
        if segment.id != "SE" {
            return Err(EdiError::InvalidSegmentFormat(format!("Expected SE, got {}", segment.id)));
        }
        
        if segment.elements.len() < 2 {
            return Err(EdiError::InvalidSegmentFormat("SE segment requires 2 elements".to_string()));
        }
        
        Ok(SeSegment {
            number_of_included_segments: segment.elements[0].clone(),
            transaction_set_control_number: segment.elements[1].clone(),
        })
    }
    
    fn segment_id() -> &'static str {
        "SE"
    }
    
    fn validate(&self) -> Result<(), EdiError> {
        if self.transaction_set_control_number.trim().is_empty() {
            return Err(EdiError::ValidationError("SE control number cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl TransactionParser for PurchaseOrder850 {
    type Output = Self;
    
    fn parse_transaction(transaction: &Transaction) -> Result<Self::Output, EdiError> {
        if transaction.transaction_set_id != "850" {
            return Err(EdiError::UnsupportedTransactionType(transaction.transaction_set_id.clone()));
        }
        
        let mut header: Option<StSegment> = None;
        let mut beginning_segment: Option<BegSegment> = None;
        let mut parties: Vec<N1Segment> = Vec::new();
        let mut line_items: Vec<Po1Segment> = Vec::new();
        let mut totals: Option<CttSegment> = None;
        let mut trailer: Option<SeSegment> = None;
        
        for segment in &transaction.segments {
            match segment.id.as_str() {
                "ST" => {
                    header = Some(StSegment::parse_segment(segment)?);
                }
                "BEG" => {
                    beginning_segment = Some(BegSegment::parse_segment(segment)?);
                }
                "N1" => {
                    parties.push(N1Segment::parse_segment(segment)?);
                }
                "PO1" => {
                    line_items.push(Po1Segment::parse_segment(segment)?);
                }
                "CTT" => {
                    totals = Some(CttSegment::parse_segment(segment)?);
                }
                "SE" => {
                    trailer = Some(SeSegment::parse_segment(segment)?);
                }
                _ => {
                    // For now, ignore unknown segments
                    // In a production system, you might want to collect them
                }
            }
        }
        
        Ok(PurchaseOrder850 {
            header: header.ok_or_else(|| EdiError::MissingRequiredSegment("ST".to_string()))?,
            beginning_segment: beginning_segment.ok_or_else(|| EdiError::MissingRequiredSegment("BEG".to_string()))?,
            parties,
            line_items,
            totals,
            trailer,
        })
    }
    
    fn validate(&self) -> Result<(), EdiError> {
        // Validate required segments
        self.header.validate()?;
        self.beginning_segment.validate()?;
        
        // Validate that control numbers match
        if let Some(trailer) = &self.trailer {
            trailer.validate()?;
            if self.header.transaction_set_control_number != trailer.transaction_set_control_number {
                return Err(EdiError::ValidationError(
                    "ST and SE control numbers must match".to_string()
                ));
            }
        }
        
        // Validate line item count if CTT is present
        if let Some(totals) = &self.totals {
            totals.validate()?;
            if let Ok(expected_count) = totals.number_of_line_items.parse::<usize>() {
                if expected_count != self.line_items.len() {
                    return Err(EdiError::ValidationError(
                        format!("Expected {} line items, found {}", expected_count, self.line_items.len())
                    ));
                }
            }
        }
        
        // Validate all parties
        for party in &self.parties {
            party.validate()?;
        }
        
        // Validate all line items
        for item in &self.line_items {
            item.validate()?;
        }
        
        Ok(())
    }
    
    fn transaction_type() -> &'static str {
        "850"
    }
}

impl PurchaseOrder850 {
    /// Get the purchase order number
    pub fn purchase_order_number(&self) -> &str {
        &self.beginning_segment.purchase_order_number
    }
    
    /// Get the buyer party (usually entity code 'BY')
    pub fn buyer(&self) -> Option<&N1Segment> {
        self.parties.iter().find(|p| p.entity_identifier_code == "BY")
    }
    
    /// Get the ship-to party (usually entity code 'ST')
    pub fn ship_to(&self) -> Option<&N1Segment> {
        self.parties.iter().find(|p| p.entity_identifier_code == "ST")
    }
    
    /// Get the vendor party (usually entity code 'VN')
    pub fn vendor(&self) -> Option<&N1Segment> {
        self.parties.iter().find(|p| p.entity_identifier_code == "VN")
    }
    
    /// Calculate total order value if unit prices are available
    pub fn total_value(&self) -> Option<f64> {
        let mut total = 0.0;
        for item in &self.line_items {
            if let (Some(qty_str), Some(price_str)) = (&item.quantity_ordered, &item.unit_price) {
                if let (Ok(qty), Ok(price)) = (qty_str.parse::<f64>(), price_str.parse::<f64>()) {
                    total += qty * price;
                } else {
                    return None; // Invalid number format
                }
            } else {
                return None; // Missing price or quantity
            }
        }
        Some(total)
    }
}
