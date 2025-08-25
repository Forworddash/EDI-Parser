use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EdiError {
    #[error("Invalid segment format: {0}")]
    InvalidSegmentFormat(String),
    
    #[error("Missing required segment: {0}")]
    MissingRequiredSegment(String),
    
    #[error("Invalid control structure")]
    InvalidControlStructure,
    
    #[error("Parse error at position {0}: {1}")]
    ParseError(usize, String),
    
    #[error("Unsupported EDI standard: {0}")]
    UnsupportedStandard(String),
    
    #[error("Unsupported transaction type: {0}")]
    UnsupportedTransactionType(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}