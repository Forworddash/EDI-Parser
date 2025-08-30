use super::{Segment, Transaction, X12Version};

#[derive(Debug, Clone, PartialEq)]
pub struct InterchangeControl {
    pub isa_segment: Segment,
    pub iea_segment: Option<Segment>,
    pub functional_groups: Vec<FunctionalGroup>,
    pub version: X12Version,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionalGroup {
    pub gs_segment: Segment,
    pub ge_segment: Option<Segment>,
    pub transactions: Vec<Transaction>,
}