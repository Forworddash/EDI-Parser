use std::collections::HashMap;

/// Defines the data type and validation rules for an EDI element
#[derive(Debug, Clone)]
pub struct ElementDefinition {
    pub name: String,
    pub data_type: ElementDataType,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub required: bool,
    pub description: String,
}

/// Supported EDI element data types
#[derive(Debug, Clone)]
pub enum ElementDataType {
    /// Alphanumeric string
    AN,
    /// Numeric string
    N,
    /// Decimal number
    R,
    /// Identifier (coded value)
    ID,
    /// Date in various formats
    DT,
    /// Time in various formats
    TM,
}

/// Defines the structure and validation rules for an EDI segment
#[derive(Debug, Clone)]
pub struct SegmentDefinition {
    pub id: String,
    pub name: String,
    pub elements: Vec<ElementDefinition>,
    pub min_usage: u32,
    pub max_usage: Option<u32>, // None means unlimited
    pub description: String,
}

/// X12 implementation guide version
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum X12Version {
    V4010,
    V5010,
    V8010,
}

/// Registry for segment definitions across different X12 versions
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
            .map(|(_, id)| id.clone())
            .collect()
    }

    fn register_standard_segments(&mut self) {
        // Register BEG segment for 850 Purchase Order
        self.register_beg_segment();
        self.register_po1_segment();
        self.register_n1_segment();
        self.register_ctt_segment();
        self.register_ref_segment();
        self.register_dtm_segment();
    }

    fn register_beg_segment(&mut self) {
        let beg_definition = SegmentDefinition {
            id: "BEG".to_string(),
            name: "Beginning Segment for Purchase Order".to_string(),
            elements: vec![
                ElementDefinition {
                    name: "Transaction Set Purpose Code".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(2),
                    required: true,
                    description: "Code identifying purpose of transaction set".to_string(),
                },
                ElementDefinition {
                    name: "Purchase Order Type Code".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(2),
                    required: true,
                    description: "Code specifying the type of Purchase Order".to_string(),
                },
                ElementDefinition {
                    name: "Purchase Order Number".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(22),
                    required: true,
                    description: "Identifying number for Purchase Order".to_string(),
                },
                ElementDefinition {
                    name: "Release Number".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(30),
                    required: false,
                    description: "Number identifying a release against a Purchase Order".to_string(),
                },
                ElementDefinition {
                    name: "Date".to_string(),
                    data_type: ElementDataType::DT,
                    min_length: Some(8),
                    max_length: Some(8),
                    required: true,
                    description: "Date expressed as CCYYMMDD".to_string(),
                },
            ],
            min_usage: 1,
            max_usage: Some(1),
            description: "To indicate the beginning of the Purchase Order Transaction Set and transmit identifying numbers and dates".to_string(),
        };

        // Register for different X12 versions
        self.register_segment(X12Version::V4010, beg_definition.clone());
        self.register_segment(X12Version::V5010, beg_definition.clone());
        self.register_segment(X12Version::V8010, beg_definition);
    }

    fn register_po1_segment(&mut self) {
        let po1_definition = SegmentDefinition {
            id: "PO1".to_string(),
            name: "Baseline Item Data".to_string(),
            elements: vec![
                ElementDefinition {
                    name: "Assigned Identification".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(20),
                    required: false,
                    description: "Alphanumeric characters assigned for differentiation within a transaction set".to_string(),
                },
                ElementDefinition {
                    name: "Quantity Ordered".to_string(),
                    data_type: ElementDataType::R,
                    min_length: Some(1),
                    max_length: Some(15),
                    required: false,
                    description: "Quantity ordered".to_string(),
                },
                ElementDefinition {
                    name: "Unit or Basis for Measurement Code".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(2),
                    required: false,
                    description: "Code specifying the units in which a value is being expressed".to_string(),
                },
                ElementDefinition {
                    name: "Unit Price".to_string(),
                    data_type: ElementDataType::R,
                    min_length: Some(1),
                    max_length: Some(17),
                    required: false,
                    description: "Price per unit of product, service, commodity, etc.".to_string(),
                },
                ElementDefinition {
                    name: "Basis of Unit Price Code".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(2),
                    required: false,
                    description: "Code identifying the type of unit price for an item".to_string(),
                },
                ElementDefinition {
                    name: "Product/Service ID Qualifier 1".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(2),
                    required: false,
                    description: "Code identifying the type/source of the descriptive number used in Product/Service ID".to_string(),
                },
                ElementDefinition {
                    name: "Product/Service ID 1".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(48),
                    required: false,
                    description: "Identifying number for a product or service".to_string(),
                },
                ElementDefinition {
                    name: "Product/Service ID Qualifier 2".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(2),
                    required: false,
                    description: "Code identifying the type/source of the descriptive number used in Product/Service ID".to_string(),
                },
                ElementDefinition {
                    name: "Product/Service ID 2".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(48),
                    required: false,
                    description: "Identifying number for a product or service".to_string(),
                },
                ElementDefinition {
                    name: "Product/Service ID Qualifier 3".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(2),
                    required: false,
                    description: "Code identifying the type/source of the descriptive number used in Product/Service ID".to_string(),
                },
                ElementDefinition {
                    name: "Product/Service ID 3".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(48),
                    required: false,
                    description: "Identifying number for a product or service".to_string(),
                },
            ],
            min_usage: 1,
            max_usage: Some(100000), // Very high limit for line items
            description: "To specify basic and most frequently used line item data for the purchase order".to_string(),
        };

        self.register_segment(X12Version::V4010, po1_definition.clone());
        self.register_segment(X12Version::V5010, po1_definition.clone());
        self.register_segment(X12Version::V8010, po1_definition);
    }

    fn register_n1_segment(&mut self) {
        let n1_definition = SegmentDefinition {
            id: "N1".to_string(),
            name: "Party Identification".to_string(),
            elements: vec![
                ElementDefinition {
                    name: "Entity Identifier Code".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(3),
                    required: true,
                    description: "Code identifying an organizational entity".to_string(),
                },
                ElementDefinition {
                    name: "Name".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(60),
                    required: false,
                    description: "Free-form name".to_string(),
                },
                ElementDefinition {
                    name: "Identification Code Qualifier".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(1),
                    max_length: Some(2),
                    required: false,
                    description: "Code designating the system/method of code structure used for Identification Code".to_string(),
                },
                ElementDefinition {
                    name: "Identification Code".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(2),
                    max_length: Some(80),
                    required: false,
                    description: "Code identifying a party or other code".to_string(),
                },
            ],
            min_usage: 0,
            max_usage: Some(200),
            description: "To identify a party by type of organization, name, and code".to_string(),
        };

        self.register_segment(X12Version::V4010, n1_definition.clone());
        self.register_segment(X12Version::V5010, n1_definition.clone());
        self.register_segment(X12Version::V8010, n1_definition);
    }

    fn register_ctt_segment(&mut self) {
        let ctt_definition = SegmentDefinition {
            id: "CTT".to_string(),
            name: "Transaction Totals".to_string(),
            elements: vec![
                ElementDefinition {
                    name: "Number of Line Items".to_string(),
                    data_type: ElementDataType::N,
                    min_length: Some(1),
                    max_length: Some(6),
                    required: true,
                    description: "Total number of line items in the transaction set".to_string(),
                },
                ElementDefinition {
                    name: "Hash Total".to_string(),
                    data_type: ElementDataType::R,
                    min_length: Some(1),
                    max_length: Some(10),
                    required: false,
                    description: "Sum of values of the specified data element".to_string(),
                },
            ],
            min_usage: 0,
            max_usage: Some(1),
            description: "To transmit a hash total for a specific element in the transaction set".to_string(),
        };

        self.register_segment(X12Version::V4010, ctt_definition.clone());
        self.register_segment(X12Version::V5010, ctt_definition.clone());
        self.register_segment(X12Version::V8010, ctt_definition);
    }

    fn register_ref_segment(&mut self) {
        let ref_definition = SegmentDefinition {
            id: "REF".to_string(),
            name: "Reference Information".to_string(),
            elements: vec![
                ElementDefinition {
                    name: "Reference Identification Qualifier".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(3),
                    required: true,
                    description: "Code qualifying the Reference Identification".to_string(),
                },
                ElementDefinition {
                    name: "Reference Identification".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(50),
                    required: false,
                    description: "Reference information as defined for a particular Transaction Set".to_string(),
                },
                ElementDefinition {
                    name: "Description".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(80),
                    required: false,
                    description: "A free-form description to clarify the related data elements".to_string(),
                },
            ],
            min_usage: 0,
            max_usage: Some(12),
            description: "To specify identifying information".to_string(),
        };

        self.register_segment(X12Version::V4010, ref_definition.clone());
        self.register_segment(X12Version::V5010, ref_definition.clone());
        self.register_segment(X12Version::V8010, ref_definition);
    }

    fn register_dtm_segment(&mut self) {
        let dtm_definition = SegmentDefinition {
            id: "DTM".to_string(),
            name: "Date/Time Reference".to_string(),
            elements: vec![
                ElementDefinition {
                    name: "Date/Time Qualifier".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(3),
                    max_length: Some(3),
                    required: true,
                    description: "Code specifying type of date or time or both date and time".to_string(),
                },
                ElementDefinition {
                    name: "Date".to_string(),
                    data_type: ElementDataType::DT,
                    min_length: Some(8),
                    max_length: Some(8),
                    required: false,
                    description: "Date expressed as CCYYMMDD".to_string(),
                },
                ElementDefinition {
                    name: "Time".to_string(),
                    data_type: ElementDataType::TM,
                    min_length: Some(4),
                    max_length: Some(8),
                    required: false,
                    description: "Time expressed in 24-hour clock time".to_string(),
                },
            ],
            min_usage: 0,
            max_usage: Some(10),
            description: "To specify pertinent dates and times".to_string(),
        };

        self.register_segment(X12Version::V4010, dtm_definition.clone());
        self.register_segment(X12Version::V5010, dtm_definition.clone());
        self.register_segment(X12Version::V8010, dtm_definition);
    }
}

impl Default for SegmentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
