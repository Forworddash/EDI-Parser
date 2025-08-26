# EDI Element ID System

This document explains how the EDI parser implements element IDs for consistent data mapping and validation.

## Overview

In EDI (Electronic Data Interchange), each data element has a standardized ID number that remains consistent across different versions and implementations. For example:
- Element ID 98: Entity Identifier Code (used in CUR01, REF01, etc.)
- Element ID 100: Currency Code (used in CUR02)
- Element ID 324: Purchase Order Number (used in BEG03)

## Why Element IDs Matter

1. **Version Consistency**: Element IDs remain the same across X12 versions (4010, 5010, etc.)
2. **System Integration**: Other EDI systems reference elements by ID for mapping
3. **Validation**: Proper validation rules can be applied based on element type
4. **Debugging**: Easier to identify specific elements when troubleshooting

## Implementation

### Element Definition Structure

```rust
#[derive(Debug, Clone)]
pub struct ElementDefinition {
    pub element_id: u32,           // Standard EDI element ID
    pub name: String,              // Human-readable name
    pub data_type: DataType,       // String, Numeric, Date, etc.
    pub min_length: Option<u32>,   // Minimum allowed length
    pub max_length: Option<u32>,   // Maximum allowed length
    pub requirement: ElementRequirement, // Mandatory/Optional/Conditional
    pub valid_values: Option<Vec<String>>, // Code list if applicable
}
```

### Common Element IDs

| Element ID | Name | Common Usage | Example Values |
|------------|------|--------------|----------------|
| 92 | Purchase Order Type Code | BEG02 | SA, KN, NE |
| 98 | Entity Identifier Code | CUR01, REF01 | BY, ST, BT |
| 100 | Currency Code | CUR02 | USD, EUR, CAD |
| 324 | Purchase Order Number | BEG03 | PO-123456 |
| 328 | Release Number | BEG04 | REL-001 |
| 353 | Transaction Set Purpose Code | BEG01 | 00, 01, 04 |
| 373 | Date | BEG05, DTM02 | 20240826 |

### Usage Examples

#### Basic Validation

```rust
use edi_parser::{ValidatingSegmentParser, X12Version};

let validator = ValidatingSegmentParser::new(X12Version::V4010);
let validated_segment = validator.parse_and_validate(&segment)?;

// Access element by ID
if let Some(po_number) = validated_segment.get_element_by_id(324) {
    println!("PO Number: {}", po_number.value().unwrap_or("N/A"));
}
```

#### Segment Definition with Element IDs

```rust
// BEG segment definition for version 4010
SegmentDefinition {
    segment_id: "BEG".to_string(),
    description: "Beginning Segment for Purchase Order".to_string(),
    elements: vec![
        ElementDefinition {
            element_id: 353,
            name: "Transaction Set Purpose Code".to_string(),
            data_type: DataType::String,
            min_length: Some(2),
            max_length: Some(2),
            requirement: ElementRequirement::Mandatory,
            valid_values: Some(vec!["00".to_string(), "01".to_string(), "04".to_string()]),
        },
        ElementDefinition {
            element_id: 92,
            name: "Purchase Order Type Code".to_string(),
            data_type: DataType::String,
            min_length: Some(2),
            max_length: Some(2),
            requirement: ElementRequirement::Mandatory,
            valid_values: Some(vec!["SA".to_string(), "KN".to_string(), "NE".to_string()]),
        },
        // ... more elements
    ],
}
```

## Running the Example

To see the element ID system in action:

```bash
cargo run --example element_id_validation
```

This example demonstrates:
- Element validation with IDs
- Accessing elements by ID
- Cross-referencing element usage
- Error reporting with element context
- Version-specific definitions

## Integration Guide

### For Existing Code

1. **Update your segment structs** to include element metadata
2. **Use ValidatingSegmentParser** instead of basic parsing
3. **Access elements by ID** for reliable data mapping
4. **Add validation** based on element definitions

### Benefits

- ✅ **Standards Compliance**: Follows official EDI element numbering
- ✅ **Reliable Integration**: Compatible with other EDI systems
- ✅ **Better Validation**: Type-specific validation rules
- ✅ **Easier Debugging**: Clear element identification
- ✅ **Version Independence**: Same IDs across X12 versions

## Version Differences

The element ID system handles version differences:

```rust
// Version 4010
let v4010_validator = ValidatingSegmentParser::new(X12Version::V4010);

// Version 5010 (when implemented)
let v5010_validator = ValidatingSegmentParser::new(X12Version::V5010);

// Element IDs remain consistent, but validation rules may differ
```

## Error Handling

The system provides detailed error messages with element context:

```
Validation Error: Element 324 (Purchase Order Number) at position 3: 
Value '' is invalid - Required element cannot be empty (min_length: 1, max_length: 22)
```

## Future Enhancements

- [ ] Complete element registry for all X12 segments
- [ ] Composite element support (sub-elements)
- [ ] Dynamic validation rule loading
- [ ] Element dependency validation
- [ ] Integration with X12 data dictionaries
