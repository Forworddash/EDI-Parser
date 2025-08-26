# Adding New Segments to the EDI Parser

This guide explains how to add new structured segments to the EDI parser with full validation support.

## Overview

The EDI parser uses a three-layer approach for segments:
1. **Segment Definition** - Defines validation rules and structure
2. **Structured Segment** - Type-safe Rust struct representing the segment
3. **Registration** - Adds the segment to the registry for validation

## Step-by-Step Guide

### Step 1: Add Segment Definition to Registry

Edit `src/models/segment_definition.rs` and add your segment registration method:

```rust
impl SegmentRegistry {
    fn register_standard_segments(&mut self) {
        // ... existing segments
        self.register_your_new_segment(); // Add this line
    }

    fn register_your_new_segment(&mut self) {
        let segment_definition = SegmentDefinition {
            id: "YourSegmentID".to_string(),
            name: "Your Segment Name".to_string(),
            elements: vec![
                ElementDefinition {
                    name: "Element 1 Name".to_string(),
                    data_type: ElementDataType::AN, // or N, R, ID, DT, TM
                    min_length: Some(1),
                    max_length: Some(30),
                    required: true,
                    description: "Description of what this element contains".to_string(),
                },
                ElementDefinition {
                    name: "Element 2 Name".to_string(),
                    data_type: ElementDataType::N,
                    min_length: Some(1),
                    max_length: Some(6),
                    required: false,
                    description: "Another element description".to_string(),
                },
                // Add more elements as needed
            ],
            min_usage: 0, // Minimum times this segment can appear
            max_usage: Some(1), // Maximum times (None = unlimited)
            description: "Purpose and usage of this segment".to_string(),
        };

        // Register for all supported X12 versions
        self.register_segment(X12Version::V4010, segment_definition.clone());
        self.register_segment(X12Version::V5010, segment_definition.clone());
        self.register_segment(X12Version::V8010, segment_definition);
    }
}
```

### Step 2: Create Structured Segment Type

Add your structured segment to `src/models/structured_segments.rs`:

```rust
/// YourSegmentID - Your Segment Description
#[derive(Debug, Clone, PartialEq)]
pub struct YourSegmentStruct {
    pub element_1: String,           // Required elements as direct fields
    pub element_2: Option<u32>,      // Optional elements as Option<T>
    pub element_3: Option<String>,   // Use appropriate types (String, u32, f64, etc.)
}

impl EdiSegment for YourSegmentStruct {
    fn segment_id() -> &'static str {
        "YourSegmentID"
    }

    fn from_segment(segment: &Segment, version: X12Version) -> Result<Self, EdiError> {
        if segment.id != Self::segment_id() {
            return Err(EdiError::InvalidSegmentFormat(
                format!("Expected {} segment, got {}", Self::segment_id(), segment.id)
            ));
        }

        // Validate the segment first
        let validator = SegmentValidator::new(version);
        let validation_result = validator.validate_segment(segment);
        if !validation_result.is_valid {
            return Err(EdiError::InvalidSegmentFormat(
                format!("{} segment validation failed: {:?}", Self::segment_id(), validation_result.errors)
            ));
        }

        // Check minimum required elements
        if segment.elements.len() < 1 { // Adjust based on required elements
            return Err(EdiError::InvalidSegmentFormat(
                format!("{} segment requires at least 1 element", Self::segment_id())
            ));
        }

        Ok(YourSegmentStruct {
            element_1: segment.elements[0].clone(),
            element_2: Self::get_optional_element(&segment.elements, 1)
                .and_then(|s| s.parse().ok()),
            element_3: Self::get_optional_element(&segment.elements, 2),
        })
    }

    fn to_segment(&self) -> Segment {
        let mut elements = vec![
            self.element_1.clone(),
            self.element_2.map(|e| e.to_string()).unwrap_or_default(),
            self.element_3.clone().unwrap_or_default(),
        ];

        // Remove trailing empty elements (keep required ones)
        while elements.len() > 1 && elements.last().map_or(false, |e| e.is_empty()) {
            elements.pop();
        }

        Segment::new(Self::segment_id().to_string(), elements)
    }

    fn validate(&self, version: X12Version) -> SegmentValidationResult {
        let validator = SegmentValidator::new(version);
        validator.validate_segment(&self.to_segment())
    }
}

impl YourSegmentStruct {
    fn get_optional_element(elements: &[String], index: usize) -> Option<String> {
        elements.get(index)
            .filter(|s| !s.is_empty())
            .cloned()
    }
}
```

### Step 3: Add Tests

Add tests for your new segment in the same file or in the test files:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_segment_creation() {
        let segment = Segment::new(
            "YourSegmentID".to_string(),
            vec![
                "VALUE1".to_string(),
                "123".to_string(),
                "VALUE3".to_string(),
            ],
        );

        let your_seg = YourSegmentStruct::from_segment(&segment, X12Version::V4010).unwrap();
        assert_eq!(your_seg.element_1, "VALUE1");
        assert_eq!(your_seg.element_2, Some(123));
        assert_eq!(your_seg.element_3, Some("VALUE3".to_string()));

        let back_to_segment = your_seg.to_segment();
        assert_eq!(back_to_segment.id, "YourSegmentID");
        assert_eq!(back_to_segment.elements.len(), 3);
    }
}
```

## Real Examples

### Example 1: Adding REF (Reference Information) Segment

This segment is already defined in the registry but needs a structured implementation:

```rust
/// REF - Reference Information
#[derive(Debug, Clone, PartialEq)]
pub struct RefSegment {
    pub reference_identification_qualifier: String,
    pub reference_identification: Option<String>,
    pub description: Option<String>,
}

impl EdiSegment for RefSegment {
    fn segment_id() -> &'static str {
        "REF"
    }

    fn from_segment(segment: &Segment, version: X12Version) -> Result<Self, EdiError> {
        if segment.id != Self::segment_id() {
            return Err(EdiError::InvalidSegmentFormat(
                format!("Expected {} segment, got {}", Self::segment_id(), segment.id)
            ));
        }

        if segment.elements.is_empty() {
            return Err(EdiError::InvalidSegmentFormat(
                "REF segment requires at least 1 element".to_string()
            ));
        }

        let validator = SegmentValidator::new(version);
        let validation_result = validator.validate_segment(segment);
        if !validation_result.is_valid {
            return Err(EdiError::InvalidSegmentFormat(
                format!("REF segment validation failed: {:?}", validation_result.errors)
            ));
        }

        Ok(RefSegment {
            reference_identification_qualifier: segment.elements[0].clone(),
            reference_identification: Self::get_optional_element(&segment.elements, 1),
            description: Self::get_optional_element(&segment.elements, 2),
        })
    }

    fn to_segment(&self) -> Segment {
        let mut elements = vec![
            self.reference_identification_qualifier.clone(),
            self.reference_identification.clone().unwrap_or_default(),
            self.description.clone().unwrap_or_default(),
        ];

        while elements.len() > 1 && elements.last().map_or(false, |e| e.is_empty()) {
            elements.pop();
        }

        Segment::new(Self::segment_id().to_string(), elements)
    }

    fn validate(&self, version: X12Version) -> SegmentValidationResult {
        let validator = SegmentValidator::new(version);
        validator.validate_segment(&self.to_segment())
    }
}

impl RefSegment {
    fn get_optional_element(elements: &[String], index: usize) -> Option<String> {
        elements.get(index)
            .filter(|s| !s.is_empty())
            .cloned()
    }
}
```

### Example 2: Adding DTM (Date/Time Reference) Segment

```rust
/// DTM - Date/Time Reference
#[derive(Debug, Clone, PartialEq)]
pub struct DtmSegment {
    pub date_time_qualifier: String,
    pub date: Option<String>,
    pub time: Option<String>,
}

impl EdiSegment for DtmSegment {
    fn segment_id() -> &'static str {
        "DTM"
    }

    fn from_segment(segment: &Segment, version: X12Version) -> Result<Self, EdiError> {
        if segment.id != Self::segment_id() {
            return Err(EdiError::InvalidSegmentFormat(
                format!("Expected {} segment, got {}", Self::segment_id(), segment.id)
            ));
        }

        if segment.elements.is_empty() {
            return Err(EdiError::InvalidSegmentFormat(
                "DTM segment requires at least 1 element".to_string()
            ));
        }

        let validator = SegmentValidator::new(version);
        let validation_result = validator.validate_segment(segment);
        if !validation_result.is_valid {
            return Err(EdiError::InvalidSegmentFormat(
                format!("DTM segment validation failed: {:?}", validation_result.errors)
            ));
        }

        Ok(DtmSegment {
            date_time_qualifier: segment.elements[0].clone(),
            date: Self::get_optional_element(&segment.elements, 1),
            time: Self::get_optional_element(&segment.elements, 2),
        })
    }

    fn to_segment(&self) -> Segment {
        let mut elements = vec![
            self.date_time_qualifier.clone(),
            self.date.clone().unwrap_or_default(),
            self.time.clone().unwrap_or_default(),
        ];

        while elements.len() > 1 && elements.last().map_or(false, |e| e.is_empty()) {
            elements.pop();
        }

        Segment::new(Self::segment_id().to_string(), elements)
    }

    fn validate(&self, version: X12Version) -> SegmentValidationResult {
        let validator = SegmentValidator::new(version);
        validator.validate_segment(&self.to_segment())
    }
}

impl DtmSegment {
    fn get_optional_element(elements: &[String], index: usize) -> Option<String> {
        elements.get(index)
            .filter(|s| !s.is_empty())
            .cloned()
    }
}
```

## Data Types Reference

Use these data types in your element definitions:

- **`ElementDataType::AN`** - Alphanumeric (letters, numbers, spaces, special chars)
- **`ElementDataType::N`** - Numeric only (0-9)
- **`ElementDataType::R`** - Decimal number (supports decimals)
- **`ElementDataType::ID`** - Identifier/Code (alphanumeric but usually coded values)
- **`ElementDataType::DT`** - Date (CCYYMMDD format)
- **`ElementDataType::TM`** - Time (HHMM, HHMMSS, or HHMMSSDD format)

## Usage Patterns

### Creating segments programmatically:
```rust
let ref_segment = RefSegment {
    reference_identification_qualifier: "PO".to_string(),
    reference_identification: Some("ORDER123".to_string()),
    description: None,
};

// Validate
let validation_result = ref_segment.validate(X12Version::V4010);
if validation_result.is_valid {
    println!("Segment is valid!");
}

// Convert to generic segment for serialization
let generic_segment = ref_segment.to_segment();
```

### Parsing from EDI data:
```rust
// After parsing EDI document
for segment in &transaction.segments {
    match segment.id.as_str() {
        "REF" => {
            match RefSegment::from_segment(segment, X12Version::V4010) {
                Ok(ref_seg) => {
                    println!("Reference: {} = {:?}", 
                        ref_seg.reference_identification_qualifier,
                        ref_seg.reference_identification);
                }
                Err(e) => println!("Failed to parse REF: {}", e),
            }
        }
        _ => { /* handle other segments */ }
    }
}
```

## Tips and Best Practices

1. **Always validate first** - Use the validator before creating structured segments
2. **Handle optional elements** - Use `Option<T>` for non-required elements
3. **Use appropriate types** - Convert to proper Rust types (u32, f64, etc.) when possible
4. **Test thoroughly** - Include tests for valid, invalid, and edge cases
5. **Follow naming conventions** - Use descriptive field names from X12 documentation
6. **Support all versions** - Register your segment for all X12 versions you support
7. **Document element meanings** - Include clear descriptions in element definitions

This system makes it easy to add new segments while maintaining type safety and comprehensive validation!
