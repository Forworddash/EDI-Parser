use super::Segment;

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Invoice810,
    PurchaseOrder850,
    Unknown(String),
}

impl TransactionType {
    pub fn from_id(id: &str) -> Self {
        match id {
            "810" => Self::Invoice810,
            "850" => Self::PurchaseOrder850,
            other => Self::Unknown(other.to_string()),
        }
    }

    pub fn required_segments(&self) -> Vec<&str> {
        match self {
            Self::Invoice810 => vec!["ST", "BIG", "SE"],
            Self::PurchaseOrder850 => vec!["ST", "BEG", "SE"], // Core required
            Self::Unknown(_) => vec!["ST", "SE"],
        }
    }

    pub fn optional_segments(&self) -> Vec<&str> {
        match self {
            Self::Invoice810 => vec!["N1", "IT1", "TDS", "CTT"],
            Self::PurchaseOrder850 => vec!["N1", "PO1", "CTT", "DTM", "REF", "PER", "FOB", "ITD", "PID", "SAC"],
            Self::Unknown(_) => vec![],
        }
    }

    pub fn validate_segment(&self, segment: &Segment) -> Result<(), String> {
        match self {
            Self::PurchaseOrder850 => self.validate_850_segment(segment),
            Self::Invoice810 => self.validate_810_segment(segment),
            Self::Unknown(_) => Ok(()), // No specific validation for unknown types
        }
    }

    fn validate_850_segment(&self, segment: &Segment) -> Result<(), String> {
        match segment.id.as_str() {
            "BEG" => {
                // BEG: Beginning Segment for Purchase Order
                // BEG01: Transaction Set Purpose Code (00=Original, 01=Cancellation, etc.)
                // BEG02: Purchase Order Type Code (SA=Stand-alone order)
                // BEG03: Purchase Order Number
                if segment.elements.len() < 3 {
                    return Err("BEG segment requires at least 3 elements".to_string());
                }
                let purpose_code = &segment.elements[0];
                if !["00", "01", "04", "05", "06", "07"].contains(&purpose_code.as_str()) {
                    return Err(format!("Invalid BEG01 purpose code: {}", purpose_code));
                }
                Ok(())
            }
            "PO1" => {
                // PO1: Purchase Order Line Item
                // PO101: Assigned Identification
                // PO102: Quantity Ordered
                // PO103: Unit of Measure
                // PO104: Unit Price
                if segment.elements.len() < 4 {
                    return Err("PO1 segment requires at least 4 elements".to_string());
                }
                // Validate quantity is numeric
                if let Ok(_) = segment.elements[1].parse::<f64>() {
                    // Valid number
                } else {
                    return Err(format!("PO102 quantity must be numeric: {}", segment.elements[1]));
                }
                Ok(())
            }
            "N1" => {
                // N1: Name
                // N101: Entity Identifier Code (ST=Ship To, BT=Bill To, etc.)
                // N102: Name
                if segment.elements.len() < 2 {
                    return Err("N1 segment requires at least 2 elements".to_string());
                }
                let entity_code = &segment.elements[0];
                if !["ST", "BT", "SF", "BS", "BY", "SE", "SU"].contains(&entity_code.as_str()) {
                    return Err(format!("Invalid N101 entity code: {}", entity_code));
                }
                Ok(())
            }
            _ => Ok(()), // Other segments don't have specific validation yet
        }
    }

    fn validate_810_segment(&self, segment: &Segment) -> Result<(), String> {
        match segment.id.as_str() {
            "BIG" => {
                // BIG: Beginning Segment for Invoice
                // BIG01: Date
                // BIG02: Invoice Number
                if segment.elements.len() < 2 {
                    return Err("BIG segment requires at least 2 elements".to_string());
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Invoice810 => "810",
            Self::PurchaseOrder850 => "850",
            Self::Unknown(s) => s,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub segments: Vec<Segment>,
    pub transaction_set_id: String,
    pub control_number: String,
    pub transaction_type: TransactionType,
}

impl Transaction {
    pub fn new(segments: Vec<Segment>, transaction_set_id: String, control_number: String) -> Self {
        let transaction_type = TransactionType::from_id(&transaction_set_id);
        Self {
            segments,
            transaction_set_id,
            control_number,
            transaction_type,
        }
    }
}