use edi_parser::{
    X12Parser, EdiParser, TransactionParser, PurchaseOrder850, SegmentParser
};
use edi_parser::segments::{BegSegment, Po1Segment, N1Segment};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read a sample 850 file
    let content = fs::read_to_string("tests/test_files/sample_850.edi")?;
    
    // Parse the EDI interchange
    let parser = X12Parser::default();
    let interchange = parser.parse(&content)?;
    
    println!("✅ Successfully parsed EDI interchange");
    println!("   Functional Groups: {}", interchange.functional_groups.len());
    
    // Process each transaction
    for (fg_idx, functional_group) in interchange.functional_groups.iter().enumerate() {
        println!("\n📦 Functional Group {}", fg_idx + 1);
        
        for (tx_idx, transaction) in functional_group.transactions.iter().enumerate() {
            println!("   📄 Transaction {} (Type: {})", tx_idx + 1, transaction.transaction_set_id);
            
            // If it's a 850 Purchase Order, parse it into structured format
            if transaction.transaction_set_id == "850" {
                match PurchaseOrder850::parse_transaction(transaction) {
                    Ok(po) => {
                        println!("      🛒 Purchase Order Details:");
                        println!("         PO Number: {}", po.purchase_order_number());
                        println!("         Line Items: {}", po.line_items.len());
                        
                        // Show buyer information
                        if let Some(buyer) = po.buyer() {
                            println!("         Buyer: {}", buyer.name.as_deref().unwrap_or("N/A"));
                        }
                        
                        // Show ship-to information
                        if let Some(ship_to) = po.ship_to() {
                            println!("         Ship To: {}", ship_to.name.as_deref().unwrap_or("N/A"));
                        }
                        
                        // Show line item details
                        for (i, item) in po.line_items.iter().enumerate() {
                            println!("         Item {}: Qty={}, Price={}, Product={}", 
                                i + 1,
                                item.quantity_ordered.as_deref().unwrap_or("N/A"),
                                item.unit_price.as_deref().unwrap_or("N/A"),
                                item.product_id_1.as_deref().unwrap_or("N/A")
                            );
                        }
                        
                        // Show total value if calculable
                        if let Some(total) = po.total_value() {
                            println!("         Total Value: ${:.2}", total);
                        }
                        
                        // Validate the purchase order
                        match po.validate() {
                            Ok(()) => println!("         ✅ Validation passed"),
                            Err(e) => println!("         ❌ Validation failed: {}", e),
                        }
                    }
                    Err(e) => {
                        println!("      ❌ Failed to parse as structured PO: {}", e);
                        
                        // Fallback: show raw segment information
                        println!("      📋 Raw segments:");
                        for segment in &transaction.segments {
                            println!("         {}: {} elements", segment.id, segment.elements.len());
                            
                            // Try to parse individual segments for more detail
                            match segment.id.as_str() {
                                "BEG" => {
                                    if let Ok(beg) = BegSegment::parse_segment(segment) {
                                        println!("            PO#: {}", beg.purchase_order_number);
                                    }
                                }
                                "PO1" => {
                                    if let Ok(po1) = Po1Segment::parse_segment(segment) {
                                        println!("            Product: {}", 
                                            po1.product_id_1.as_deref().unwrap_or("N/A"));
                                    }
                                }
                                "N1" => {
                                    if let Ok(n1) = N1Segment::parse_segment(segment) {
                                        println!("            Entity: {} ({})", 
                                            n1.entity_identifier_code,
                                            n1.name.as_deref().unwrap_or("N/A"));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            } else {
                println!("      📋 Raw Transaction (Type: {}):", transaction.transaction_set_id);
                for segment in &transaction.segments {
                    println!("         {}: {} elements", segment.id, segment.elements.len());
                }
            }
        }
    }
    
    Ok(())
}
