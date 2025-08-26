# Structured Segments Documentation

This document describes the structured segment validation system for the EDI Parser, which provides an easy and straightforward way to add individual segments with semantic validation, including min/max length of characters, data types, repeatability, and multi-version X12 implementation guide support.

## Overview

The structured segment system consists of three main components:

1. **Segment Definitions** - Define the structure and validation rules for each segment type
2. **Segment Validator** - Validates segments against their definitions
3. **Structured Segments** - Type-safe representations of specific segment types

## Key Features

- ✅ **Type-safe segment parsing** - Convert between generic segments and strongly-typed structs
- ✅ **Comprehensive validation** - Element data types, length constraints, required/optional fields
- ✅ **Multi-version support** - Support for different X12 implementation guide versions (4010, 5010, 8010)
- ✅ **Extensible architecture** - Easy to add new segment types
- ✅ **Detailed error reporting** - Specific validation errors with element positions
- ✅ **Semantic validation** - Business rule validation beyond basic parsing

## Quick Start

### Basic Usage

```rust
use edi_parser::{
    X12Parser, EdiParser,
    models::{
        structured_segments::{EdiSegment, BegSegment},
        segment_definition::X12Version,
    }
};

// Parse an EDI document
let parser = X12Parser::default();
let interchange = parser.parse(edi_data)?;

// Get a transaction's segments
let transaction = &interchange.functional_groups[0].transactions[0];

// Find and convert a BEG segment
for segment in &transaction.segments {
    if segment.id == "BEG" {
        let beg = BegSegment::from_segment(segment, X12Version::V4010)?;
        println!("PO Number: {}", beg.purchase_order_number);
        println!("Date: {}", beg.date);
    }
}
```

### Creating Segments Programmatically

```rust
use edi_parser::models::structured_segments::{BegSegment, EdiSegment};

// Create a new BEG segment
let beg = BegSegment {
    transaction_set_purpose_code: "00".to_string(),
    purchase_order_type_code: "NE".to_string(),
    purchase_order_number: "PO-123456".to_string(),
    release_number: Some("REL-001".to_string()),
    date: "20250825".to_string(),
};

// Validate it
let validation_result = beg.validate(X12Version::V4010);
if validation_result.is_valid {
    // Convert to generic segment for transmission
    let segment = beg.to_segment();
    // Use segment in EDI document...
}
```

## Supported Segments

Currently implemented structured segments for 850 Purchase Orders:

### BEG - Beginning Segment for Purchase Order
- **Usage**: Required (1)
- **Purpose**: Identifies the beginning of a Purchase Order transaction
- **Elements**:
  - Transaction Set Purpose Code (Required, 2 chars)
  - Purchase Order Type Code (Required, 2 chars)  
  - Purchase Order Number (Required, 1-22 chars)
  - Release Number (Optional, 1-30 chars)
  - Date (Required, 8 chars, CCYYMMDD format)

### PO1 - Baseline Item Data
- **Usage**: Required (1-100,000)
- **Purpose**: Specifies line item details
- **Elements**:
  - Assigned Identification (Optional, 1-20 chars)
  - Quantity Ordered (Optional, decimal)
  - Unit of Measurement Code (Optional, 2 chars)
  - Unit Price (Optional, decimal)
  - Basis of Unit Price Code (Optional, 2 chars)
  - Product/Service ID pairs (Multiple pairs supported)

### N1 - Party Identification  
- **Usage**: Optional (0-200)
- **Purpose**: Identifies parties (ship-to, bill-to, etc.)
- **Elements**:
  - Entity Identifier Code (Required, 2-3 chars)
  - Name (Optional, 1-60 chars)
  - Identification Code Qualifier (Optional, 1-2 chars)
  - Identification Code (Optional, 2-80 chars)

### CTT - Transaction Totals
- **Usage**: Optional (0-1)
- **Purpose**: Provides transaction summary information
- **Elements**:
  - Number of Line Items (Required, numeric)
  - Hash Total (Optional, decimal)

## Data Type Validation

The system supports comprehensive data type validation:

| Type | Description | Validation |
|------|-------------|------------|
| `AN` | Alphanumeric | Printable ASCII characters |
| `N` | Numeric | Digits only (0-9) |
| `R` | Decimal | Numeric with optional decimal point |
| `ID` | Identifier | Coded values, alphanumeric |
| `DT` | Date | CCYYMMDD format (e.g., 20250825) |
| `TM` | Time | HHMM, HHMMSS, or HHMMSSDD format |

## Adding New Segments

### 1. Define the Segment Structure

Add a new segment definition to the registry:

```rust
fn register_new_segment(&mut self) {
    let definition = SegmentDefinition {
        id: "NEW".to_string(),
        name: "New Segment".to_string(),
        elements: vec![
            ElementDefinition {
                name: "Element 1".to_string(),
                data_type: ElementDataType::AN,
                min_length: Some(1),
                max_length: Some(20),
                required: true,
                description: "First element description".to_string(),
            },
            // Add more elements...
        ],
        min_usage: 1,
        max_usage: Some(1),
        description: "Segment description".to_string(),
    };
    
    self.register_segment(X12Version::V4010, definition);
}
```

### 2. Create a Structured Segment Type

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct NewSegment {
    pub element1: String,
    pub element2: Option<String>,
}

impl EdiSegment for NewSegment {
    fn segment_id() -> &'static str {
        "NEW"
    }

    fn from_segment(segment: &Segment, version: X12Version) -> Result<Self, EdiError> {
        // Validation and parsing logic
        let validator = SegmentValidator::new(version);
        let validation_result = validator.validate_segment(segment);
        if !validation_result.is_valid {
            return Err(EdiError::InvalidSegmentFormat(
                format!("NEW segment validation failed: {:?}", validation_result.errors)
            ));
        }

        Ok(NewSegment {
            element1: segment.elements[0].clone(),
            element2: segment.elements.get(1).cloned(),
        })
    }

    fn to_segment(&self) -> Segment {
        let elements = vec![
            self.element1.clone(),
            self.element2.clone().unwrap_or_default(),
        ];
        Segment::new(Self::segment_id().to_string(), elements)
    }

    fn validate(&self, version: X12Version) -> SegmentValidationResult {
        let validator = SegmentValidator::new(version);
        validator.validate_segment(&self.to_segment())
    }
}
```

## Validation Examples

### Comprehensive Validation

```rust
use edi_parser::models::segment_validator::SegmentValidator;

let validator = SegmentValidator::new(X12Version::V4010);
let validation_result = validator.validate_segment(&segment);

if !validation_result.is_valid {
    println!("Validation errors:");
    for error in &validation_result.errors {
        println!("  - Element {}: {}", 
                 error.element_position.unwrap_or(0), 
                 error.message);
    }
}

if !validation_result.warnings.is_empty() {
    println!("Validation warnings:");
    for warning in &validation_result.warnings {
        println!("  - Element {}: {}", 
                 warning.element_position.unwrap_or(0), 
                 warning.message);
    }
}
```

### Segment Usage Validation

```rust
// Validate segment usage counts within a transaction
let validator = SegmentValidator::new(X12Version::V4010);

// Check if BEG segment usage is within limits (should be exactly 1)
validator.validate_segment_usage(&transaction.segments, "BEG")?;

// Check if PO1 segments are within limits (1 to 100,000)
validator.validate_segment_usage(&transaction.segments, "PO1")?;
```

## X12 Version Support

The system supports multiple X12 implementation guide versions:

```rust
use edi_parser::models::segment_definition::X12Version;

// Different versions may have different validation rules
let validator_4010 = SegmentValidator::new(X12Version::V4010);
let validator_5010 = SegmentValidator::new(X12Version::V5010);
let validator_8010 = SegmentValidator::new(X12Version::V8010);

// Validate against specific version
let beg = BegSegment::from_segment(&segment, X12Version::V5010)?;
```

## Error Handling

The system provides detailed error information:

```rust
use edi_parser::models::segment_validator::{ValidationError, ValidationErrorType};

match validation_result.errors.first() {
    Some(error) => match error.error_type {
        ValidationErrorType::MissingRequiredElement => {
            println!("Missing required element at position {}", 
                     error.element_position.unwrap());
        }
        ValidationErrorType::InvalidLength => {
            println!("Invalid length: {}", error.message);
        }
        ValidationErrorType::InvalidDataType => {
            println!("Invalid data type: {}", error.message);
        }
        ValidationErrorType::InvalidFormat => {
            println!("Invalid format: {}", error.message);
        }
        ValidationErrorType::TooManyElements => {
            println!("Too many elements: {}", error.message);
        }
        ValidationErrorType::UnknownSegment => {
            println!("Unknown segment: {}", error.message);
        }
    }
    None => println!("No validation errors"),
}
```

## Best Practices

1. **Always validate** segments before processing business logic
2. **Use structured segments** instead of accessing raw elements by index
3. **Handle validation errors gracefully** and provide meaningful feedback
4. **Register segment definitions** for all segments you plan to use
5. **Test with multiple X12 versions** if you support multiple implementation guides
6. **Use the registry** to introspect segment definitions for documentation or UI generation

## Performance Considerations

- Segment definitions are registered once at startup
- Validation is performed on-demand and cached where possible
- Regular expressions for data type validation are compiled once using `lazy_static`
- Converting between structured and generic segments is lightweight

## Future Enhancements

- Support for composite elements (sub-elements)
- Code list validation for ID elements
- Custom validation rules per implementation guide
- Automatic generation of segment definitions from X12 standards
- Support for conditional requirements based on other elements
- Performance optimizations for high-volume processing
