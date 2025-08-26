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

/// Definition of an EDI segment with its elements and rules
#[derive(Debug, Clone)]
pub struct SegmentDefinition {
    /// Segment identifier (e.g., "BEG", "CUR")
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// List of element definitions in order
    pub elements: Vec<ElementDefinition>,
    /// Minimum number of times this segment can appear
    pub min_usage: u32,
    /// Maximum number of times this segment can appear (None = unlimited)
    pub max_usage: Option<u32>,
    /// Description of the segment's purpose
    pub description: String,
}

/// X12 version enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            .map(|(_, id)| id.clone())
            .collect()
    }

    fn register_standard_segments(&mut self) {
        // Register standard EDI segments for purchase orders
        self.register_beg_segment();
        self.register_cur_segment();
        self.register_ref_segment();
        self.register_po1_segment();
        // Skip DTM, N1, CTT for now (placeholders)
        // self.register_dtm_segment();
        // self.register_n1_segment();
        // self.register_ctt_segment();
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
                    description: "Number identifying a release against a Purchase Order previously placed by the parties involved in the transaction".to_string(),
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
            description: "To indicate the beginning of the Purchase Order Transaction Set and transmit identifying numbers and dates".to_string(),
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
                    description: "Code identifying an organizational entity, a physical location, property or an individual".to_string(),
                    valid_codes: Some(vec![
                        "BY".to_string(), // Buying Party
                        "SE".to_string(), // Selling Party
                        "PR".to_string(), // Payer
                        "PE".to_string(), // Payee
                    ]),
                },
                ElementDefinition {
                    element_id: 100,
                    name: "Currency Code".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(3),
                    max_length: Some(3),
                    requirement: ElementRequirement::Mandatory,
                    description: "Code (Standard ISO) for country in whose currency the charges are specified".to_string(),
                    valid_codes: Some(vec![
                        "USD".to_string(), // US Dollar
                        "CAD".to_string(), // Canadian Dollar
                        "EUR".to_string(), // Euro
                        "GBP".to_string(), // British Pound
                        "JPY".to_string(), // Japanese Yen
                    ]),
                },
                ElementDefinition {
                    element_id: 280,
                    name: "Exchange Rate".to_string(),
                    data_type: ElementDataType::R,
                    min_length: Some(4),
                    max_length: Some(10),
                    requirement: ElementRequirement::Optional,
                    description: "Value to be used as a multiplier conversion factor to convert monetary value from one currency to another".to_string(),
                    valid_codes: None,
                },
            ],
            min_usage: 0,
            max_usage: Some(1),
            description: "To specify the currency (dollars, pounds, francs, etc.) used in a transaction".to_string(),
        };

        // Register for different versions
        self.register_segment(X12Version::V4010, cur_definition.clone());
        self.register_segment(X12Version::V5010, cur_definition.clone());
        self.register_segment(X12Version::V8010, cur_definition);
    }

    fn register_po1_segment(&mut self) {
        let po1_definition = SegmentDefinition {
            id: "PO1".to_string(),
            name: "Baseline Item Data".to_string(),
            elements: vec![
                ElementDefinition {
                    element_id: 350,
                    name: "Assigned Identification".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(20),
                    requirement: ElementRequirement::Optional,
                    description: "Alphanumeric characters assigned for differentiation within a transaction set".to_string(),
                    valid_codes: None,
                },
                ElementDefinition {
                    element_id: 330,
                    name: "Quantity Ordered".to_string(),
                    data_type: ElementDataType::R,
                    min_length: Some(1),
                    max_length: Some(15),
                    requirement: ElementRequirement::Mandatory,
                    description: "Quantity ordered".to_string(),
                    valid_codes: None,
                },
            ],
            min_usage: 1,
            max_usage: Some(100000),
            description: "To specify basic and most frequently used line item data".to_string(),
        };

        self.register_segment(X12Version::V4010, po1_definition.clone());
        self.register_segment(X12Version::V5010, po1_definition.clone());
        self.register_segment(X12Version::V8010, po1_definition);
    }

    fn register_ref_segment(&mut self) {
        let ref_definition = SegmentDefinition {
            id: "REF".to_string(),
            name: "Reference Identification".to_string(),
            elements: vec![
                ElementDefinition {
                    element_id: 128,
                    name: "Reference Identification Qualifier".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(3),
                    requirement: ElementRequirement::Mandatory,
                    description: "Code qualifying the Reference Identification".to_string(),
                    valid_codes: Some(vec![
                        "DP".to_string(), // Department Number
                        "IA".to_string(), // Internal Vendor Number
                        "PD".to_string(), // Promotion/Deal Number
                    ]),
                },
                ElementDefinition {
                    element_id: 127,
                    name: "Reference Identification".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(50),
                    requirement: ElementRequirement::Optional,
                    description: "Reference information as defined for a particular Transaction Set".to_string(),
                    valid_codes: None,
                },
            ],
            min_usage: 1,
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
                    element_id: 374,
                    name: "Date/Time Qualifier".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(3),
                    max_length: Some(3),
                    requirement: ElementRequirement::Mandatory,
                    description: "Code specifying type of date or time, or both date and time".to_string(),
                    valid_codes: Some(vec![
                        "002".to_string(), // Delivery Requested
                        "010".to_string(), // Requested Ship
                        "037".to_string(), // Ship Not Before
                        "038".to_string(), // Ship Not Later Than
                    ]),
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
                ElementDefinition {
                    element_id: 337,
                    name: "Time".to_string(),
                    data_type: ElementDataType::TM,
                    min_length: Some(4),
                    max_length: Some(8),
                    requirement: ElementRequirement::Optional,
                    description: "Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD".to_string(),
                    valid_codes: None,
                },
            ],
            min_usage: 0,
            max_usage: None, // Can repeat
            description: "To specify pertinent dates and times".to_string(),
        };

        self.register_segment(X12Version::V4010, dtm_definition.clone());
        self.register_segment(X12Version::V5010, dtm_definition.clone());
        self.register_segment(X12Version::V8010, dtm_definition);
    }

    fn register_n1_segment(&mut self) {
        let n1_definition = SegmentDefinition {
            id: "N1".to_string(),
            name: "Name".to_string(),
            elements: vec![
                ElementDefinition {
                    element_id: 98,
                    name: "Entity Identifier Code".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(3),
                    requirement: ElementRequirement::Mandatory,
                    description: "Code identifying an organizational entity, a physical location, property or an individual".to_string(),
                    valid_codes: Some(vec![
                        "BY".to_string(), // Buyer
                        "ST".to_string(), // Ship To
                        "VN".to_string(), // Vendor
                        "SE".to_string(), // Selling Party
                    ]),
                },
                ElementDefinition {
                    element_id: 93,
                    name: "Name".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(60),
                    requirement: ElementRequirement::Optional,
                    description: "Free-form name".to_string(),
                    valid_codes: None,
                },
                ElementDefinition {
                    element_id: 66,
                    name: "Identification Code Qualifier".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(1),
                    max_length: Some(2),
                    requirement: ElementRequirement::Optional,
                    description: "Code designating the system/method of code structure used for Identification Code".to_string(),
                    valid_codes: Some(vec![
                        "1".to_string(),  // D-U-N-S Number
                        "9".to_string(),  // D-U-N-S+4
                        "92".to_string(), // Assigned by Buyer
                    ]),
                },
                ElementDefinition {
                    element_id: 67,
                    name: "Identification Code".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(2),
                    max_length: Some(80),
                    requirement: ElementRequirement::Optional,
                    description: "Code identifying a party or other code".to_string(),
                    valid_codes: None,
                },
            ],
            min_usage: 0,
            max_usage: None, // Can repeat
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
                    element_id: 354,
                    name: "Number of Line Items".to_string(),
                    data_type: ElementDataType::N,
                    min_length: Some(1),
                    max_length: Some(6),
                    requirement: ElementRequirement::Mandatory,
                    description: "Total number of line items in the transaction set".to_string(),
                    valid_codes: None,
                },
                ElementDefinition {
                    element_id: 347,
                    name: "Hash Total".to_string(),
                    data_type: ElementDataType::R,
                    min_length: Some(1),
                    max_length: Some(10),
                    requirement: ElementRequirement::Optional,
                    description: "Sum of values of the specified data element".to_string(),
                    valid_codes: None,
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

    /// Get all element definitions that use a specific element ID
    pub fn get_element_definitions_by_id(&self, element_id: u32) -> Vec<(String, &ElementDefinition)> {
        let mut results = Vec::new();
        
        for ((_, segment_id), definition) in &self.definitions {
            for element in &definition.elements {
                if element.element_id == element_id {
                    results.push((segment_id.clone(), element));
                }
            }
        }
        
        results
    }

    /// Validate an element value against its definition
    pub fn validate_element_value(
        &self,
        version: &X12Version,
        segment_id: &str,
        element_position: usize,
        value: &str,
    ) -> Result<(), EdiError> {
        let definition = self.get_definition(version, segment_id)
            .ok_or_else(|| EdiError::ValidationError(format!("Unknown segment: {}", segment_id)))?;

        let element_def = definition.elements.get(element_position)
            .ok_or_else(|| EdiError::ValidationError(format!("Element position {} not found in segment {}", element_position + 1, segment_id)))?;

        // Check if required element is empty
        if element_def.requirement == ElementRequirement::Mandatory && value.is_empty() {
            return Err(EdiError::ValidationError(format!(
                "Element {} ({}) at position {} is mandatory but empty",
                element_def.element_id, element_def.name, element_position + 1
            )));
        }

        // Skip validation if optional element is empty
        if value.is_empty() {
            return Ok(());
        }

        // Check length constraints
        let value_len = value.len() as u32;
        
        if let Some(min_len) = element_def.min_length {
            if value_len < min_len {
                return Err(EdiError::ValidationError(format!(
                    "Element {} ({}) value '{}' is too short (min: {}, actual: {})",
                    element_def.element_id, element_def.name, value, min_len, value_len
                )));
            }
        }

        if let Some(max_len) = element_def.max_length {
            if value_len > max_len {
                return Err(EdiError::ValidationError(format!(
                    "Element {} ({}) value '{}' is too long (max: {}, actual: {})",
                    element_def.element_id, element_def.name, value, max_len, value_len
                )));
            }
        }

        // Check valid codes if specified
        if let Some(ref valid_codes) = element_def.valid_codes {
            if !valid_codes.contains(&value.to_string()) {
                return Err(EdiError::ValidationError(format!(
                    "Element {} ({}) value '{}' is not valid. Valid values: {:?}",
                    element_def.element_id, element_def.name, value, valid_codes
                )));
            }
        }

        // Validate data type format
        match element_def.data_type {
            ElementDataType::N => {
                if !value.chars().all(|c| c.is_ascii_digit()) {
                    return Err(EdiError::ValidationError(format!(
                        "Element {} ({}) value '{}' must be numeric",
                        element_def.element_id, element_def.name, value
                    )));
                }
            },
            ElementDataType::R => {
                if value.parse::<f64>().is_err() {
                    return Err(EdiError::ValidationError(format!(
                        "Element {} ({}) value '{}' must be a valid decimal number",
                        element_def.element_id, element_def.name, value
                    )));
                }
            },
            ElementDataType::DT => {
                if value.len() != 8 || !value.chars().all(|c| c.is_ascii_digit()) {
                    return Err(EdiError::ValidationError(format!(
                        "Element {} ({}) value '{}' must be a valid date in CCYYMMDD format",
                        element_def.element_id, element_def.name, value
                    )));
                }
            },
            ElementDataType::TM => {
                if ![4, 6, 7, 8].contains(&value.len()) || !value.chars().all(|c| c.is_ascii_digit()) {
                    return Err(EdiError::ValidationError(format!(
                        "Element {} ({}) value '{}' must be a valid time in HHMM, HHMMSS, HHMMSSD, or HHMMSSDD format",
                        element_def.element_id, element_def.name, value
                    )));
                }
            },
            _ => {
                // AN and ID types allow any alphanumeric content within length constraints
            }
        }

        Ok(())
    }
}

impl Default for SegmentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
