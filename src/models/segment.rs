#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub id: String,
    pub elements: Vec<String>,
}

impl Segment {
    pub fn new(id: String, elements: Vec<String>) -> Self {
        Self { id, elements }
    }
}