# EDI-Parser

A high-performance EDI (Electronic Data Interchange) parser written in Rust, supporting X12 format with extensible segment validation and loop-aware parsing.

## Features

- ✅ **X12 Standard Support**: Full X12 EDI parsing with version detection (4010, 5010, 6010)
- ✅ **Document Type Recognition**: Automatic detection of 810 (Invoice), 850 (Purchase Order), and custom transaction types
- ✅ **Segment Validation**: Built-in validation for common segments (BEG, PO1, N1, DTM, etc.)
- ✅ **Loop-Aware Parsing**: Structured parsing of EDI loops (party loops, line item loops)
- ✅ **Extensible Architecture**: Easy to add new segments, document types, and validation rules
- ✅ **Error Handling**: Comprehensive error reporting with detailed validation messages
- ✅ **Performance**: Zero-copy parsing with efficient memory usage

## Quick Start

### Prerequisites
- Rust 1.70+ installed
- Cargo package manager

### Installation
```bash
git clone https://github.com/Forworddash/EDI-Parser.git
cd EDI-Parser
cargo build --release
```

### Basic Usage
```rust
use edi_parser::{X12Parser, EdiParser};

let parser = X12Parser::default();
let edi_content = "ISA*00*...*IEA*1*000000001~"; // Your EDI data
let result = parser.parse(&edi_content);

match result {
    Ok(interchange) => {
        println!("Parsed {} transactions", interchange.functional_groups[0].transactions.len());
        println!("EDI Version: {}", interchange.version.as_str());
    }
    Err(e) => println!("Parse error: {}", e),
}
```

## Testing

### Running Tests

#### Run All Tests
```bash
cargo test
```

#### Run Specific Test
```bash
cargo test test_850_extended_parsing
```

#### Run with Verbose Output
```bash
cargo test -- --nocapture
```

### Test File Locations

#### Built-in Test Files
Test EDI files are located in `tests/test_files/`:
- `sample_810.edi` - Invoice (810) transaction
- `sample_850.edi` - Basic Purchase Order (850)
- `sample_850_extended.edi` - Extended Purchase Order with loops
- `invalid_sample.edi` - Invalid EDI for error testing

#### Adding Your Own Test Files

1. **Create test EDI file** in `tests/test_files/`:
```bash
# Example: Create a new 850 test file
echo "ISA*00*          *00*          *01*BUYERID      *01*SELLERID     *230101*1300*U*00401*000000002*0*T*>~
GS*PO*BUYERID*SELLERID*20230101*1300*2*X*004010~
ST*850*0001~
BEG*00*SA*PO-001**20230101~
N1*ST*Test Company*92*12345~
PO1*1*10*EA*15.00**BP*TEST-001~
CTT*1~
SE*6*0001~
GE*1*2~
IEA*1*000000002~" > tests/test_files/my_test_850.edi
```

2. **Add test function** in `tests/integration_tests.rs`:
```rust
#[test]
fn test_my_custom_850() {
    let content = fs::read_to_string("tests/test_files/my_test_850.edi")
        .expect("Failed to read test file");

    let parser = X12Parser::default();
    let result = parser.parse(&content);

    assert!(result.is_ok(), "Parse failed: {:?}", result.err());

    let interchange = result.unwrap();
    assert_eq!(interchange.version, X12Version::V4010);
    assert_eq!(interchange.functional_groups[0].transactions[0].transaction_set_id, "850");
}
```

3. **Run your test**:
```bash
cargo test test_my_custom_850
```

### Testing Examples

#### Run Basic Parser Example
```bash
cargo run --example basic_parser
```
This uses `tests/test_files/sample_810.edi` by default.

#### Run Extended 850 Parser
```bash
cargo run --example extended_850_parser
```
Demonstrates segment validation and loop parsing.

#### Run Loop Parsing Demo
```bash
cargo run --example loop_parsing_demo
```
Shows both basic and structured loop parsing approaches.

## Usage Examples

### Basic Parsing
```rust
use edi_parser::{X12Parser, EdiParser};

let parser = X12Parser::default();
let edi_data = fs::read_to_string("path/to/your/file.edi")?;
let interchange = parser.parse(&edi_data)?;

// Access parsed data
println!("Version: {}", interchange.version.as_str());
for fg in &interchange.functional_groups {
    for transaction in &fg.transactions {
        println!("Transaction: {} ({})",
            transaction.transaction_set_id,
            transaction.transaction_type.as_str());
    }
}
```

### Structured Loop Parsing
```rust
use edi_parser::{X12Parser, EdiParser, PurchaseOrder850};

let parser = X12Parser::default();
let interchange = parser.parse(&edi_data)?;

// Parse into structured loops
for fg in &interchange.functional_groups {
    for transaction in &fg.transactions {
        if let Ok(po850) = PurchaseOrder850::parse_from_transaction(transaction) {
            println!("PO Number: {}", po850.get_total_line_items());
            println!("Total Quantity: {}", po850.get_total_quantity());

            // Access specific parties
            let ship_to_parties = po850.get_parties_by_type("ST");
            for party in ship_to_parties {
                println!("Ship To: {}", party.n1_segment.elements[1]);
            }
        }
    }
}
```

### Custom Segment Validation
```rust
use edi_parser::{TransactionType, Segment};

// Add custom validation
impl TransactionType {
    pub fn validate_custom_segment(&self, segment: &Segment) -> Result<(), String> {
        match segment.id.as_str() {
            "MY_SEG" => {
                if segment.elements.len() < 2 {
                    return Err("MY_SEG requires at least 2 elements".to_string());
                }
                // Your custom validation logic
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
```

## Adding New Segments

### Method 1: Simple Validation
Add to `src/models/transaction.rs` in the `validate_850_segment()` method:

```rust
"NEW_SEG" => {
    // Validate NEW_SEG elements
    if segment.elements.is_empty() {
        return Err("NEW_SEG requires at least 1 element".to_string());
    }

    let code = &segment.elements[0];
    if !["A", "B", "C"].contains(&code.as_str()) {
        return Err(format!("Invalid NEW_SEG code: {}", code));
    }

    Ok(())
}
```

### Method 2: Loop-Based Parsing
Extend the loop structures in `src/models/loops.rs`:

```rust
pub struct LineItemLoop {
    pub po1_segment: Segment,
    pub pid_segments: Vec<Segment>,
    pub new_seg_segments: Vec<Segment>,  // Add your new segment
    // ... other fields
}
```

### Method 3: New Document Type
1. Add to `TransactionType` enum in `src/models/transaction.rs`
2. Implement required/optional segments
3. Add validation methods

## File Structure

```
EDI-Parser/
├── src/
│   ├── lib.rs              # Main library exports
│   ├── error.rs            # Error types
│   ├── models/
│   │   ├── mod.rs          # Model exports
│   │   ├── segment.rs      # Basic segment structure
│   │   ├── transaction.rs  # Transaction types and validation
│   │   ├── interchange.rs  # Interchange structure
│   │   ├── version.rs      # X12 version handling
│   │   └── loops.rs        # Loop-based parsing structures
│   ├── parsers/
│   │   ├── mod.rs          # Parser exports
│   │   ├── x12.rs          # X12 parser implementation
│   │   └── common.rs       # Common parser utilities
│   └── utils/
│       └── mod.rs          # Utility functions
├── tests/
│   ├── integration_tests.rs    # Integration tests
│   └── test_files/            # EDI test files
│       ├── sample_810.edi
│       ├── sample_850.edi
│       ├── sample_850_extended.edi
│       └── invalid_sample.edi
├── examples/
│   ├── basic_parser.rs        # Basic parsing example
│   ├── extended_850_parser.rs # Extended 850 parsing
│   └── loop_parsing_demo.rs   # Loop parsing demonstration
├── Cargo.toml
└── README.md
```

## API Reference

### Core Types
- `X12Parser` - Main parser for X12 EDI
- `InterchangeControl` - Top-level EDI structure
- `FunctionalGroup` - GS-GE group
- `Transaction` - ST-SE transaction
- `Segment` - Individual EDI segment
- `PurchaseOrder850` - Structured 850 parsing
- `TransactionType` - Document type enumeration
- `X12Version` - EDI version enumeration

### Key Methods
- `X12Parser::parse()` - Parse EDI string
- `X12Parser::validate()` - Validate parsed structure
- `PurchaseOrder850::parse_from_transaction()` - Structured parsing
- `TransactionType::validate_segment()` - Segment validation

## Error Handling

The parser provides detailed error messages:

```rust
use edi_parser::EdiError;

match parser.parse(&edi_data) {
    Ok(interchange) => { /* Success */ }
    Err(EdiError::InvalidSegmentFormat(msg)) => {
        println!("Segment format error: {}", msg);
    }
    Err(EdiError::MissingRequiredSegment(seg)) => {
        println!("Missing required segment: {}", seg);
    }
    Err(EdiError::ValidationError(msg)) => {
        println!("Validation error: {}", msg);
    }
    _ => { /* Other errors */ }
}
```

## Performance

- **Zero-copy parsing** where possible
- **Efficient memory usage** with Vec-based storage
- **Fast validation** with early error detection
- **Scalable architecture** for large EDI files

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

### Adding New Features
- Add comprehensive tests
- Update documentation
- Follow existing code patterns
- Maintain backward compatibility

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For questions or issues:
1. Check existing tests and examples
2. Review the API documentation
3. Create an issue with sample EDI data
4. Include error messages and expected behavior
