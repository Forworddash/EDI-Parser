#[derive(Debug, Clone, PartialEq)]
pub enum X12Version {
    V4010,
    V5010,
    V6010,
    Unknown(String),
}

impl X12Version {
    pub fn from_isa(isa_segment: &super::Segment) -> Result<Self, super::super::error::EdiError> {
        if isa_segment.elements.len() < 12 {
            return Err(super::super::error::EdiError::InvalidSegmentFormat(
                "ISA segment too short for version".to_string()
            ));
        }
        match isa_segment.elements[11].as_str() {
            "00401" => Ok(Self::V4010),
            "00501" => Ok(Self::V5010),
            "00601" => Ok(Self::V6010),
            other => Ok(Self::Unknown(other.to_string())),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::V4010 => "00401",
            Self::V5010 => "00501",
            Self::V6010 => "00601",
            Self::Unknown(s) => s,
        }
    }
}
