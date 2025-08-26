use std::collections::HashMap;
use crate::error::EdiError;
use crate::models::segment_definition::{SegmentDefinition, ElementDataType, X12Version};

/// Comprehensive schema engine for version-aware EDI parsing
/// Supports multiple transaction types across different X12 versions
#[derive(Debug, Clone)]
pub struct SchemaEngine {
    /// Transaction sets organized by type and version
    transaction_sets: HashMap<(String, X12Version), TransactionSetSchema>,
    /// Element dictionary for cross-reference validation
    element_dictionary: HashMap<u32, ElementMasterDefinition>,
    /// Segment definitions organized by version
    segment_registry: HashMap<(X12Version, String), SegmentDefinition>,
}

/// Master element definition with comprehensive metadata
#[derive(Debug, Clone)]
pub struct ElementMasterDefinition {
    pub element_id: u32,
    pub name: String,
    pub description: String,
    pub data_type: ElementDataType,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    /// Version-specific implementations
    pub version_implementations: HashMap<X12Version, ElementVersionInfo>,
}

/// Version-specific element implementation details
#[derive(Debug, Clone)]
pub struct ElementVersionInfo {
    pub valid_codes: Option<Vec<String>>,
    pub notes: Option<String>,
    pub usage_rules: Vec<String>,
}

/// Complete transaction set schema with version awareness
#[derive(Debug, Clone)]
pub struct TransactionSetSchema {
    pub transaction_type: String,
    pub version: X12Version,
    pub name: String,
    pub description: String,
    pub segments: Vec<SegmentSchema>,
    pub trading_partner_agreements: Vec<TradingPartnerAgreement>,
}

/// Segment schema with positional and hierarchical context
#[derive(Debug, Clone)]
pub struct SegmentSchema {
    pub segment_id: String,
    pub name: String,
    pub requirement: SegmentRequirement,
    pub max_use: Option<u32>,
    /// Hierarchical level (0 = header, 1 = detail, 2 = summary)
    pub hierarchy_level: u32,
    /// Dependencies on other segments
    pub dependencies: Vec<SegmentDependency>,
    /// Context-specific validation rules
    pub context_rules: Vec<ContextRule>,
}

/// Trading partner specific agreements and customizations
#[derive(Debug, Clone)]
pub struct TradingPartnerAgreement {
    pub partner_id: String,
    pub agreement_name: String,
    pub customizations: Vec<SchemaCustomization>,
}

/// Schema customizations for specific trading partners
#[derive(Debug, Clone)]
pub struct SchemaCustomization {
    pub segment_id: String,
    pub element_position: Option<usize>,
    pub customization_type: CustomizationType,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum CustomizationType {
    /// Make optional element mandatory
    MakeMandatory,
    /// Make mandatory element optional
    MakeOptional,
    /// Add additional valid codes
    ExtendValidCodes(Vec<String>),
    /// Restrict valid codes
    RestrictValidCodes(Vec<String>),
    /// Change length constraints
    ChangeLengthConstraints { min: Option<u32>, max: Option<u32> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SegmentRequirement {
    Mandatory,
    Optional,
    Conditional(String), // Condition description
}

#[derive(Debug, Clone)]
pub struct SegmentDependency {
    pub depends_on: String,
    pub dependency_type: DependencyType,
    pub condition: Option<String>,
}

#[derive(Debug, Clone)]
pub enum DependencyType {
    /// Must appear after the specified segment
    MustFollow,
    /// Must appear before the specified segment
    MustPrecede,
    /// Cannot appear with the specified segment
    MutuallyExclusive,
    /// Required if the specified segment is present
    RequiredIf,
}

#[derive(Debug, Clone)]
pub struct ContextRule {
    pub rule_id: String,
    pub description: String,
    pub condition: String,
    pub action: ValidationAction,
}

#[derive(Debug, Clone)]
pub enum ValidationAction {
    Error(String),
    Warning(String),
    Info(String),
    Transform(String), // Transformation rule
}

impl SchemaEngine {
    /// Create a new schema engine with comprehensive EDI standards
    pub fn new() -> Self {
        let mut engine = SchemaEngine {
            transaction_sets: HashMap::new(),
            element_dictionary: HashMap::new(),
            segment_registry: HashMap::new(),
        };
        
        engine.initialize_standard_schemas();
        engine
    }

    /// Initialize all standard X12 transaction sets and versions
    fn initialize_standard_schemas(&mut self) {
        // Purchase Order (850)
        self.register_850_schemas();
        // Invoice (810)
        self.register_810_schemas();
        // Functional Acknowledgment (997)
        self.register_997_schemas();
        // Advance Ship Notice (856)
        self.register_856_schemas();
        // Purchase Order Change (860)
        self.register_860_schemas();

        // Initialize element dictionary
        self.initialize_element_dictionary();
    }

    /// Register 850 Purchase Order schemas for all versions
    fn register_850_schemas(&mut self) {
        for version in [X12Version::V4010, X12Version::V5010, X12Version::V8010] {
            let schema = self.create_850_schema(version);
            self.transaction_sets.insert(("850".to_string(), version), schema);
        }
    }

    /// Register 810 Invoice schemas for all versions
    fn register_810_schemas(&mut self) {
        for version in [X12Version::V4010, X12Version::V5010, X12Version::V8010] {
            let schema = self.create_810_schema(version);
            self.transaction_sets.insert(("810".to_string(), version), schema);
        }
    }

    /// Register 997 Functional Acknowledgment schemas
    fn register_997_schemas(&mut self) {
        for version in [X12Version::V4010, X12Version::V5010, X12Version::V8010] {
            let schema = self.create_997_schema(version);
            self.transaction_sets.insert(("997".to_string(), version), schema);
        }
    }

    /// Register 856 Advance Ship Notice schemas
    fn register_856_schemas(&mut self) {
        for version in [X12Version::V4010, X12Version::V5010, X12Version::V8010] {
            let schema = self.create_856_schema(version);
            self.transaction_sets.insert(("856".to_string(), version), schema);
        }
    }

    /// Register 860 Purchase Order Change schemas
    fn register_860_schemas(&mut self) {
        for version in [X12Version::V4010, X12Version::V5010, X12Version::V8010] {
            let schema = self.create_860_schema(version);
            self.transaction_sets.insert(("860".to_string(), version), schema);
        }
    }

    /// Create comprehensive 850 Purchase Order schema
    fn create_850_schema(&self, version: X12Version) -> TransactionSetSchema {
        TransactionSetSchema {
            transaction_type: "850".to_string(),
            version,
            name: "Purchase Order".to_string(),
            description: "This X12 Transaction Set contains the format and establishes the data contents of the Purchase Order Transaction Set (850) for use within the context of an Electronic Data Interchange (EDI) environment.".to_string(),
            segments: vec![
                // Header Level
                SegmentSchema {
                    segment_id: "ST".to_string(),
                    name: "Transaction Set Header".to_string(),
                    requirement: SegmentRequirement::Mandatory,
                    max_use: Some(1),
                    hierarchy_level: 0,
                    dependencies: vec![],
                    context_rules: vec![],
                },
                SegmentSchema {
                    segment_id: "BEG".to_string(),
                    name: "Beginning Segment for Purchase Order".to_string(),
                    requirement: SegmentRequirement::Mandatory,
                    max_use: Some(1),
                    hierarchy_level: 0,
                    dependencies: vec![
                        SegmentDependency {
                            depends_on: "ST".to_string(),
                            dependency_type: DependencyType::MustFollow,
                            condition: None,
                        }
                    ],
                    context_rules: vec![],
                },
                SegmentSchema {
                    segment_id: "CUR".to_string(),
                    name: "Currency".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: Some(1),
                    hierarchy_level: 0,
                    dependencies: vec![],
                    context_rules: vec![
                        ContextRule {
                            rule_id: "CUR_01_VALIDATION".to_string(),
                            description: "Currency code must be valid ISO 4217".to_string(),
                            condition: "CUR01 is present".to_string(),
                            action: ValidationAction::Error("Invalid currency code".to_string()),
                        }
                    ],
                },
                SegmentSchema {
                    segment_id: "REF".to_string(),
                    name: "Reference Identification".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: None, // Unlimited
                    hierarchy_level: 0,
                    dependencies: vec![],
                    context_rules: vec![],
                },
                SegmentSchema {
                    segment_id: "PER".to_string(),
                    name: "Administrative Communications Contact".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: Some(3),
                    hierarchy_level: 0,
                    dependencies: vec![],
                    context_rules: vec![],
                },
                SegmentSchema {
                    segment_id: "DTM".to_string(),
                    name: "Date/Time Reference".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: Some(10),
                    hierarchy_level: 0,
                    dependencies: vec![],
                    context_rules: vec![],
                },
                // Detail Level - N1 Loop
                SegmentSchema {
                    segment_id: "N1".to_string(),
                    name: "Name".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: Some(200),
                    hierarchy_level: 1,
                    dependencies: vec![],
                    context_rules: vec![],
                },
                SegmentSchema {
                    segment_id: "N2".to_string(),
                    name: "Additional Name Information".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: Some(2),
                    hierarchy_level: 1,
                    dependencies: vec![
                        SegmentDependency {
                            depends_on: "N1".to_string(),
                            dependency_type: DependencyType::RequiredIf,
                            condition: Some("N1 segment is present".to_string()),
                        }
                    ],
                    context_rules: vec![],
                },
                SegmentSchema {
                    segment_id: "N3".to_string(),
                    name: "Address Information".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: Some(2),
                    hierarchy_level: 1,
                    dependencies: vec![
                        SegmentDependency {
                            depends_on: "N1".to_string(),
                            dependency_type: DependencyType::RequiredIf,
                            condition: Some("N1 segment is present".to_string()),
                        }
                    ],
                    context_rules: vec![],
                },
                SegmentSchema {
                    segment_id: "N4".to_string(),
                    name: "Geographic Location".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: Some(1),
                    hierarchy_level: 1,
                    dependencies: vec![
                        SegmentDependency {
                            depends_on: "N1".to_string(),
                            dependency_type: DependencyType::RequiredIf,
                            condition: Some("N1 segment is present".to_string()),
                        }
                    ],
                    context_rules: vec![],
                },
                // Detail Level - PO1 Loop
                SegmentSchema {
                    segment_id: "PO1".to_string(),
                    name: "Baseline Item Data".to_string(),
                    requirement: SegmentRequirement::Mandatory,
                    max_use: Some(100000),
                    hierarchy_level: 1,
                    dependencies: vec![],
                    context_rules: vec![
                        ContextRule {
                            rule_id: "PO1_QUANTITY_PRICE".to_string(),
                            description: "Either quantity or price must be specified".to_string(),
                            condition: "PO102 or PO104 must be present".to_string(),
                            action: ValidationAction::Error("Missing quantity or price".to_string()),
                        }
                    ],
                },
                SegmentSchema {
                    segment_id: "CUR".to_string(),
                    name: "Currency".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: Some(1),
                    hierarchy_level: 1,
                    dependencies: vec![
                        SegmentDependency {
                            depends_on: "PO1".to_string(),
                            dependency_type: DependencyType::RequiredIf,
                            condition: Some("PO1 segment is present".to_string()),
                        }
                    ],
                    context_rules: vec![],
                },
                SegmentSchema {
                    segment_id: "PID".to_string(),
                    name: "Product/Item Description".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: Some(1000),
                    hierarchy_level: 1,
                    dependencies: vec![
                        SegmentDependency {
                            depends_on: "PO1".to_string(),
                            dependency_type: DependencyType::RequiredIf,
                            condition: Some("PO1 segment is present".to_string()),
                        }
                    ],
                    context_rules: vec![],
                },
                // Summary Level
                SegmentSchema {
                    segment_id: "CTT".to_string(),
                    name: "Transaction Totals".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: Some(1),
                    hierarchy_level: 2,
                    dependencies: vec![],
                    context_rules: vec![
                        ContextRule {
                            rule_id: "CTT_LINE_COUNT".to_string(),
                            description: "Line count must match actual PO1 segments".to_string(),
                            condition: "CTT01 is present".to_string(),
                            action: ValidationAction::Error("Line count mismatch".to_string()),
                        }
                    ],
                },
                SegmentSchema {
                    segment_id: "SE".to_string(),
                    name: "Transaction Set Trailer".to_string(),
                    requirement: SegmentRequirement::Mandatory,
                    max_use: Some(1),
                    hierarchy_level: 2,
                    dependencies: vec![],
                    context_rules: vec![],
                },
            ],
            trading_partner_agreements: vec![],
        }
    }

    /// Create 810 Invoice schema (placeholder for now)
    fn create_810_schema(&self, version: X12Version) -> TransactionSetSchema {
        TransactionSetSchema {
            transaction_type: "810".to_string(),
            version,
            name: "Invoice".to_string(),
            description: "Invoice transaction set for billing information".to_string(),
            segments: vec![], // TODO: Implement full 810 schema
            trading_partner_agreements: vec![],
        }
    }

    /// Create 997 Functional Acknowledgment schema (placeholder for now)
    fn create_997_schema(&self, version: X12Version) -> TransactionSetSchema {
        TransactionSetSchema {
            transaction_type: "997".to_string(),
            version,
            name: "Functional Acknowledgment".to_string(),
            description: "Functional acknowledgment for EDI transactions".to_string(),
            segments: vec![], // TODO: Implement full 997 schema
            trading_partner_agreements: vec![],
        }
    }

    /// Create 856 Advance Ship Notice schema (placeholder for now)
    fn create_856_schema(&self, version: X12Version) -> TransactionSetSchema {
        TransactionSetSchema {
            transaction_type: "856".to_string(),
            version,
            name: "Advance Ship Notice".to_string(),
            description: "Advance ship notice for shipment information".to_string(),
            segments: vec![], // TODO: Implement full 856 schema
            trading_partner_agreements: vec![],
        }
    }

    /// Create 860 Purchase Order Change schema (placeholder for now)
    fn create_860_schema(&self, version: X12Version) -> TransactionSetSchema {
        TransactionSetSchema {
            transaction_type: "860".to_string(),
            version,
            name: "Purchase Order Change Request - Buyer Initiated".to_string(),
            description: "Purchase order change request initiated by buyer".to_string(),
            segments: vec![], // TODO: Implement full 860 schema
            trading_partner_agreements: vec![],
        }
    }

    /// Initialize comprehensive element dictionary
    fn initialize_element_dictionary(&mut self) {
        // Element 98 - Entity Identifier Code (used in CUR01)
        self.element_dictionary.insert(98, ElementMasterDefinition {
            element_id: 98,
            name: "Entity Identifier Code".to_string(),
            description: "Code identifying an organizational entity, a physical location, property or an individual".to_string(),
            data_type: ElementDataType::ID,
            min_length: Some(2),
            max_length: Some(3),
            version_implementations: HashMap::from([
                (X12Version::V4010, ElementVersionInfo {
                    valid_codes: Some(vec![
                        "BY".to_string(), // Buying Party (Purchaser)
                        "SE".to_string(), // Selling Party
                        "BT".to_string(), // Bill-to-Party
                        "ST".to_string(), // Ship-to-Party
                        "SU".to_string(), // Supplier/Manufacturer
                    ]),
                    notes: Some("Version 4010 implementation".to_string()),
                    usage_rules: vec!["Must be present in CUR segment".to_string()],
                }),
                (X12Version::V5010, ElementVersionInfo {
                    valid_codes: Some(vec![
                        "BY".to_string(), // Buying Party (Purchaser)
                        "SE".to_string(), // Selling Party
                        "BT".to_string(), // Bill-to-Party
                        "ST".to_string(), // Ship-to-Party
                        "SU".to_string(), // Supplier/Manufacturer
                        "VN".to_string(), // Vendor
                    ]),
                    notes: Some("Version 5010 implementation with expanded codes".to_string()),
                    usage_rules: vec!["Must be present in CUR segment".to_string()],
                }),
                (X12Version::V8010, ElementVersionInfo {
                    valid_codes: Some(vec![
                        "BY".to_string(), // Buying Party (Purchaser)
                        "SE".to_string(), // Selling Party
                        "BT".to_string(), // Bill-to-Party
                        "ST".to_string(), // Ship-to-Party
                        "SU".to_string(), // Supplier/Manufacturer
                        "VN".to_string(), // Vendor
                        "3P".to_string(), // Third Party
                    ]),
                    notes: Some("Version 8010 implementation with additional codes".to_string()),
                    usage_rules: vec!["Must be present in CUR segment".to_string()],
                }),
            ]),
        });

        // Element 100 - Currency Code
        self.element_dictionary.insert(100, ElementMasterDefinition {
            element_id: 100,
            name: "Currency Code".to_string(),
            description: "Code (Standard ISO) for country in whose currency the charges are specified".to_string(),
            data_type: ElementDataType::ID,
            min_length: Some(3),
            max_length: Some(3),
            version_implementations: HashMap::from([
                (X12Version::V4010, ElementVersionInfo {
                    valid_codes: Some(vec![
                        "USD".to_string(), // US Dollar
                        "CAD".to_string(), // Canadian Dollar
                        "EUR".to_string(), // Euro
                        "GBP".to_string(), // British Pound
                        "JPY".to_string(), // Japanese Yen
                    ]),
                    notes: Some("ISO 4217 currency codes".to_string()),
                    usage_rules: vec!["Must be valid ISO 4217 code".to_string()],
                }),
                (X12Version::V5010, ElementVersionInfo {
                    valid_codes: None, // Uses full ISO 4217 list
                    notes: Some("Full ISO 4217 support".to_string()),
                    usage_rules: vec!["Must be valid ISO 4217 code".to_string()],
                }),
                (X12Version::V8010, ElementVersionInfo {
                    valid_codes: None, // Uses full ISO 4217 list
                    notes: Some("Full ISO 4217 support with digital currencies".to_string()),
                    usage_rules: vec!["Must be valid ISO 4217 code or approved digital currency code".to_string()],
                }),
            ]),
        });

        // Add more elements as needed...
    }

    /// Get transaction set schema by type and version
    pub fn get_transaction_schema(&self, transaction_type: &str, version: &X12Version) -> Option<&TransactionSetSchema> {
        self.transaction_sets.get(&(transaction_type.to_string(), *version))
    }

    /// Get element definition by ID
    pub fn get_element_definition(&self, element_id: u32) -> Option<&ElementMasterDefinition> {
        self.element_dictionary.get(&element_id)
    }

    /// Validate transaction structure against schema
    pub fn validate_transaction_structure(
        &self,
        transaction_type: &str,
        version: &X12Version,
        segments: &[String],
    ) -> Result<Vec<ValidationResult>, EdiError> {
        let schema = self.get_transaction_schema(transaction_type, version)
            .ok_or_else(|| EdiError::ValidationError(format!("Unknown transaction type: {} version: {:?}", transaction_type, version)))?;

        let mut results = Vec::new();
        let mut current_hierarchy = 0;
        let mut segment_counts = HashMap::new();

        for (position, segment_id) in segments.iter().enumerate() {
            // Find segment schema
            if let Some(segment_schema) = schema.segments.iter().find(|s| &s.segment_id == segment_id) {
                // Update counts
                *segment_counts.entry(segment_id.clone()).or_insert(0) += 1;

                // Check max usage
                if let Some(max_use) = segment_schema.max_use {
                    if segment_counts[segment_id] > max_use {
                        results.push(ValidationResult {
                            result_type: ValidationResultType::Error,
                            message: format!("Segment {} exceeds maximum usage of {} at position {}", segment_id, max_use, position + 1),
                            segment_position: Some(position),
                            element_position: None,
                        });
                    }
                }

                // Check hierarchy level progression
                if segment_schema.hierarchy_level < current_hierarchy {
                    results.push(ValidationResult {
                        result_type: ValidationResultType::Warning,
                        message: format!("Segment {} appears to move backwards in hierarchy at position {}", segment_id, position + 1),
                        segment_position: Some(position),
                        element_position: None,
                    });
                }
                current_hierarchy = segment_schema.hierarchy_level;

                // Check dependencies
                for dependency in &segment_schema.dependencies {
                    match dependency.dependency_type {
                        DependencyType::MustFollow => {
                            // Check if required segment appeared before this one
                            let found_dependency = segments[..position].iter().any(|s| s == &dependency.depends_on);
                            if !found_dependency {
                                results.push(ValidationResult {
                                    result_type: ValidationResultType::Error,
                                    message: format!("Segment {} requires {} to appear before it", segment_id, dependency.depends_on),
                                    segment_position: Some(position),
                                    element_position: None,
                                });
                            }
                        },
                        DependencyType::RequiredIf => {
                            // This check would need more context about loop structures
                        },
                        _ => {} // Other dependency types would be implemented similarly
                    }
                }
            } else {
                results.push(ValidationResult {
                    result_type: ValidationResultType::Warning,
                    message: format!("Unknown segment {} at position {}", segment_id, position + 1),
                    segment_position: Some(position),
                    element_position: None,
                });
            }
        }

        // Check for missing mandatory segments
        for segment_schema in &schema.segments {
            if segment_schema.requirement == SegmentRequirement::Mandatory {
                if !segments.contains(&segment_schema.segment_id) {
                    results.push(ValidationResult {
                        result_type: ValidationResultType::Error,
                        message: format!("Missing mandatory segment {}", segment_schema.segment_id),
                        segment_position: None,
                        element_position: None,
                    });
                }
            }
        }

        Ok(results)
    }

    /// Add trading partner agreement
    pub fn add_trading_partner_agreement(
        &mut self,
        transaction_type: &str,
        version: X12Version,
        agreement: TradingPartnerAgreement,
    ) -> Result<(), EdiError> {
        if let Some(schema) = self.transaction_sets.get_mut(&(transaction_type.to_string(), version)) {
            schema.trading_partner_agreements.push(agreement);
            Ok(())
        } else {
            Err(EdiError::ValidationError(format!("Transaction schema not found: {} {:?}", transaction_type, version)))
        }
    }

    /// Get available transaction types for a version
    pub fn get_available_transaction_types(&self, version: &X12Version) -> Vec<String> {
        self.transaction_sets
            .keys()
            .filter(|(_, v)| v == version)
            .map(|(t, _)| t.clone())
            .collect()
    }

    /// Get supported versions for a transaction type
    pub fn get_supported_versions(&self, transaction_type: &str) -> Vec<X12Version> {
        self.transaction_sets
            .keys()
            .filter(|(t, _)| t == transaction_type)
            .map(|(_, v)| *v)
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub result_type: ValidationResultType,
    pub message: String,
    pub segment_position: Option<usize>,
    pub element_position: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResultType {
    Error,
    Warning,
    Info,
}

impl Default for SchemaEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_engine_creation() {
        let engine = SchemaEngine::new();
        
        // Test that 850 schema is available for all versions
        assert!(engine.get_transaction_schema("850", &X12Version::V4010).is_some());
        assert!(engine.get_transaction_schema("850", &X12Version::V5010).is_some());
        assert!(engine.get_transaction_schema("850", &X12Version::V8010).is_some());
    }

    #[test]
    fn test_element_dictionary() {
        let engine = SchemaEngine::new();
        
        // Test element 98 (Entity Identifier Code)
        let element = engine.get_element_definition(98).unwrap();
        assert_eq!(element.element_id, 98);
        assert_eq!(element.name, "Entity Identifier Code");
        
        // Test version-specific implementations
        assert!(element.version_implementations.contains_key(&X12Version::V4010));
        assert!(element.version_implementations.contains_key(&X12Version::V5010));
        assert!(element.version_implementations.contains_key(&X12Version::V8010));
    }

    #[test]
    fn test_transaction_structure_validation() {
        let engine = SchemaEngine::new();
        
        let segments = vec![
            "ST".to_string(),
            "BEG".to_string(),
            "PO1".to_string(),
            "SE".to_string(),
        ];
        
        let results = engine.validate_transaction_structure("850", &X12Version::V4010, &segments).unwrap();
        
        // Should pass basic validation
        let error_count = results.iter().filter(|r| r.result_type == ValidationResultType::Error).count();
        assert_eq!(error_count, 0);
    }
}
