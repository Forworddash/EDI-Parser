use edi_parser::{
    X12Parser, EdiParser, ValidatingSegmentParser, X12Version,
    Segment, ValidatedElement, ElementRequirement,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 EDI Element ID and Validation System Demo");
    println!("============================================\n");

    // Create a validating parser for X12 version 4010
    let validator = ValidatingSegmentParser::new(X12Version::V4010);

    // Example 1: Validate a BEG segment with element IDs
    println!("📋 Example 1: BEG Segment Validation");
    println!("-----------------------------------");
    
    let beg_segment = Segment::new(
        "BEG".to_string(),
        vec![
            "00".to_string(),        // Element ID 353: Transaction Set Purpose Code
            "SA".to_string(),        // Element ID 92:  Purchase Order Type Code  
            "PO-123456".to_string(), // Element ID 324: Purchase Order Number
            "REL-001".to_string(),   // Element ID 328: Release Number (optional)
            "20240826".to_string(),  // Element ID 373: Date
        ],
    );

    match validator.parse_and_validate(&beg_segment) {
        Ok(validated) => {
            println!("✅ BEG segment validation passed!");
            
            // Show element details with IDs
            for element in &validated.elements {
                println!("  Element ID {}: {} = '{}'", 
                    element.element_id(),
                    element.element_name(),
                    element.value().unwrap_or("(empty)")
                );
            }
            
            // Access specific elements by ID
            if let Some(po_number) = validated.get_element_by_id(324) {
                println!("  🆔 Purchase Order Number (Element 324): {}", 
                    po_number.value().unwrap_or("N/A"));
            }
        }
        Err(e) => println!("❌ BEG validation failed: {}", e),
    }

    // Example 2: Validate a CUR segment
    println!("\n💱 Example 2: CUR Segment Validation");
    println!("-----------------------------------");
    
    let cur_segment = Segment::new(
        "CUR".to_string(),
        vec![
            "BY".to_string(),   // Element ID 98:  Entity Identifier Code
            "USD".to_string(),  // Element ID 100: Currency Code
            "1.25".to_string(), // Element ID 280: Exchange Rate
        ],
    );

    match validator.parse_and_validate(&cur_segment) {
        Ok(validated) => {
            println!("✅ CUR segment validation passed!");
            
            // Demonstrate accessing by element ID
            if let Some(currency) = validated.get_element_by_id(100) {
                println!("  💰 Currency Code (Element 100): {}", 
                    currency.value().unwrap_or("N/A"));
            }
            
            if let Some(exchange_rate) = validated.get_element_by_id(280) {
                println!("  📈 Exchange Rate (Element 280): {}", 
                    exchange_rate.value().unwrap_or("N/A"));
            }
        }
        Err(e) => println!("❌ CUR validation failed: {}", e),
    }

    // Example 3: Show validation errors
    println!("\n❌ Example 3: Validation Error Demonstration");
    println!("--------------------------------------------");
    
    let invalid_beg = Segment::new(
        "BEG".to_string(),
        vec![
            "99".to_string(),        // Invalid purpose code
            "INVALID".to_string(),   // Invalid PO type code
            "".to_string(),          // Missing required PO number
            "".to_string(),          // Optional release number
            "2024-08-26".to_string(), // Invalid date format
        ],
    );

    match validator.parse_and_validate(&invalid_beg) {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => {
            println!("Expected validation errors:");
            println!("  {}", e);
        }
    }

    // Example 4: Element ID lookup across segments
    println!("\n🔍 Example 4: Element ID Cross-Reference");
    println!("---------------------------------------");
    
    // Find all segments that use element ID 98 (Entity Identifier Code)
    let element_98_usage = validator.get_element_definition_by_id(98);
    println!("Element ID 98 (Entity Identifier Code) is used in:");
    for (segment_id, element_def) in element_98_usage {
        println!("  📄 Segment {}: {} ({})", 
            segment_id, 
            element_def.name,
            match element_def.requirement {
                ElementRequirement::Mandatory => "Required",
                ElementRequirement::Optional => "Optional", 
                ElementRequirement::Conditional => "Conditional",
            }
        );
    }

    // Example 5: Individual element validation
    println!("\n🧪 Example 5: Individual Element Validation");
    println!("-------------------------------------------");
    
    // Test individual element validation
    let test_cases = vec![
        ("BEG", 1, "00", "Valid purpose code"),
        ("BEG", 1, "99", "Invalid purpose code"),
        ("BEG", 3, "PO-123", "Valid PO number"),
        ("BEG", 3, "", "Empty required field"),
        ("BEG", 5, "20240826", "Valid date"),
        ("BEG", 5, "2024-08-26", "Invalid date format"),
        ("CUR", 2, "USD", "Valid currency"),
        ("CUR", 2, "XYZ", "Invalid currency"),
    ];

    for (segment_id, position, value, description) in test_cases {
        match validator.validate_element_value(segment_id, position, value) {
            Ok(()) => println!("  ✅ {}: '{}' - VALID", description, value),
            Err(e) => println!("  ❌ {}: '{}' - INVALID ({})", description, value, e),
        }
    }

    // Example 6: Show available segments for version
    println!("\n📋 Example 6: Available Segments for X12 v4010");
    println!("----------------------------------------------");
    
    let available_segments = validator.get_available_segments();
    println!("Registered segments:");
    for segment_id in available_segments {
        println!("  📄 {}", segment_id);
    }

    // Example 7: Version-specific differences (future enhancement)
    println!("\n🔄 Example 7: Version Comparison (Concept)");
    println!("------------------------------------------");
    println!("In version 4010: BEG segment has 5 elements");
    println!("In version 5010: BEG segment might have additional elements");
    println!("Element IDs remain consistent across versions for data mapping");

    Ok(())
}

/// Helper function to demonstrate how to add element ID mapping to existing segments
fn demonstrate_element_id_integration() {
    println!("\n🔧 How to Add Element IDs to Your Existing Segments:");
    println!("===================================================");
    
    println!("1. Update your segment structs to include element metadata:");
    println!("   #[derive(Debug, Clone)]");
    println!("   pub struct BegSegmentWithIds {{");
    println!("     pub purpose_code: (u32, String),  // (element_id, value)");
    println!("     pub po_type: (u32, String),       // (92, value)");
    println!("     pub po_number: (u32, String),     // (324, value)");
    println!("   }}");
    
    println!("\n2. Use ValidatingSegmentParser for validation:");
    println!("   let validator = ValidatingSegmentParser::new(X12Version::V4010);");
    println!("   let validated = validator.parse_and_validate(&segment)?;");
    
    println!("\n3. Access elements by ID for reliable data mapping:");
    println!("   let po_number = validated.get_element_by_id(324);");
    println!("   let currency = validated.get_element_by_id(100);");
    
    println!("\n4. Benefits:");
    println!("   ✅ Consistent element identification across versions");
    println!("   ✅ Automatic validation against EDI standards");
    println!("   ✅ Better integration with other EDI systems");
    println!("   ✅ Easier debugging and troubleshooting");
}
