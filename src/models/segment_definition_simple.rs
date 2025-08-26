use std::collections::HashMap;
use crate::error::EdiError;

/// Represents the requirement level of an element in an EDI segment
#[derive(Debug, Clone, PartialEq)]
pub enum ElementRequirement {
    Mandatory,
    Optional,
    Conditional,
}

/// Represents the data type of an EDI element
#[derive(Debug, Clone, PartialEq)]
pub enum ElementDataType {
    /// Alphanumeric string
    AN,
    /// Numeric string
    N,
    /// Real number (decimal)
    R,
    /// Identifier (code list)
    ID,
    /// Date (CCYYMMDD)
    DT,
    /// Time (HHMM or HHMMSS)
    TM,
    /// Binary data
    B,
}

/// Definition of a single EDI element with validation rules
#[derive(Debug, Clone)]
pub struct ElementDefinition {
    /// Standard EDI element ID number
    pub element_id: u32,
    /// Human-readable name
    pub name: String,
    /// Data type of the element
    pub data_type: ElementDataType,
    /// Minimum allowed length
    pub min_length: Option<u32>,
    /// Maximum allowed length
    pub max_length: Option<u32>,
    /// Whether this element is mandatory, optional, or conditional
    pub requirement: ElementRequirement,
    /// Description of the element's purpose
    pub description: String,
    /// Valid code values (for ID type elements)
    pub valid_codes: Option<Vec<String>>,
}

/// Definition of an EDI segment with its elements and validation rules
#[derive(Debug, Clone)]
pub struct SegmentDefinition {
    /// Segment identifier (e.g., "BEG", "CUR")
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// List of element definitions for this segment
    pub elements: Vec<ElementDefinition>,
    /// Minimum usage count
    pub min_usage: u32,
    /// Maximum usage count (None = unlimited)
    pub max_usage: Option<u32>,
    /// Description of the segment's purpose
    pub description: String,
}

/// X12 version enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum X12Version {
    V4010,
    V5010,
    V8010,
}

/// Registry for managing segment definitions across different X12 versions
#[derive(Debug)]
pub struct SegmentRegistry {
    definitions: HashMap<(X12Version, String), SegmentDefinition>,
}

impl SegmentRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            definitions: HashMap::new(),
        };
        registry.register_standard_segments();
        registry
    }

    pub fn register_segment(&mut self, version: X12Version, definition: SegmentDefinition) {
        self.definitions.insert((version, definition.id.clone()), definition);
    }

    pub fn get_definition(&self, version: &X12Version, segment_id: &str) -> Option<&SegmentDefinition> {
        self.definitions.get(&(version.clone(), segment_id.to_string()))
    }

    pub fn get_registered_segments_for_version(&self, version: &X12Version) -> Vec<String> {
        self.definitions
            .keys()
            .filter(|(v, _)| v == version)
            .map(|(_, segment_id)| segment_id.clone())
            .collect()
    }

    fn register_standard_segments(&mut self) {
        // Register basic segments
        self.register_beg_segment();
        self.register_cur_segment();
    }

    fn register_beg_segment(&mut self) {
        let beg_definition = SegmentDefinition {
            id: "BEG".to_string(),
            name: "Beginning Segment for Purchase Order".to_string(),
            elements: vec![
                ElementDefinition {
                    element_id: 353,
                    name: "Transaction Set Purpose Code".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(2),
                    requirement: ElementRequirement::Mandatory,
                    description: "Code identifying purpose of transaction set".to_string(),
                    valid_codes: Some(vec![
                        "00".to_string(), // Original
                        "01".to_string(), // Cancellation
                        "04".to_string(), // Change
                        "05".to_string(), // Replace
                    ]),
                },
                ElementDefinition {
                    element_id: 92,
                    name: "Purchase Order Type Code".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(2),
                    requirement: ElementRequirement::Mandatory,
                    description: "Code specifying the type of Purchase Order".to_string(),
                    valid_codes: Some(vec![
                        "NE".to_string(), // New Order
                        "RL".to_string(), // Release or Delivery Order
                        "SA".to_string(), // Stand-alone Order
                    ]),
                },
                ElementDefinition {
                    element_id: 324,
                    name: "Purchase Order Number".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(22),
                    requirement: ElementRequirement::Mandatory,
                    description: "Identifying number for Purchase Order".to_string(),
                    valid_codes: None,
                },
                ElementDefinition {
                    element_id: 328,
                    name: "Release Number".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(30),
                    requirement: ElementRequirement::Optional,
                    description: "Number identifying a release against a Purchase Order".to_string(),
                    valid_codes: None,
                },
                ElementDefinition {
                    element_id: 373,
                    name: "Date".to_string(),
                    data_type: ElementDataType::DT,
                    min_length: Some(8),
                    max_length: Some(8),
                    requirement: ElementRequirement::Mandatory,
                    description: "Date expressed as CCYYMMDD".to_string(),
                    valid_codes: None,
                },
            ],
            min_usage: 1,
            max_usage: Some(1),
            description: "To indicate the beginning of the Purchase Order Transaction Set".to_string(),
        };

        // Register for different versions
        self.register_segment(X12Version::V4010, beg_definition.clone());
        self.register_segment(X12Version::V5010, beg_definition.clone());
        self.register_segment(X12Version::V8010, beg_definition);
    }

    fn register_cur_segment(&mut self) {
        let cur_definition = SegmentDefinition {
            id: "CUR".to_string(),
            name: "Currency".to_string(),
            elements: vec![
                ElementDefinition {
                    element_id: 98,
                    name: "Entity Identifier Code".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(3),
                    requirement: ElementRequirement::Mandatory,
                    description: "Code identifying an organizational entity".to_string(),
                    valid_codes: Some(vec![
                        "BY".to_string(), // Buying Party
                        "SE".to_string(), // Selling Party
                    ]),
                },
                ElementDefinition {
                    element_id: 100,
                    name: "Currency Code".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(3),
                    max_length: Some(3),
                    requirement: ElementRequirement::Mandatory,
                    description: "Code for country currency".to_string(),
                    valid_codes: Some(vec![
                        "USD".to_string(), // US Dollar
                        "CAD".to_string(), // Canadian Dollar
                        "EUR".to_string(), // Euro
                    ]),
                },
            ],
            min_usage: 1,
            max_usage: Some(1),
            description: "To specify the currency being used in the transaction".to_string(),
        };

        // Register for different versions
        self.register_segment(X12Version::V4010, cur_definition.clone());
        self.register_segment(X12Version::V5010, cur_definition.clone());
        self.register_segment(X12Version::V8010, cur_definition);
    }
}

impl Default for SegmentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
