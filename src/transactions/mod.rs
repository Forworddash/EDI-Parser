pub mod purchase_order_850;
pub mod invoice_810;

use crate::{models::Transaction, error::EdiError};

/// Trait for parsing raw transactions into structured business documents
pub trait TransactionParser {
    type Output;
    
    fn parse_transaction(transaction: &Transaction) -> Result<Self::Output, EdiError>;
    fn validate(&self) -> Result<(), EdiError>;
    fn transaction_type() -> &'static str;
}

// Re-export transaction types
pub use purchase_order_850::PurchaseOrder850;
pub use invoice_810::Invoice810;
