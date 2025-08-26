# EDI Parser - Schema Extension Guide

This guide explains how to extend the EDI Parser with additional segments, document types, and partner-specific configurations.

## Table of Contents

- [Adding New Segments](#adding-new-segments)
- [Adding New Document Types](#adding-new-document-types)
- [Version-Specific Configurations](#version-specific-configurations)
- [Partner-Specific Customizations](#partner-specific-customizations)
- [Element Dictionary Management](#element-dictionary-management)
- [Best Practices](#best-practices)
- [Examples](#examples)

## Adding New Segments

### Step 1: Define the Segment in SegmentRegistry

Add your new segment definition to the `SegmentRegistry` in `src/models/segment_definition.rs`:

```rust
impl SegmentRegistry {
    // Add a new method to register your segment
    fn register_tax_segment(&mut self) {
        let tax_definition = SegmentDefinition {
            id: "TAX".to_string(),
            name: "Tax Reference".to_string(),
            elements: vec![
                ElementDefinition {
                    element_id: 963,  // Tax Type Code
                    name: "Tax Type Code".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(3),
                    requirement: ElementRequirement::Mandatory,
                    description: "Code identifying the type of tax".to_string(),
                    valid_codes: Some(vec![
                        "ST".to_string(), // State Tax
                        "LT".to_string(), // Local Tax
                        "FT".to_string(), // Federal Tax
                        "GS".to_string(), // Goods and Services Tax
                        "VA".to_string(), // Value Added Tax
                    ]),
                },
                ElementDefinition {
                    element_id: 782,  // Monetary Amount
                    name: "Tax Amount".to_string(),
                    data_type: ElementDataType::R,
                    min_length: Some(1),
                    max_length: Some(18),
                    requirement: ElementRequirement::Mandatory,
                    description: "Monetary amount of the tax".to_string(),
                    valid_codes: None,
                },
                ElementDefinition {
                    element_id: 954,  // Percent
                    name: "Tax Rate".to_string(),
                    data_type: ElementDataType::R,
                    min_length: Some(1),
                    max_length: Some(10),
                    requirement: ElementRequirement::Optional,
                    description: "Tax rate expressed as a percentage".to_string(),
                    valid_codes: None,
                },
            ],
            min_usage: 0,
            max_usage: Some(10), // Can repeat up to 10 times
            description: "To specify tax information".to_string(),
        };

        // Register for all supported versions
        self.register_segment(X12Version::V4010, tax_definition.clone());
        self.register_segment(X12Version::V5010, tax_definition.clone());
        self.register_segment(X12Version::V8010, tax_definition);
    }
}
```

### Step 2: Call the Registration Method

Add your segment registration to the `new()` method:

```rust
impl SegmentRegistry {
    pub fn new() -> Self {
        let mut registry = SegmentRegistry {
            definitions: HashMap::new(),
        };

        // Existing registrations
        registry.register_beg_segment();
        registry.register_po1_segment();
        registry.register_ref_segment();
        registry.register_cur_segment();
        
        // Add your new segment
        registry.register_tax_segment();

        registry
    }
}
```

### Step 3: Add Version-Specific Differences (if needed)

If your segment differs between versions, create separate definitions:

```rust
fn register_tax_segment(&mut self) {
    // Version 4010 - Basic tax segment
    let tax_v4010 = SegmentDefinition {
        id: "TAX".to_string(),
        name: "Tax Reference".to_string(),
        elements: vec![
            // Basic elements only
            ElementDefinition {
                element_id: 963,
                name: "Tax Type Code".to_string(),
                // ... basic definition
            },
        ],
        // ... rest of definition
    };

    // Version 5010 - Enhanced with additional elements
    let tax_v5010 = SegmentDefinition {
        id: "TAX".to_string(),
        name: "Tax Reference".to_string(),
        elements: vec![
            // All v4010 elements plus new ones
            ElementDefinition {
                element_id: 963,
                name: "Tax Type Code".to_string(),
                // ... enhanced definition with more valid codes
            },
            ElementDefinition {
                element_id: 1271,  // New in v5010
                name: "Tax Jurisdiction Code".to_string(),
                // ... new element definition
            },
        ],
        // ... rest of definition
    };

    self.register_segment(X12Version::V4010, tax_v4010);
    self.register_segment(X12Version::V5010, tax_v5010);
    self.register_segment(X12Version::V8010, tax_v5010.clone()); // Same as 5010
}
```

## Adding New Document Types

### Step 1: Create the Transaction Schema

Add your transaction schema to the `SchemaEngine` in `src/models/schema_engine.rs`:

```rust
impl SchemaEngine {
    // Add registration method for your new transaction type
    fn register_820_schemas(&mut self) {
        for version in [X12Version::V4010, X12Version::V5010, X12Version::V8010] {
            let schema = self.create_820_schema(version);
            self.transaction_sets.insert(("820".to_string(), version), schema);
        }
    }

    // Create the specific schema for Payment Order/Remittance Advice (820)
    fn create_820_schema(&self, version: X12Version) -> TransactionSetSchema {
        TransactionSetSchema {
            transaction_type: "820".to_string(),
            version,
            name: "Payment Order/Remittance Advice".to_string(),
            description: "This transaction set can be used to make a payment, send a remittance advice, or make a payment and send a remittance advice.".to_string(),
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
                    segment_id: "BPR".to_string(),
                    name: "Beginning Segment for Payment Order/Remittance".to_string(),
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
                    context_rules: vec![
                        ContextRule {
                            rule_id: "BPR_PAYMENT_VALIDATION".to_string(),
                            description: "Payment amount must be positive".to_string(),
                            condition: "BPR02 > 0".to_string(),
                            action: ValidationAction::Error("Payment amount must be greater than zero".to_string()),
                        }
                    ],
                },
                SegmentSchema {
                    segment_id: "TRN".to_string(),
                    name: "Trace".to_string(),
                    requirement: SegmentRequirement::Mandatory,
                    max_use: Some(1),
                    hierarchy_level: 0,
                    dependencies: vec![],
                    context_rules: vec![],
                },
                SegmentSchema {
                    segment_id: "CUR".to_string(),
                    name: "Currency".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: Some(1),
                    hierarchy_level: 0,
                    dependencies: vec![],
                    context_rules: vec![],
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
                    segment_id: "DTM".to_string(),
                    name: "Date/Time Reference".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: Some(10),
                    hierarchy_level: 0,
                    dependencies: vec![],
                    context_rules: vec![],
                },
                // Detail Level - N1 Loop (Payer/Payee Information)
                SegmentSchema {
                    segment_id: "N1".to_string(),
                    name: "Name".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: Some(10),
                    hierarchy_level: 1,
                    dependencies: vec![],
                    context_rules: vec![],
                },
                // Detail Level - RMR Loop (Remittance Detail)
                SegmentSchema {
                    segment_id: "RMR".to_string(),
                    name: "Remittance Advice".to_string(),
                    requirement: SegmentRequirement::Optional,
                    max_use: Some(999999),
                    hierarchy_level: 1,
                    dependencies: vec![],
                    context_rules: vec![],
                },
                // Add TAX segment if supported in this version
                if version != X12Version::V4010 {
                    SegmentSchema {
                        segment_id: "TAX".to_string(),
                        name: "Tax Reference".to_string(),
                        requirement: SegmentRequirement::Optional,
                        max_use: Some(10),
                        hierarchy_level: 1,
                        dependencies: vec![
                            SegmentDependency {
                                depends_on: "RMR".to_string(),
                                dependency_type: DependencyType::RequiredIf,
                                condition: Some("RMR segment is present".to_string()),
                            }
                        ],
                        context_rules: vec![],
                    }
                } else {
                    // Skip TAX for v4010 as it might not be supported
                },
                // Summary Level
                SegmentSchema {
                    segment_id: "SE".to_string(),
                    name: "Transaction Set Trailer".to_string(),
                    requirement: SegmentRequirement::Mandatory,
                    max_use: Some(1),
                    hierarchy_level: 2,
                    dependencies: vec![],
                    context_rules: vec![],
                },
            ].into_iter().filter(|_| true).collect(), // Filter out None values
            trading_partner_agreements: vec![],
        }
    }
}
```

### Step 2: Register the New Transaction Type

Add the registration call to the initialization:

```rust
impl SchemaEngine {
    fn initialize_standard_schemas(&mut self) {
        // Existing transaction types
        self.register_850_schemas();
        self.register_810_schemas();
        self.register_997_schemas();
        self.register_856_schemas();
        self.register_860_schemas();
        
        // Add your new transaction type
        self.register_820_schemas();

        self.initialize_element_dictionary();
    }
}
```

## Version-Specific Configurations

### Handling Version Differences

Different X12 versions may have:
- Different element definitions
- Additional or removed segments
- Changed validation rules
- Different valid code lists

Example of version-specific handling:

```rust
fn create_segment_with_version_differences(&self, version: X12Version) -> SegmentSchema {
    let mut segment = SegmentSchema {
        segment_id: "ADV".to_string(),
        name: "Advance Information".to_string(),
        requirement: SegmentRequirement::Optional,
        max_use: Some(1),
        hierarchy_level: 1,
        dependencies: vec![],
        context_rules: vec![],
    };

    // Version-specific modifications
    match version {
        X12Version::V4010 => {
            // V4010 has basic validation
            segment.context_rules.push(ContextRule {
                rule_id: "ADV_BASIC_VALIDATION".to_string(),
                description: "Basic advance validation".to_string(),
                condition: "ADV01 is present".to_string(),
                action: ValidationAction::Warning("Consider upgrading to newer version for enhanced validation".to_string()),
            });
        },
        X12Version::V5010 => {
            // V5010 has enhanced validation
            segment.context_rules.push(ContextRule {
                rule_id: "ADV_ENHANCED_VALIDATION".to_string(),
                description: "Enhanced advance validation".to_string(),
                condition: "ADV01 and ADV02 are present".to_string(),
                action: ValidationAction::Error("Both advance code and amount are required".to_string()),
            });
        },
        X12Version::V8010 => {
            // V8010 has strict validation
            segment.context_rules.push(ContextRule {
                rule_id: "ADV_STRICT_VALIDATION".to_string(),
                description: "Strict advance validation with digital compliance".to_string(),
                condition: "ADV01, ADV02, and ADV03 are present and comply with digital standards".to_string(),
                action: ValidationAction::Error("All advance fields required for digital compliance".to_string()),
            });
        },
    }

    segment
}
```

## Partner-Specific Customizations

### Step 1: Define Partner Agreements

Create trading partner agreements for specific customizations:

```rust
// Example: Create a Walmart-specific agreement for 850 Purchase Orders
let walmart_agreement = TradingPartnerAgreement {
    partner_id: "WALMART_US".to_string(),
    agreement_name: "Walmart US EDI Agreement v2.1".to_string(),
    customizations: vec![
        // Make REF segment mandatory for Walmart
        SchemaCustomization {
            segment_id: "REF".to_string(),
            element_position: None, // Applies to entire segment
            customization_type: CustomizationType::MakeMandatory,
            description: "Walmart requires REF segment with vendor number".to_string(),
        },
        // Require specific REF qualifier codes
        SchemaCustomization {
            segment_id: "REF".to_string(),
            element_position: Some(0), // REF01
            customization_type: CustomizationType::RestrictValidCodes(vec![
                "VN".to_string(), // Vendor Number (required)
                "DP".to_string(), // Department Number (optional)
            ]),
            description: "Walmart only accepts VN and DP reference qualifiers".to_string(),
        },
        // Extend valid UOM codes for PO1 segment
        SchemaCustomization {
            segment_id: "PO1".to_string(),
            element_position: Some(2), // PO103 - Unit of Measure
            customization_type: CustomizationType::ExtendValidCodes(vec![
                "PL".to_string(), // Pallet (Walmart-specific)
                "CA".to_string(), // Case
                "EA".to_string(), // Each
            ]),
            description: "Walmart accepts additional UOM codes including pallets".to_string(),
        },
        // Change length constraints for item description
        SchemaCustomization {
            segment_id: "PID".to_string(),
            element_position: Some(4), // PID05 - Description
            customization_type: CustomizationType::ChangeLengthConstraints { 
                min: Some(10), 
                max: Some(80) 
            },
            description: "Walmart requires detailed product descriptions (10-80 chars)".to_string(),
        },
    ],
};
```

### Step 2: Add Partner Agreement to Schema

```rust
// Add the partner agreement to a specific transaction schema
let mut schema_engine = SchemaEngine::new();
schema_engine.add_trading_partner_agreement(
    "850", 
    X12Version::V4010, 
    walmart_agreement
)?;
```

### Step 3: Apply Partner-Specific Validation

```rust
// When parsing with partner-specific rules
let config = ParsingConfig {
    version: X12Version::V4010,
    trading_partner_id: Some("WALMART_US".to_string()),
    validation_level: ValidationLevel::Strict,
    continue_on_error: false,
    collect_validation_details: true,
};

let mut parser = SchemaValidatingParser::new(config);
let result = parser.parse_with_schema(edi_data)?;
```

### Step 4: Advanced Partner Customizations

Create complex partner-specific business rules:

```rust
// Example: Target-specific inventory validation
let target_agreement = TradingPartnerAgreement {
    partner_id: "TARGET_CORP".to_string(),
    agreement_name: "Target Corporation EDI Standards".to_string(),
    customizations: vec![
        SchemaCustomization {
            segment_id: "PO1".to_string(),
            element_position: None,
            customization_type: CustomizationType::MakeMandatory,
            description: "Target requires all PO1 segments to have vendor item numbers".to_string(),
        },
    ],
};

// Advanced validation function for Target
fn validate_target_specific_rules(segment: &ValidatedSegment) -> Vec<ValidationResult> {
    let mut results = Vec::new();
    
    if segment.segment.id == "PO1" {
        // Target requires specific item number format: TGT-XXXXXXXX
        if let Some(vendor_item) = segment.segment.elements.get(6) { // PO107
            if !vendor_item.starts_with("TGT-") {
                results.push(ValidationResult {
                    result_type: ValidationResultType::Error,
                    message: "Target requires vendor item numbers to start with 'TGT-'".to_string(),
                    segment_position: None,
                    element_position: Some(6),
                });
            }
        }
        
        // Target requires minimum order quantities
        if let Some(quantity_str) = segment.segment.elements.get(1) { // PO102
            if let Ok(quantity) = quantity_str.parse::<f64>() {
                if quantity < 12.0 {
                    results.push(ValidationResult {
                        result_type: ValidationResultType::Warning,
                        message: "Target prefers minimum order quantities of 12 or more".to_string(),
                        segment_position: None,
                        element_position: Some(1),
                    });
                }
            }
        }
    }
    
    results
}
```

## Element Dictionary Management

### Adding New Elements

Add new elements to the element dictionary for cross-referencing:

```rust
impl SchemaEngine {
    fn initialize_element_dictionary(&mut self) {
        // Existing elements...
        
        // Add new tax-related elements
        self.element_dictionary.insert(963, ElementMasterDefinition {
            element_id: 963,
            name: "Tax Type Code".to_string(),
            description: "Code identifying the type of tax".to_string(),
            data_type: ElementDataType::ID,
            min_length: Some(2),
            max_length: Some(3),
            version_implementations: HashMap::from([
                (X12Version::V4010, ElementVersionInfo {
                    valid_codes: Some(vec![
                        "ST".to_string(), // State Tax
                        "LT".to_string(), // Local Tax
                        "FT".to_string(), // Federal Tax
                    ]),
                    notes: Some("Basic tax types for v4010".to_string()),
                    usage_rules: vec!["Required in TAX segment".to_string()],
                }),
                (X12Version::V5010, ElementVersionInfo {
                    valid_codes: Some(vec![
                        "ST".to_string(), // State Tax
                        "LT".to_string(), // Local Tax
                        "FT".to_string(), // Federal Tax
                        "GS".to_string(), // Goods and Services Tax
                        "VA".to_string(), // Value Added Tax
                    ]),
                    notes: Some("Enhanced tax types for v5010".to_string()),
                    usage_rules: vec!["Required in TAX segment".to_string()],
                }),
                (X12Version::V8010, ElementVersionInfo {
                    valid_codes: Some(vec![
                        "ST".to_string(), // State Tax
                        "LT".to_string(), // Local Tax
                        "FT".to_string(), // Federal Tax
                        "GS".to_string(), // Goods and Services Tax
                        "VA".to_string(), // Value Added Tax
                        "DT".to_string(), // Digital Tax (new in v8010)
                        "CT".to_string(), // Carbon Tax (new in v8010)
                    ]),
                    notes: Some("Full tax types including digital compliance for v8010".to_string()),
                    usage_rules: vec![
                        "Required in TAX segment".to_string(),
                        "DT and CT codes require additional documentation".to_string(),
                    ],
                }),
            ]),
        });

        // Add monetary amount element
        self.element_dictionary.insert(782, ElementMasterDefinition {
            element_id: 782,
            name: "Monetary Amount".to_string(),
            description: "Monetary amount".to_string(),
            data_type: ElementDataType::R,
            min_length: Some(1),
            max_length: Some(18),
            version_implementations: HashMap::from([
                (X12Version::V4010, ElementVersionInfo {
                    valid_codes: None, // No code restrictions for monetary amounts
                    notes: Some("Standard monetary amount format".to_string()),
                    usage_rules: vec!["Must be positive for tax amounts".to_string()],
                }),
                (X12Version::V5010, ElementVersionInfo {
                    valid_codes: None,
                    notes: Some("Enhanced precision support".to_string()),
                    usage_rules: vec![
                        "Must be positive for tax amounts".to_string(),
                        "Up to 6 decimal places supported".to_string(),
                    ],
                }),
                (X12Version::V8010, ElementVersionInfo {
                    valid_codes: None,
                    notes: Some("Full precision with digital currency support".to_string()),
                    usage_rules: vec![
                        "Must be positive for tax amounts".to_string(),
                        "Up to 8 decimal places for digital currencies".to_string(),
                        "Supports micro-payment precision".to_string(),
                    ],
                }),
            ]),
        });
    }
}
```

## Best Practices

### 1. Naming Conventions

- **Segment IDs**: Use official X12 segment identifiers (e.g., "TAX", "ADV", "RMR")
- **Element IDs**: Use official X12 data element numbers (e.g., 963, 782, 954)
- **Method Names**: Use pattern `register_[segment_id]_segment()` (e.g., `register_tax_segment()`)
- **Schema Names**: Use pattern `create_[transaction_id]_schema()` (e.g., `create_820_schema()`)

### 2. Version Management

- Always define elements for all supported versions
- Use version-specific logic when element definitions differ
- Document version differences in comments
- Test each version separately

### 3. Partner Customizations

- Keep partner customizations separate from base schemas
- Document business reasons for each customization
- Use descriptive partner IDs (e.g., "WALMART_US", "TARGET_CORP")
- Validate partner customizations don't conflict with standards

### 4. Testing

Always add tests for new segments and transaction types:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_segment_validation() {
        let registry = SegmentRegistry::new();
        let tax_def = registry.get_definition(&X12Version::V4010, "TAX").unwrap();
        
        assert_eq!(tax_def.id, "TAX");
        assert_eq!(tax_def.elements.len(), 3);
        assert_eq!(tax_def.elements[0].element_id, 963);
    }

    #[test]
    fn test_820_schema_creation() {
        let engine = SchemaEngine::new();
        let schema = engine.get_transaction_schema("820", &X12Version::V4010).unwrap();
        
        assert_eq!(schema.transaction_type, "820");
        assert_eq!(schema.name, "Payment Order/Remittance Advice");
        assert!(schema.segments.iter().any(|s| s.segment_id == "BPR"));
    }

    #[test]
    fn test_partner_customization() {
        let mut engine = SchemaEngine::new();
        
        let walmart_agreement = TradingPartnerAgreement {
            partner_id: "WALMART_US".to_string(),
            agreement_name: "Test Agreement".to_string(),
            customizations: vec![],
        };
        
        let result = engine.add_trading_partner_agreement("850", X12Version::V4010, walmart_agreement);
        assert!(result.is_ok());
    }
}
```

## Examples

### Complete Example: Adding a FOB (Free On Board) Segment

```rust
// 1. Add to SegmentRegistry
impl SegmentRegistry {
    fn register_fob_segment(&mut self) {
        let fob_definition = SegmentDefinition {
            id: "FOB".to_string(),
            name: "F.O.B. Related Instructions".to_string(),
            elements: vec![
                ElementDefinition {
                    element_id: 146,
                    name: "Shipment Method of Payment".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(2),
                    max_length: Some(2),
                    requirement: ElementRequirement::Mandatory,
                    description: "Code identifying payment terms for transportation charges".to_string(),
                    valid_codes: Some(vec![
                        "BP".to_string(), // Paid by Buyer
                        "CC".to_string(), // Collect
                        "DF".to_string(), // Defined by Buyer and Seller
                        "FO".to_string(), // FOB Port of Call
                        "HP".to_string(), // Half Prepaid
                        "MX".to_string(), // Mixed
                        "PC".to_string(), // Prepaid but Charged to Customer
                        "PP".to_string(), // Prepaid (by Seller)
                        "PS".to_string(), // Paid by Seller
                        "PU".to_string(), // Pickup
                        "TP".to_string(), // Third Party Pay
                    ]),
                },
                ElementDefinition {
                    element_id: 309,
                    name: "Location Qualifier".to_string(),
                    data_type: ElementDataType::ID,
                    min_length: Some(1),
                    max_length: Some(2),
                    requirement: ElementRequirement::Optional,
                    description: "Code identifying type of location".to_string(),
                    valid_codes: Some(vec![
                        "OR".to_string(), // Origin (Shipping Point)
                        "DE".to_string(), // Destination (Receiving Point)
                        "VE".to_string(), // Vessel
                        "PL".to_string(), // Plant
                        "WH".to_string(), // Warehouse
                    ]),
                },
                ElementDefinition {
                    element_id: 352,
                    name: "Description".to_string(),
                    data_type: ElementDataType::AN,
                    min_length: Some(1),
                    max_length: Some(80),
                    requirement: ElementRequirement::Optional,
                    description: "Free-form description of the FOB terms".to_string(),
                    valid_codes: None,
                },
            ],
            min_usage: 0,
            max_usage: Some(1),
            description: "To specify transportation special handling requirements, or hazardous materials information, or both".to_string(),
        };

        self.register_segment(X12Version::V4010, fob_definition.clone());
        self.register_segment(X12Version::V5010, fob_definition.clone());
        self.register_segment(X12Version::V8010, fob_definition);
    }
}

// 2. Add to SchemaEngine transaction definitions
// In create_850_schema method, add:
SegmentSchema {
    segment_id: "FOB".to_string(),
    name: "F.O.B. Related Instructions".to_string(),
    requirement: SegmentRequirement::Optional,
    max_use: Some(1),
    hierarchy_level: 0, // Header level
    dependencies: vec![],
    context_rules: vec![
        ContextRule {
            rule_id: "FOB_PAYMENT_VALIDATION".to_string(),
            description: "FOB payment terms must be valid".to_string(),
            condition: "FOB01 is present".to_string(),
            action: ValidationAction::Error("Invalid FOB payment terms".to_string()),
        }
    ],
},

// 3. Add element definitions to element dictionary
self.element_dictionary.insert(146, ElementMasterDefinition {
    element_id: 146,
    name: "Shipment Method of Payment".to_string(),
    description: "Code identifying payment terms for transportation charges".to_string(),
    data_type: ElementDataType::ID,
    min_length: Some(2),
    max_length: Some(2),
    version_implementations: HashMap::from([
        (X12Version::V4010, ElementVersionInfo {
            valid_codes: Some(vec!["BP".to_string(), "CC".to_string(), "PP".to_string()]),
            notes: Some("Basic payment terms".to_string()),
            usage_rules: vec!["Required when FOB segment is used".to_string()],
        }),
        // ... other versions
    ]),
});

// 4. Add test
#[test]
fn test_fob_segment_validation() {
    let registry = SegmentRegistry::new();
    let fob_def = registry.get_definition(&X12Version::V4010, "FOB").unwrap();
    
    assert_eq!(fob_def.id, "FOB");
    assert_eq!(fob_def.elements.len(), 3);
    assert_eq!(fob_def.elements[0].element_id, 146); // Payment method
    assert_eq!(fob_def.elements[1].element_id, 309); // Location qualifier
    assert_eq!(fob_def.elements[2].element_id, 352); // Description
}
```

### Partner-Specific Example: Amazon Requirements

```rust
let amazon_agreement = TradingPartnerAgreement {
    partner_id: "AMAZON_US".to_string(),
    agreement_name: "Amazon Vendor Central EDI Requirements".to_string(),
    customizations: vec![
        // Amazon requires FOB segment for all purchase orders
        SchemaCustomization {
            segment_id: "FOB".to_string(),
            element_position: None,
            customization_type: CustomizationType::MakeMandatory,
            description: "Amazon requires FOB terms for all purchase orders".to_string(),
        },
        // Amazon only accepts specific FOB payment terms
        SchemaCustomization {
            segment_id: "FOB".to_string(),
            element_position: Some(0), // FOB01
            customization_type: CustomizationType::RestrictValidCodes(vec![
                "PP".to_string(), // Prepaid only
            ]),
            description: "Amazon only accepts prepaid shipping terms".to_string(),
        },
        // Amazon requires detailed item descriptions
        SchemaCustomization {
            segment_id: "PID".to_string(),
            element_position: Some(4), // Product description
            customization_type: CustomizationType::ChangeLengthConstraints { 
                min: Some(20), 
                max: Some(200) 
            },
            description: "Amazon requires detailed product descriptions for catalog matching".to_string(),
        },
    ],
};
```

This guide should help you extend the EDI Parser with any additional segments, document types, and partner-specific requirements. Remember to always test your changes thoroughly and document any business logic or partner requirements.
