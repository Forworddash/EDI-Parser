//! # EDI Parser
//!
//! A high-performance EDI (Electronic Data Interchange) parser written in Rust,
//! supporting X12 format with extensible segment validation and loop-aware parsing.
//!
//! ## Features
//!
//! - ✅ **X12 Standard Support**: Full X12 EDI parsing with version detection (4010, 5010, 6010)
//! - ✅ **Document Type Recognition**: Automatic detection of 810 (Invoice), 850 (Purchase Order), and custom transaction types
//! - ✅ **Segment Validation**: Built-in validation for common segments (BEG, PO1, N1, DTM, etc.)
//! - ✅ **Loop-Aware Parsing**: Structured parsing of EDI loops (party loops, line item loops)
//! - ✅ **Extensible Architecture**: Easy to add new segments, document types, and validation rules
//! - ✅ **Error Handling**: Comprehensive error reporting with detailed validation messages
//! - ✅ **Performance**: Zero-copy parsing with efficient memory usage
//!
//! ## Quick Start
//!
//! ```rust
//! use edi_parser::{X12Parser, EdiParser};
//!
//! let parser = X12Parser::default();
//! let edi_data = "ISA*00*...*IEA*1*000000001~"; // Your EDI data
//! let result = parser.parse(&edi_data);
//!
//! match result {
//!     Ok(interchange) => {
//!         println!("Parsed {} transactions", interchange.functional_groups[0].transactions.len());
//!         println!("EDI Version: {}", interchange.version.as_str());
//!     }
//!     Err(e) => println!("Parse error: {}", e),
//! }
//! ```
//!
//! ## Basic Usage
//!
//! ```rust
//! use edi_parser::{X12Parser, EdiParser, InterchangeControl, EdiError};
//! use std::fs;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Read EDI file
//!     let content = fs::read_to_string("tests/test_files/sample_850.edi")?;
//!
//!     // Parse the content
//!     let parser = X12Parser::default();
//!     let interchange = parser.parse(&content)?;
//!
//!     // Validate the parsed structure
//!     parser.validate(&interchange)?;
//!
//!     // Access parsed data
//!     println!("Version: {}", interchange.version.as_str());
//!     for fg in &interchange.functional_groups {
//!         for transaction in &fg.transactions {
//!             println!("Transaction: {} ({})",
//!                 transaction.transaction_set_id,
//!                 transaction.transaction_type.as_str());
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Structured Loop Parsing
//!
//! For more advanced use cases, you can parse EDI into structured loops:
//!
//! ```rust
//! use edi_parser::{X12Parser, EdiParser, PurchaseOrder850};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let edi_data = "ISA*00*          *00*          *01*SENDERID     *01*RECEIVERID   *230101*1253*U*00401*000000001*0*T*>~GS*PO*SENDERID*RECEIVERID*20230101*1253*1*X*004010~ST*850*0001~BEG*00*SA*PO-001**20230101~N1*ST*ABC Corporation*92*12345~PO1*1*100*EA*10.50**BP*ITEM-001~CTT*1~SE*6*0001~GE*1*1~IEA*1*000000001~";
//! let parser = X12Parser::default();
//! let interchange = parser.parse(&edi_data)?;
//!
//! // Parse into structured loops
//! for fg in &interchange.functional_groups {
//!     for transaction in &fg.transactions {
//!         if let Ok(po850) = PurchaseOrder850::parse_from_transaction(transaction) {
//!             println!("PO Number: {}", po850.get_total_line_items());
//!             println!("Total Quantity: {}", po850.get_total_quantity());
//!
//!             // Access specific parties
//!             let ship_to_parties = po850.get_parties_by_type("ST");
//!             for party in ship_to_parties {
//!                 println!("Ship To: {}", party.n1_segment.elements[1]);
//!             }
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! The parser provides detailed error messages:
//!
//! ```rust
//! use edi_parser::{X12Parser, EdiParser, EdiError};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let parser = X12Parser::default();
//! let edi_data = "INVALID*DATA*HERE~";
//!
//! match parser.parse(&edi_data) {
//!     Ok(interchange) => { /* Success */ }
//!     Err(EdiError::InvalidSegmentFormat(msg)) => {
//!         println!("Segment format error: {}", msg);
//!     }
//!     Err(EdiError::MissingRequiredSegment(seg)) => {
//!         println!("Missing required segment: {}", seg);
//!     }
//!     Err(EdiError::ValidationError(msg)) => {
//!         println!("Validation error: {}", msg);
//!     }
//!     _ => { /* Other errors */ }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Adding Custom Segments
//!
//! ### Method 1: Simple Validation
//! ```rust
//! use edi_parser::{TransactionType, Segment};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Example of custom segment validation
//! let segment = Segment::new("MY_SEG".to_string(), vec!["A".to_string(), "B".to_string()]);
//!
//! match segment.id.as_str() {
//!     "MY_SEG" => {
//!         if segment.elements.len() < 2 {
//!             println!("MY_SEG requires at least 2 elements");
//!         } else {
//!             println!("MY_SEG validation passed");
//!         }
//!     }
//!     _ => println!("Unknown segment"),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Method 2: Loop-Based Parsing
//! Extend the loop structures for more complex parsing.
//!
//! ## Performance
//!
//! - **Zero-copy parsing** where possible
//! - **Efficient memory usage** with Vec-based storage
//! - **Fast validation** with early error detection
//! - **Scalable architecture** for large EDI files

pub mod error;
pub mod models;
pub mod parsers;
pub mod utils;

// Re-export the main types for easier access
pub use error::EdiError;
pub use models::*;
pub use parsers::*;