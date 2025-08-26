use crate::{
    error::EdiError,
    models::{
        Segment,
        segment_definition::X12Version,
        segment_validator::{SegmentValidator, SegmentValidationResult}
    }
};

/// Trait for all EDI segments that provides validation and structured access
pub trait EdiSegment {
    /// Get the segment ID (e.g., "BEG", "PO1", etc.)
    fn segment_id() -> &'static str;
    
    /// Convert from a generic Segment to this structured type
    fn from_segment(segment: &Segment, version: X12Version) -> Result<Self, EdiError>
    where
        Self: Sized;
    
    /// Convert to a generic Segment
    fn to_segment(&self) -> Segment;
    
    /// Validate this segment
    fn validate(&self, version: X12Version) -> SegmentValidationResult;
}

/// BEG - Beginning Segment for Purchase Order
#[derive(Debug, Clone, PartialEq)]
pub struct BegSegment {
    pub transaction_set_purpose_code: String,
    pub purchase_order_type_code: String,
    pub purchase_order_number: String,
    pub release_number: Option<String>,
    pub date: String,
}

impl EdiSegment for BegSegment {
    fn segment_id() -> &'static str {
        "BEG"
    }

    fn from_segment(segment: &Segment, version: X12Version) -> Result<Self, EdiError> {
        if segment.id != Self::segment_id() {
            return Err(EdiError::InvalidSegmentFormat(
                format!("Expected {} segment, got {}", Self::segment_id(), segment.id)
            ));
        }

        if segment.elements.len() < 4 {
            return Err(EdiError::InvalidSegmentFormat(
                "BEG segment requires at least 4 elements".to_string()
            ));
        }

        // Validate the segment first
        let validator = SegmentValidator::new(version);
        let validation_result = validator.validate_segment(segment);
        if !validation_result.is_valid {
            return Err(EdiError::InvalidSegmentFormat(
                format!("BEG segment validation failed: {:?}", validation_result.errors)
            ));
        }

        Ok(BegSegment {
            transaction_set_purpose_code: segment.elements[0].clone(),
            purchase_order_type_code: segment.elements[1].clone(),
            purchase_order_number: segment.elements[2].clone(),
            release_number: if segment.elements.len() > 3 && !segment.elements[3].is_empty() {
                Some(segment.elements[3].clone())
            } else {
                None
            },
            date: segment.elements.get(4).cloned().unwrap_or_default(),
        })
    }

    fn to_segment(&self) -> Segment {
        let mut elements = vec![
            self.transaction_set_purpose_code.clone(),
            self.purchase_order_type_code.clone(),
            self.purchase_order_number.clone(),
            self.release_number.clone().unwrap_or_default(),
            self.date.clone(),
        ];

        // Remove trailing empty elements (except for required ones)
        while elements.len() > 5 && elements.last().map_or(false, |e| e.is_empty()) {
            elements.pop();
        }

        Segment::new(Self::segment_id().to_string(), elements)
    }

    fn validate(&self, version: X12Version) -> SegmentValidationResult {
        let validator = SegmentValidator::new(version);
        validator.validate_segment(&self.to_segment())
    }
}

/// PO1 - Baseline Item Data
#[derive(Debug, Clone, PartialEq)]
pub struct Po1Segment {
    pub assigned_identification: Option<String>,
    pub quantity_ordered: Option<f64>,
    pub unit_of_measurement_code: Option<String>,
    pub unit_price: Option<f64>,
    pub basis_of_unit_price_code: Option<String>,
    pub product_service_ids: Vec<ProductServiceId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProductServiceId {
    pub qualifier: String,
    pub id: String,
}

impl EdiSegment for Po1Segment {
    fn segment_id() -> &'static str {
        "PO1"
    }

    fn from_segment(segment: &Segment, version: X12Version) -> Result<Self, EdiError> {
        if segment.id != Self::segment_id() {
            return Err(EdiError::InvalidSegmentFormat(
                format!("Expected {} segment, got {}", Self::segment_id(), segment.id)
            ));
        }

        // Validate the segment first
        let validator = SegmentValidator::new(version);
        let validation_result = validator.validate_segment(segment);
        if !validation_result.is_valid {
            return Err(EdiError::InvalidSegmentFormat(
                format!("PO1 segment validation failed: {:?}", validation_result.errors)
            ));
        }

        // Parse Product/Service ID pairs (starting from element 5)
        let mut product_service_ids = Vec::new();
        let mut i = 5;
        while i < segment.elements.len() && i + 1 < segment.elements.len() {
            let qualifier = Self::get_optional_element(&segment.elements, i);
            let id = Self::get_optional_element(&segment.elements, i + 1);
            
            if let (Some(qual), Some(prod_id)) = (qualifier, id) {
                product_service_ids.push(ProductServiceId {
                    qualifier: qual,
                    id: prod_id,
                });
            }
            i += 2;
        }

        Ok(Po1Segment {
            assigned_identification: Self::get_optional_element(&segment.elements, 0),
            quantity_ordered: Self::get_optional_element(&segment.elements, 1)
                .and_then(|s| s.parse().ok()),
            unit_of_measurement_code: Self::get_optional_element(&segment.elements, 2),
            unit_price: Self::get_optional_element(&segment.elements, 3)
                .and_then(|s| s.parse().ok()),
            basis_of_unit_price_code: Self::get_optional_element(&segment.elements, 4),
            product_service_ids,
        })
    }

    fn to_segment(&self) -> Segment {
        let mut elements = vec![
            self.assigned_identification.clone().unwrap_or_default(),
            self.quantity_ordered.map(|q| q.to_string()).unwrap_or_default(),
            self.unit_of_measurement_code.clone().unwrap_or_default(),
            self.unit_price.map(|p| p.to_string()).unwrap_or_default(),
            self.basis_of_unit_price_code.clone().unwrap_or_default(),
        ];

        // Add Product/Service ID pairs
        for product_service_id in &self.product_service_ids {
            elements.push(product_service_id.qualifier.clone());
            elements.push(product_service_id.id.clone());
        }

        Segment::new(Self::segment_id().to_string(), elements)
    }

    fn validate(&self, version: X12Version) -> SegmentValidationResult {
        let validator = SegmentValidator::new(version);
        validator.validate_segment(&self.to_segment())
    }
}

impl Po1Segment {
    fn get_optional_element(elements: &[String], index: usize) -> Option<String> {
        elements.get(index)
            .filter(|s| !s.is_empty())
            .cloned()
    }
}

/// N1 - Party Identification
#[derive(Debug, Clone, PartialEq)]
pub struct N1Segment {
    pub entity_identifier_code: String,
    pub name: Option<String>,
    pub identification_code_qualifier: Option<String>,
    pub identification_code: Option<String>,
}

impl EdiSegment for N1Segment {
    fn segment_id() -> &'static str {
        "N1"
    }

    fn from_segment(segment: &Segment, version: X12Version) -> Result<Self, EdiError> {
        if segment.id != Self::segment_id() {
            return Err(EdiError::InvalidSegmentFormat(
                format!("Expected {} segment, got {}", Self::segment_id(), segment.id)
            ));
        }

        if segment.elements.is_empty() {
            return Err(EdiError::InvalidSegmentFormat(
                "N1 segment requires at least 1 element".to_string()
            ));
        }

        // Validate the segment first
        let validator = SegmentValidator::new(version);
        let validation_result = validator.validate_segment(segment);
        if !validation_result.is_valid {
            return Err(EdiError::InvalidSegmentFormat(
                format!("N1 segment validation failed: {:?}", validation_result.errors)
            ));
        }

        Ok(N1Segment {
            entity_identifier_code: segment.elements[0].clone(),
            name: Self::get_optional_element(&segment.elements, 1),
            identification_code_qualifier: Self::get_optional_element(&segment.elements, 2),
            identification_code: Self::get_optional_element(&segment.elements, 3),
        })
    }

    fn to_segment(&self) -> Segment {
        let mut elements = vec![
            self.entity_identifier_code.clone(),
            self.name.clone().unwrap_or_default(),
            self.identification_code_qualifier.clone().unwrap_or_default(),
            self.identification_code.clone().unwrap_or_default(),
        ];

        // Remove trailing empty elements
        while elements.len() > 1 && elements.last().map_or(false, |e| e.is_empty()) {
            elements.pop();
        }

        Segment::new(Self::segment_id().to_string(), elements)
    }

    fn validate(&self, version: X12Version) -> SegmentValidationResult {
        let validator = SegmentValidator::new(version);
        validator.validate_segment(&self.to_segment())
    }
}

impl N1Segment {
    fn get_optional_element(elements: &[String], index: usize) -> Option<String> {
        elements.get(index)
            .filter(|s| !s.is_empty())
            .cloned()
    }
}

/// CTT - Transaction Totals
#[derive(Debug, Clone, PartialEq)]
pub struct CttSegment {
    pub number_of_line_items: u32,
    pub hash_total: Option<f64>,
}

/// REF - Reference Information
#[derive(Debug, Clone, PartialEq)]
pub struct RefSegment {
    pub reference_identification_qualifier: String,
    pub reference_identification: Option<String>,
    pub description: Option<String>,
}

impl EdiSegment for CttSegment {
    fn segment_id() -> &'static str {
        "CTT"
    }

    fn from_segment(segment: &Segment, version: X12Version) -> Result<Self, EdiError> {
        if segment.id != Self::segment_id() {
            return Err(EdiError::InvalidSegmentFormat(
                format!("Expected {} segment, got {}", Self::segment_id(), segment.id)
            ));
        }

        if segment.elements.is_empty() {
            return Err(EdiError::InvalidSegmentFormat(
                "CTT segment requires at least 1 element".to_string()
            ));
        }

        // Validate the segment first
        let validator = SegmentValidator::new(version);
        let validation_result = validator.validate_segment(segment);
        if !validation_result.is_valid {
            return Err(EdiError::InvalidSegmentFormat(
                format!("CTT segment validation failed: {:?}", validation_result.errors)
            ));
        }

        let number_of_line_items = segment.elements[0].parse()
            .map_err(|_| EdiError::InvalidSegmentFormat(
                "CTT segment first element must be a valid number".to_string()
            ))?;

        let hash_total = if segment.elements.len() > 1 && !segment.elements[1].is_empty() {
            Some(segment.elements[1].parse()
                .map_err(|_| EdiError::InvalidSegmentFormat(
                    "CTT segment hash total must be a valid number".to_string()
                ))?)
        } else {
            None
        };

        Ok(CttSegment {
            number_of_line_items,
            hash_total,
        })
    }

    fn to_segment(&self) -> Segment {
        let mut elements = vec![
            self.number_of_line_items.to_string(),
        ];

        if let Some(hash_total) = self.hash_total {
            elements.push(hash_total.to_string());
        }

        Segment::new(Self::segment_id().to_string(), elements)
    }

    fn validate(&self, version: X12Version) -> SegmentValidationResult {
        let validator = SegmentValidator::new(version);
        validator.validate_segment(&self.to_segment())
    }
}

impl EdiSegment for RefSegment {
    fn segment_id() -> &'static str {
        "REF"
    }

    fn from_segment(segment: &Segment, version: X12Version) -> Result<Self, EdiError> {
        if segment.id != Self::segment_id() {
            return Err(EdiError::InvalidSegmentFormat(
                format!("Expected {} segment, got {}", Self::segment_id(), segment.id)
            ));
        }

        if segment.elements.is_empty() {
            return Err(EdiError::InvalidSegmentFormat(
                "REF segment requires at least 1 element".to_string()
            ));
        }

        // Validate the segment first
        let validator = SegmentValidator::new(version);
        let validation_result = validator.validate_segment(segment);
        if !validation_result.is_valid {
            return Err(EdiError::InvalidSegmentFormat(
                format!("REF segment validation failed: {:?}", validation_result.errors)
            ));
        }

        Ok(RefSegment {
            reference_identification_qualifier: segment.elements[0].clone(),
            reference_identification: Self::get_optional_element(&segment.elements, 1),
            description: Self::get_optional_element(&segment.elements, 2),
        })
    }

    fn to_segment(&self) -> Segment {
        let mut elements = vec![
            self.reference_identification_qualifier.clone(),
            self.reference_identification.clone().unwrap_or_default(),
            self.description.clone().unwrap_or_default(),
        ];

        // Remove trailing empty elements
        while elements.len() > 1 && elements.last().map_or(false, |e| e.is_empty()) {
            elements.pop();
        }

        Segment::new(Self::segment_id().to_string(), elements)
    }

    fn validate(&self, version: X12Version) -> SegmentValidationResult {
        let validator = SegmentValidator::new(version);
        validator.validate_segment(&self.to_segment())
    }
}

impl RefSegment {
    fn get_optional_element(elements: &[String], index: usize) -> Option<String> {
        elements.get(index)
            .filter(|s| !s.is_empty())
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beg_segment_creation() {
        let segment = Segment::new(
            "BEG".to_string(),
            vec![
                "00".to_string(),
                "SA".to_string(),
                "PO-001".to_string(),
                "".to_string(),
                "20230101".to_string(),
            ],
        );

        let beg = BegSegment::from_segment(&segment, X12Version::V4010).unwrap();
        assert_eq!(beg.transaction_set_purpose_code, "00");
        assert_eq!(beg.purchase_order_type_code, "SA");
        assert_eq!(beg.purchase_order_number, "PO-001");
        assert_eq!(beg.release_number, None);
        assert_eq!(beg.date, "20230101");

        let back_to_segment = beg.to_segment();
        assert_eq!(back_to_segment.id, "BEG");
        assert_eq!(back_to_segment.elements.len(), 5);
    }

    #[test]
    fn test_po1_segment_creation() {
        let segment = Segment::new(
            "PO1".to_string(),
            vec![
                "1".to_string(),
                "100".to_string(),
                "EA".to_string(),
                "10.50".to_string(),
                "".to_string(),
                "BP".to_string(),
                "ITEM-001".to_string(),
            ],
        );

        let po1 = Po1Segment::from_segment(&segment, X12Version::V4010).unwrap();
        assert_eq!(po1.assigned_identification, Some("1".to_string()));
        assert_eq!(po1.quantity_ordered, Some(100.0));
        assert_eq!(po1.unit_of_measurement_code, Some("EA".to_string()));
        assert_eq!(po1.unit_price, Some(10.50));
        assert_eq!(po1.basis_of_unit_price_code, None);
        assert_eq!(po1.product_service_ids.len(), 1);
        assert_eq!(po1.product_service_ids[0].qualifier, "BP");
        assert_eq!(po1.product_service_ids[0].id, "ITEM-001");
    }

    #[test]
    fn test_ref_segment_creation() {
        let segment = Segment::new(
            "REF".to_string(),
            vec![
                "PO".to_string(),
                "ORDER123".to_string(),
                "Purchase Order Reference".to_string(),
            ],
        );

        let ref_seg = RefSegment::from_segment(&segment, X12Version::V4010).unwrap();
        assert_eq!(ref_seg.reference_identification_qualifier, "PO");
        assert_eq!(ref_seg.reference_identification, Some("ORDER123".to_string()));
        assert_eq!(ref_seg.description, Some("Purchase Order Reference".to_string()));

        let back_to_segment = ref_seg.to_segment();
        assert_eq!(back_to_segment.id, "REF");
        assert_eq!(back_to_segment.elements.len(), 3);
    }
}
