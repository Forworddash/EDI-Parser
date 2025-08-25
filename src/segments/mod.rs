// Control segments
pub mod control;

// Common business segments
pub mod common;

// Transaction-specific segments
pub mod purchase_order; // For 850

// Re-export commonly used segment types
pub use control::*;
pub use common::*;
pub use purchase_order::*;

use crate::error::EdiError;

/// Trait for parsing EDI segments into structured data
pub trait SegmentParser {
    type Output;
    
    fn parse_segment(segment: &crate::models::Segment) -> Result<Self::Output, EdiError>;
    fn segment_id() -> &'static str;
    fn validate(&self) -> Result<(), EdiError>;
}
