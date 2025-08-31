use edi_parser::{
    X12Parser,
    EdiParser,
    InterchangeControl,
    EdiError,
    TransactionType,
    PurchaseOrder850,
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

    println!("=== BASIC SEGMENT-BY-SEGMENT APPROACH ===");
    for (fg_index, fg) in interchange.functional_groups.iter().enumerate() {
        for (tx_index, transaction) in fg.transactions.iter().enumerate() {
            println!("Transaction {}: {} - Type: {:?}",
                tx_index + 1,
                transaction.transaction_set_id,
                transaction.transaction_type);

            // Show all segments sequentially
            for (i, segment) in transaction.segments.iter().enumerate() {
                println!("  {:2}: {} - {} elements",
                    i + 1,
                    segment.id,
                    segment.elements.len());

                // Show first few elements for key segments
                if ["BEG", "PO1", "N1", "DTM"].contains(&segment.id.as_str()) {
                    println!("      Data: {:?}", &segment.elements[..segment.elements.len().min(4)]);
                }
            }
        }
    }

    println!("\n=== STRUCTURED LOOP APPROACH ===");
    for fg in &interchange.functional_groups {
        for transaction in &fg.transactions {
            if let Ok(po850) = PurchaseOrder850::parse_from_transaction(transaction) {
                println!("Purchase Order: {}", po850.transaction_type.as_str());
                println!("Header segments: {}", po850.header_segments.len());
                println!("Party loops: {}", po850.party_loops.len());
                println!("Line item loops: {}", po850.line_item_loops.len());
                println!("Summary segments: {}", po850.summary_segments.len());

                // Show structured data
                println!("\n--- Header Information ---");
                for segment in &po850.header_segments {
                    match segment.id.as_str() {
                        "BEG" => {
                            let po_num = segment.elements.get(2).map(|s| s.as_str()).unwrap_or("");
                            let po_type = segment.elements.get(1).map(|s| s.as_str()).unwrap_or("");
                            println!("PO Number: {}", po_num);
                            println!("PO Type: {}", po_type);
                        }
                        "DTM" => {
                            let qualifier = segment.elements.get(0).map(|s| s.as_str()).unwrap_or("");
                            let date = segment.elements.get(1).map(|s| s.as_str()).unwrap_or("");
                            println!("Date ({}): {}", qualifier, date);
                        }
                        _ => {}
                    }
                }

                println!("\n--- Parties ---");
                for (_i, party) in po850.party_loops.iter().enumerate() {
                    let entity_type = party.n1_segment.elements.get(0).map(|s| s.as_str()).unwrap_or("");
                    let name = party.n1_segment.elements.get(1).map(|s| s.as_str()).unwrap_or("");
                    println!("Party {}: {} - {}", _i + 1, entity_type, name);

                    if let Some(n4) = &party.n4_segment {
                        let city = n4.elements.get(0).map(|s| s.as_str()).unwrap_or("");
                        let state = n4.elements.get(1).map(|s| s.as_str()).unwrap_or("");
                        println!("  Location: {}, {}", city, state);
                    }
                }

                println!("\n--- Line Items ---");
                for (_i, line_item) in po850.line_item_loops.iter().enumerate() {
                    let line_num = line_item.po1_segment.elements.get(0).map(|s| s.as_str()).unwrap_or("");
                    let qty = line_item.po1_segment.elements.get(1).map(|s| s.as_str()).unwrap_or("");
                    let uom = line_item.po1_segment.elements.get(2).map(|s| s.as_str()).unwrap_or("");
                    let price = line_item.po1_segment.elements.get(3).map(|s| s.as_str()).unwrap_or("");

                    println!("Line {}: {} {} at ${}",
                        line_num, qty, uom, price);

                    // Show descriptions
                    for pid in &line_item.pid_segments {
                        if pid.elements.len() > 4 {
                            println!("  Description: {}", pid.elements[4]);
                        }
                    }
                }

                println!("\n--- Summary ---");
                println!("Total line items: {}", po850.get_total_line_items());
                println!("Total quantity: {}", po850.get_total_quantity());

                // Find specific parties
                let ship_to_parties = po850.get_parties_by_type("ST");
                println!("Ship-to parties: {}", ship_to_parties.len());
            }
        }
    }

    Ok(())
}
