# Quick Segment Addition Cheat Sheet

## 1. Add to Registry (segment_definition.rs)

```rust
// In register_standard_segments()
self.register_your_segment_name();

// New method
fn register_your_segment_name(&mut self) {
    let definition = SegmentDefinition {
        id: "ABC".to_string(),
        name: "Your Segment Name".to_string(), 
        elements: vec![
            ElementDefinition {
                name: "Field Name".to_string(),
                data_type: ElementDataType::AN, // AN, N, R, ID, DT, TM
                min_length: Some(1),
                max_length: Some(30),
                required: true,
                description: "What this field contains".to_string(),
            },
            // ... more elements
        ],
        min_usage: 0,
        max_usage: Some(1),
        description: "Segment purpose".to_string(),
    };
    
    self.register_segment(X12Version::V4010, definition.clone());
    self.register_segment(X12Version::V5010, definition.clone());
    self.register_segment(X12Version::V8010, definition);
}
```

## 2. Create Struct (structured_segments.rs)

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct YourSegment {
    pub field_1: String,           // Required
    pub field_2: Option<String>,   // Optional
    pub field_3: Option<u32>,      // Optional number
}

impl EdiSegment for YourSegment {
    fn segment_id() -> &'static str { "ABC" }
    
    fn from_segment(segment: &Segment, version: X12Version) -> Result<Self, EdiError> {
        // Validation + parsing logic
    }
    
    fn to_segment(&self) -> Segment {
        // Convert back to generic segment
    }
    
    fn validate(&self, version: X12Version) -> SegmentValidationResult {
        let validator = SegmentValidator::new(version);
        validator.validate_segment(&self.to_segment())
    }
}
```

## 3. Data Types

- `ElementDataType::AN` - Text (alphanumeric)
- `ElementDataType::N` - Numbers only  
- `ElementDataType::R` - Decimal numbers
- `ElementDataType::ID` - Codes/identifiers
- `ElementDataType::DT` - Dates (CCYYMMDD)
- `ElementDataType::TM` - Times (HHMM/HHMMSS)

## 4. Usage

```rust
// Create programmatically
let my_segment = YourSegment {
    field_1: "VALUE".to_string(),
    field_2: Some("OPTIONAL".to_string()),
    field_3: None,
};

// Validate
let result = my_segment.validate(X12Version::V4010);

// Parse from EDI
match YourSegment::from_segment(&segment, X12Version::V4010) {
    Ok(structured) => { /* use it */ }
    Err(e) => { /* handle error */ }
}
```

## 5. Testing

```rust
#[test]
fn test_your_segment() {
    let segment = Segment::new("ABC".to_string(), vec!["VAL1".to_string()]);
    let structured = YourSegment::from_segment(&segment, X12Version::V4010).unwrap();
    assert_eq!(structured.field_1, "VAL1");
}
```

That's it! See `ADDING_SEGMENTS.md` for complete examples.
