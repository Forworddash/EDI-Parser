use edi_parser::{
    X12Parser,
    EdiParser,
    InterchangeControl,
    EdiError,
    TransactionType,
};
use std::fs;

fn main() -> Result<(), EdiError> {
    // Read extended EDI file
    let content = fs::read_to_string("tests/test_files/sample_850_extended.edi")
        .expect("Failed to read EDI file");

    // Parse the content
    let parser = X12Parser::default();
    let interchange = parser.parse(&content)?;

    // Validate the parsed structure
    parser.validate(&interchange)?;

    // Print basic information
    println!("EDI Version: {}", interchange.version.as_str());
    println!("Interchange Control Number: {}",
        interchange.isa_segment.elements[12]);
    println!("Number of Functional Groups: {}",
        interchange.functional_groups.len());

    for (fg_index, fg) in interchange.functional_groups.iter().enumerate() {
        println!("Functional Group {}:", fg_index + 1);
        println!("  GS Code: {}", fg.gs_segment.elements[0]);

        for (tx_index, transaction) in fg.transactions.iter().enumerate() {
            println!("  Transaction {}: {} ({}) - Type: {:?}",
                tx_index + 1,
                transaction.transaction_set_id,
                transaction.control_number,
                transaction.transaction_type);

            // Show required and optional segments
            let required = transaction.transaction_type.required_segments();
            let optional = transaction.transaction_type.optional_segments();
            println!("    Required segments: {:?}", required);
            println!("    Optional segments: {:?}", optional);

            // Analyze segments
            let mut segment_counts = std::collections::HashMap::new();
            for segment in &transaction.segments {
                *segment_counts.entry(segment.id.clone()).or_insert(0) += 1;

                // Show details for key segments
                match segment.id.as_str() {
                    "BEG" => {
                        println!("    Purchase Order: {} (Type: {}, Purpose: {})",
                            segment.elements.get(2).unwrap_or(&"".to_string()),
                            segment.elements.get(1).unwrap_or(&"".to_string()),
                            segment.elements.get(0).unwrap_or(&"".to_string()));
                    }
                    "PO1" => {
                        if segment.elements.len() >= 4 {
                            println!("    Line Item {}: {} {} at ${}",
                                segment.elements[0],
                                segment.elements[1],
                                segment.elements[2],
                                segment.elements[3]);
                        }
                    }
                    "N1" => {
                        if segment.elements.len() >= 2 {
                            println!("    Party {}: {}",
                                segment.elements[0],
                                segment.elements[1]);
                        }
                    }
                    _ => {}
                }
            }

            println!("    Segment counts: {:?}", segment_counts);
            println!("    Total segments: {}", transaction.segments.len());
        }
    }

    Ok(())
}
