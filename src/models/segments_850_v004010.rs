// write a struct to hold segment information like this: CUR-01

pub struct SegmentInformation {
    pub segment_name: String,
    pub segment_type: String,
    pub element_id: usize,
    pub position: usize,
    pub requirement: ElementRequirement,
    pub min_length: usize,
    pub max_length: usize,
    pub repeat: usize,
}

