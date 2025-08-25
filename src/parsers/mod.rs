pub mod x12;
pub mod common;
pub mod builder;

use crate::{models::InterchangeControl, error::EdiError};

pub trait EdiParser {
    fn parse(&self, input: &str) -> Result<InterchangeControl, EdiError>;
    fn validate(&self, interchange: &InterchangeControl) -> Result<(), EdiError>;
}

// Re-export commonly used types
pub use x12::X12Parser;
pub use builder::X12ParserBuilder;