use super::Segment;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub segments: Vec<Segment>,
    pub transaction_set_id: String,
    pub control_number: String,
}

impl Transaction {
    pub fn new(segments: Vec<Segment>, transaction_set_id: String, control_number: String) -> Self {
        Self {
            segments,
            transaction_set_id,
            control_number,
        }
    }
}