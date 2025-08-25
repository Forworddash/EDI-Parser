#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Segment {
    pub id: String,
    pub elements: Vec<String>,
}

impl Segment {
    pub fn new(id: String, elements: Vec<String>) -> Self {
        Self { id, elements }
    }
}