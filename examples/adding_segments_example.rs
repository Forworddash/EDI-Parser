use edi_parser::{
    models::{
        structured_segments::{EdiSegment, RefSegment},
        segment_definition::X12Version,
        Segment,
    }
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Example: Adding and Using New Segments ===\n");

    // Example 1: Using the newly implemented REF segment
    println!("1. Creating a REF (Reference Information) segment programmatically:");
    
    let ref_segment = RefSegment {
        reference_identification_qualifier: "PO".to_string(),
        reference_identification: Some("ORDER-12345".to_string()),
        description: Some("Purchase Order Reference".to_string()),
    };

    println!("   Reference Qualifier: {}", ref_segment.reference_identification_qualifier);
    println!("   Reference ID: {:?}", ref_segment.reference_identification);
    println!("   Description: {:?}", ref_segment.description);

    // Validate the segment
    let validation_result = ref_segment.validate(X12Version::V4010);
    if validation_result.is_valid {
        println!("   ✅ REF segment is valid!");
    } else {
        println!("   ❌ REF segment has validation issues:");
        for error in &validation_result.errors {
            println!("      - {}", error.message);
        }
    }

    // Convert to generic segment
    let generic_segment = ref_segment.to_segment();
    println!("   Generic segment: {} with elements: {:?}", generic_segment.id, generic_segment.elements);

    println!("\n2. Parsing a REF segment from EDI data:");
    
    // Simulate parsing from EDI
    let edi_ref_segment = Segment::new(
        "REF".to_string(),
        vec![
            "VN".to_string(),        // Vendor Number
            "VENDOR-789".to_string(), // Vendor ID
            "".to_string(),          // No description
        ],
    );

    match RefSegment::from_segment(&edi_ref_segment, X12Version::V4010) {
        Ok(parsed_ref) => {
            println!("   ✅ Successfully parsed REF segment:");
            println!("      Qualifier: {}", parsed_ref.reference_identification_qualifier);
            println!("      ID: {:?}", parsed_ref.reference_identification);
            println!("      Description: {:?}", parsed_ref.description);
        }
        Err(e) => {
            println!("   ❌ Failed to parse REF segment: {}", e);
        }
    }

    println!("\n3. Example of how you would add more segments:");
    println!("   - Define the segment in segment_definition.rs");
    println!("   - Create a structured type implementing EdiSegment trait");
    println!("   - Add validation and conversion logic");
    println!("   - Write tests to verify functionality");
    println!("   - Use the segment in your EDI processing code");

    println!("\n   See ADDING_SEGMENTS.md for detailed step-by-step instructions!");

    Ok(())
}
