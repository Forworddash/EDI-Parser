# EDI Parser - Comprehensive Schema-Driven Parser

A comprehensive EDI (Electronic Data Interchange) parser written in Rust with advanced schema validation, element ID mapping, and version-aware processing.

## 🚀 Key Features

### ✨ **Element ID Mapping System**
- **Cross-Reference Elements by ID**: Find elements like "CUR01 = Element ID 98" across the entire document
- **Version-Aware Validation**: Different validation rules for X12 v004010, v005010, and v008010
- **Rich Element Metadata**: Names, descriptions, data types, length constraints, and valid codes

### 🎯 **Schema-Driven Architecture**
- **Multiple Transaction Types**: 850 (Purchase Order), 810 (Invoice), 856 (Ship Notice), 860 (PO Change), 997 (Functional Ack)
- **Hierarchical Validation**: Header → Detail → Summary segment flow validation
- **Business Rule Enforcement**: Context-aware validation with configurable rules
- **Trading Partner Customizations**: Partner-specific validation rules and constraints

### 🔧 **Advanced Validation Framework**
- **Four Validation Levels**: Basic, Standard, Strict, Complete
- **Comprehensive Error Reporting**: Detailed errors, warnings, and informational messages
- **Performance Metrics**: Parsing time, elements validated, memory usage tracking
- **Business Context Extraction**: Document IDs, currencies, trading partner information

## 📦 Transaction Sets Supported

### X12 Version Support Matrix
| Transaction | v004010 | v005010 | v008010 | Description |
|-------------|---------|---------|---------|-------------|
| 850 | ✅ | ✅ | ✅ | Purchase Order |
| 810 | ✅ | ✅ | ✅ | Invoice |
| 856 | ✅ | ✅ | ✅ | Advance Ship Notice |
| 860 | ✅ | ✅ | ✅ | Purchase Order Change |
| 997 | ✅ | ✅ | ✅ | Functional Acknowledgment |

## 🔍 Element ID Cross-Reference Examples

```rust
// Find all currency codes (Element 100) in the document
let currency_elements = parser.get_elements_by_id(&result, 100);

// Find entity identifiers (Element 98) - "CUR01"
let entity_elements = parser.get_elements_by_id(&result, 98);

// Find reference qualifiers (Element 128) - "REF01"
let ref_qualifiers = parser.get_elements_by_id(&result, 128);
```

### Common Element ID Mappings
| Segment | Position | Element ID | Name |
|---------|----------|------------|------|
| CUR01 | 1 | 98 | Entity Identifier Code |
| CUR02 | 2 | 100 | Currency Code |
| REF01 | 1 | 128 | Reference Identification Qualifier |
| REF02 | 2 | 127 | Reference Identification |
| PO102 | 2 | 330 | Quantity Ordered |
| PO103 | 3 | 355 | Unit or Basis for Measurement Code |
| PO104 | 4 | 212 | Unit Price |

## 🏗️ Architecture Overview

### Core Components
- **`SchemaEngine`**: Comprehensive schema management with version-aware transaction definitions
- **`SchemaValidatingParser`**: Advanced parser with full schema validation and business rule enforcement
- **`SegmentRegistry`**: Element-level validation with ID-based cross-referencing
- **`ValidatedElement`**: Rich element representation with validation metadata

### Parser Types
1. **Basic X12Parser**: Core EDI parsing functionality
2. **ValidatingSegmentParser**: Element-level validation with ID mapping
3. **SchemaValidatingParser**: Full schema-aware parsing with business rules

## 📚 Documentation

- **[Schema Extension Guide](SCHEMA_EXTENSION_GUIDE.md)**: Complete guide for adding segments, transaction types, and partner customizations
- **[Element ID System](ELEMENT_ID_SYSTEM.md)**: Detailed explanation of element ID mapping and cross-referencing
- **[Implementation Summary](IMPLEMENTATION_SUMMARY.md)**: Technical implementation details and architecture decisions

## 🚀 Quick Start

### Basic Usage

```rust
use edi_parser::parsers::schema_validating_parser::{SchemaValidatingParser, ParsingConfig};
use edi_parser::models::X12Version;

// Create parser with configuration
let config = ParsingConfig {
    version: X12Version::V4010,
    validation_level: ValidationLevel::Standard,
    ..Default::default()
};

let mut parser = SchemaValidatingParser::new(config);

// Parse EDI document with full schema validation
let result = parser.parse_with_schema(edi_data)?;

// Access validation results
println!("Parsing Status: {:?}", result.status);
println!("Segments Processed: {}", result.metrics.segments_processed);
println!("Elements Validated: {}", result.metrics.elements_validated);

// Find elements by ID
let currency_codes = parser.get_elements_by_id(&result, 100);
for element in currency_codes {
    println!("Currency: {} in segment {}", element.value, element.segment_id);
}
```

### Advanced Usage with Partner Customizations

```rust
use edi_parser::models::schema_engine::{TradingPartnerAgreement, SchemaCustomization, CustomizationType};

// Configure for specific trading partner
let config = ParsingConfig {
    version: X12Version::V4010,
    trading_partner_id: Some("WALMART_US".to_string()),
    validation_level: ValidationLevel::Strict,
    continue_on_error: false,
    collect_validation_details: true,
};

let mut parser = SchemaValidatingParser::new(config);

// Add partner-specific customizations
let walmart_agreement = TradingPartnerAgreement {
    partner_id: "WALMART_US".to_string(),
    agreement_name: "Walmart EDI Requirements".to_string(),
    customizations: vec![
        SchemaCustomization {
            segment_id: "REF".to_string(),
            element_position: None,
            customization_type: CustomizationType::MakeMandatory,
            description: "Walmart requires REF segment".to_string(),
        },
    ],
};

parser.schema_engine().add_trading_partner_agreement("850", X12Version::V4010, walmart_agreement)?;
```

## 🧪 Examples

Run the included examples to see the parser in action:

```bash
# Element ID mapping demonstration
cargo run --example element_id_mapping_demo

# Comprehensive schema validation demo
cargo run --example comprehensive_schema_demo

# Element ID validation examples
cargo run --example element_id_validation

# Adding segments and partner customizations
cargo run --example adding_segments_example

# Basic parser functionality
cargo run --example basic_parser
```

## 🧪 Testing Your Own EDI Files

### Quick File Testing
Test any EDI file with comprehensive analysis:

```bash
# Test a specific EDI file
cargo run --example edi_file_tester path/to/your/file.edi
cargo run --example edi_file_tester "C:\path\to\your\file.x12"

# Test with sample files
cargo run --example edi_file_tester tests/test_files/sample_850.edi
cargo run --example edi_file_tester tests/test_files/sample_810.edi
```

### Interactive Testing
Paste EDI content directly for immediate analysis:

```bash
cargo run --example edi_interactive_tester
# Then paste your EDI content and press Enter twice
```

### What You Get from Testing
- ✅ **Structure Validation**: ISA/GS/ST envelope validation
- 🔬 **Schema Compliance**: Element and segment validation
- 🔍 **Element ID Tracking**: "CUR01 = Element 98" style mapping
- 📋 **Multi-Version Compatibility**: Tests against X12 v004010/v005010/v008010
- ⏱️ **Performance Metrics**: Parsing speed and validation time
- 📊 **Detailed Reports**: Errors, warnings, and business data extraction

See [EDI Testing Guide](EDI_TESTING_GUIDE.md) for complete testing documentation.

## 🧪 Unit Testing

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --lib                           # Unit tests
cargo test --test integration_tests        # Integration tests
cargo test --test structured_segments_tests # Structured segment tests
```

## 🔧 Extending the Parser

### Adding New Segments
See [Schema Extension Guide](SCHEMA_EXTENSION_GUIDE.md) for detailed instructions on:
- Adding new segment definitions
- Creating version-specific configurations
- Implementing partner-specific customizations
- Managing element dictionary entries

### Adding New Transaction Types
The parser architecture supports easy addition of new transaction types:
1. Define transaction schema in `SchemaEngine`
2. Add segment definitions to `SegmentRegistry`
3. Register element IDs in element dictionary
4. Add validation tests

### Partner-Specific Customizations
The framework supports comprehensive partner customizations:
- Making optional segments mandatory
- Restricting or extending valid code lists
- Changing length constraints
- Adding business rule validations

## 📊 Performance

The parser is designed for production use with:
- **Memory Efficient**: Streaming parser with minimal memory footprint
- **Fast Validation**: Optimized validation algorithms
- **Comprehensive Metrics**: Built-in performance monitoring
- **Error Recovery**: Configurable error handling and recovery

## 🤝 Contributing

Contributions are welcome! Please:
1. Read the [Schema Extension Guide](SCHEMA_EXTENSION_GUIDE.md) for architectural guidelines
2. Add tests for new functionality
3. Follow Rust naming conventions
4. Document any new elements or segments added

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Related Projects

- **X12 Standards**: [ASC X12](https://x12.org/) for official EDI standards
- **EDI Resources**: [EDI Basics](https://www.edibasics.com/) for EDI learning resources
- **Trading Partner Networks**: Various VAN (Value Added Network) providers for EDI transmission

---

**🎉 The EDI Parser provides enterprise-grade EDI processing with comprehensive schema validation, element ID cross-referencing, and trading partner customization capabilities!** X12I Transportation (future addition)
- X12F Finance (future addition)