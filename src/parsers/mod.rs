pub mod x12;
pub mod common;

use crate::{models::InterchangeControl, error::EdiError};

pub trait EdiParser {
    fn parse(&self, input: &str) -> Result<InterchangeControl, EdiError>;
    fn validate(&self, interchange: &InterchangeControl) -> Result<(), EdiError>;
}