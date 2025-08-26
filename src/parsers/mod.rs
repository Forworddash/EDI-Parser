pub mod x12;
pub mod common;
pub mod validating_parser;
pub mod schema_validating_parser;

use crate::{models::InterchangeControl, error::EdiError};

pub trait EdiParser {
    fn parse(&self, input: &str) -> Result<InterchangeControl, EdiError>;
    fn validate(&self, interchange: &InterchangeControl) -> Result<(), EdiError>;
}

// Re-export commonly used types
pub use x12::X12Parser;
pub use validating_parser::ValidatingSegmentParser;