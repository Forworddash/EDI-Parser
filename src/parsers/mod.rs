pub mod x12;
pub mod common;

use crate::{models::InterchangeControl, error::EdiError};

pub trait EdiParser {
    fn parse(&self, input: &str) -> Result<InterchangeControl, EdiError>;
    fn validate(&self, interchange: &InterchangeControl) -> Result<(), EdiError>;
}

// Re-export the X12 parser for easier access
pub use x12::X12Parser;