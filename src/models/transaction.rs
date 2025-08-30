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
            Self::PurchaseOrder850 => vec!["ST", "BEG", "SE"],
            Self::Unknown(_) => vec!["ST", "SE"], // minimal requirements
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