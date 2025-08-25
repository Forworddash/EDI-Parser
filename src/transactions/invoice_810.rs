use crate::{models::Transaction, error::EdiError};
use super::TransactionParser;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// Placeholder for 810 Invoice transaction
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Invoice810 {
    // TODO: Implement invoice-specific segments
}

impl TransactionParser for Invoice810 {
    type Output = Self;
    
    fn parse_transaction(_transaction: &Transaction) -> Result<Self::Output, EdiError> {
        // TODO: Implement 810 parsing
        Err(EdiError::UnsupportedTransactionType("810 parsing not yet implemented".to_string()))
    }
    
    fn validate(&self) -> Result<(), EdiError> {
        Ok(())
    }
    
    fn transaction_type() -> &'static str {
        "810"
    }
}
