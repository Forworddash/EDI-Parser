use edi_parser::{
    X12Parser,
    EdiParser,
    InterchangeControl,
    EdiError,
    TransactionType,
    X12Version,
};
use std::fs;

fn main() -> Result<(), EdiError> {
    // Try to read a custom test file, fall back to sample if not found
    let test_files = [
        "my_custom_test.edi",           // User's custom file in project root
        "tests/test_files/my_test_850.edi", // User's test file in test directory
        "tests/test_files/sample_850_extended.edi", // Fallback
    ];

    let mut content = None;
    let mut used_file = "";

    for file in &test_files {
        if let Ok(file_content) = fs::read_to_string(file) {
            content = Some(file_content);
            used_file = file;
            break;
        }
    }

    let content = match content {
        Some(c) => c,
        None => {
            println!("❌ No EDI test file found. Please create one of:");
            for file in &test_files {
                println!("   - {}", file);
            }
            println!("\n💡 Tip: Copy sample_850_extended.edi and modify it for testing");
            return Ok(());
        }
    };

    println!("📄 Testing with file: {}", used_file);
    println!("========================");

    // Parse the content
    let parser = X12Parser::default();
    let interchange = parser.parse(&content)?;

    // Validate the parsed structure
    parser.validate(&interchange)?;

    // Display results
    println!("✅ EDI parsed successfully!");
    println!("📊 Version: {}", interchange.version.as_str());
    println!("📦 Functional Groups: {}", interchange.functional_groups.len());

    for (fg_idx, fg) in interchange.functional_groups.iter().enumerate() {
        println!("\n🏢 Functional Group {}: {}", fg_idx + 1, fg.gs_segment.elements.get(0).unwrap_or(&String::new()));

        for (tx_idx, transaction) in fg.transactions.iter().enumerate() {
            println!("  📋 Transaction {}: {} ({}) - Type: {:?}",
                tx_idx + 1,
                transaction.transaction_set_id,
                transaction.control_number,
                transaction.transaction_type);

            // Show segment count
            println!("    📊 Segments: {}", transaction.segments.len());

            // Show key segments
            let mut segment_counts = std::collections::HashMap::new();
            for segment in &transaction.segments {
                *segment_counts.entry(segment.id.clone()).or_insert(0) += 1;
            }

            println!("    🔢 Segment breakdown: {:?}", segment_counts);

            // Validate segments
            let mut valid_segments = 0;
            let mut invalid_segments = 0;

            for segment in &transaction.segments {
                match transaction.transaction_type.validate_segment(segment) {
                    Ok(_) => valid_segments += 1,
                    Err(_) => invalid_segments += 1,
                }
            }

            println!("    ✅ Valid segments: {}", valid_segments);
            if invalid_segments > 0 {
                println!("    ❌ Invalid segments: {}", invalid_segments);
            }
        }
    }

    // Show structured parsing if it's an 850
    if let Some(transaction) = interchange.functional_groups.first()
        .and_then(|fg| fg.transactions.first()) {

        if matches!(transaction.transaction_type, TransactionType::PurchaseOrder850) {
            println!("\n🔄 Attempting structured 850 parsing...");

            match edi_parser::PurchaseOrder850::parse_from_transaction(transaction) {
                Ok(po850) => {
                    println!("✅ Structured parsing successful!");
                    println!("📊 Header segments: {}", po850.header_segments.len());
                    println!("🏢 Party loops: {}", po850.party_loops.len());
                    println!("📦 Line item loops: {}", po850.line_item_loops.len());
                    println!("📈 Total quantity: {}", po850.get_total_quantity());
                    println!("📋 Total line items: {}", po850.get_total_line_items());
                }
                Err(e) => {
                    println!("⚠️  Structured parsing failed: {}", e);
                    println!("💡 This might be expected if your EDI file has a different structure");
                }
            }
        }
    }

    println!("\n🎉 Test completed successfully!");
    println!("💡 To test with your own EDI file:");
    println!("   1. Create 'my_custom_test.edi' in the project root");
    println!("   2. Or place it in 'tests/test_files/' directory");
    println!("   3. Run this example again");

    Ok(())
}
