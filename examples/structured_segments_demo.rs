use edi_parser::{
    X12Parser, EdiParser,
    models::{
        structured_segments::{EdiSegment, BegSegment, Po1Segment, N1Segment, CttSegment},
        segment_definition::X12Version,
        segment_validator::SegmentValidator,
    }
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sample 850 Purchase Order EDI data
    let edi_data = r#"ISA*00*          *00*          *01*BUYERID      *01*SELLERID     *230101*1300*U*00401*000000002*0*T*>~
GS*PO*BUYERID*SELLERID*20230101*1300*2*X*004010~
ST*850*0001~
BEG*00*SA*PO-001**20230101~
N1*ST*Supplier Inc*92*SUP123~
PO1*1*100*EA*10.50**BP*ITEM-001*VP*Vendor Part 001~
CTT*1~
SE*6*0001~
GE*1*2~
IEA*1*000000002~"#;

    println!("=== EDI Parser with Structured Segments Demo ===\n");

    // Parse the EDI document
    let parser = X12Parser::default();
    let interchange = parser.parse(edi_data)?;

    println!("Successfully parsed EDI interchange:");
    println!("- Functional Groups: {}", interchange.functional_groups.len());
    
    if let Some(fg) = interchange.functional_groups.first() {
        println!("- Transactions: {}", fg.transactions.len());
        
        if let Some(transaction) = fg.transactions.first() {
            println!("- Transaction Type: {}", transaction.transaction_set_id);
            println!("- Segments: {}", transaction.segments.len());
            
            // Use structured segments with validation
            let version = X12Version::V4010;
            let validator = SegmentValidator::new(version.clone());
            
            println!("\n=== Structured Segment Analysis ===");
            
            for segment in &transaction.segments {
                match segment.id.as_str() {
                    "BEG" => {
                        println!("\n📄 BEG - Beginning Segment for Purchase Order");
                        
                        // Validate the raw segment first
                        let validation_result = validator.validate_segment(segment);
                        if !validation_result.is_valid {
                            println!("❌ Validation errors:");
                            for error in &validation_result.errors {
                                println!("   - {}", error.message);
                            }
                            continue;
                        }
                        
                        // Convert to structured segment
                        match BegSegment::from_segment(segment, version.clone()) {
                            Ok(beg) => {
                                println!("✅ Successfully parsed BEG segment:");
                                println!("   - Transaction Purpose: {}", beg.transaction_set_purpose_code);
                                println!("   - PO Type: {}", beg.purchase_order_type_code);
                                println!("   - PO Number: {}", beg.purchase_order_number);
                                if let Some(release) = &beg.release_number {
                                    println!("   - Release Number: {}", release);
                                }
                                println!("   - Date: {}", beg.date);
                                
                                // Show how to convert back to generic segment
                                let converted_back = beg.to_segment();
                                println!("   - Converted back to {} elements", converted_back.elements.len());
                            }
                            Err(e) => println!("❌ Failed to parse BEG segment: {}", e),
                        }
                    }
                    "PO1" => {
                        println!("\n📦 PO1 - Baseline Item Data");
                        
                        let validation_result = validator.validate_segment(segment);
                        if !validation_result.is_valid {
                            println!("❌ Validation errors:");
                            for error in &validation_result.errors {
                                println!("   - {}", error.message);
                            }
                            continue;
                        }
                        
                        match Po1Segment::from_segment(segment, version.clone()) {
                            Ok(po1) => {
                                println!("✅ Successfully parsed PO1 segment:");
                                if let Some(id) = &po1.assigned_identification {
                                    println!("   - Line Item ID: {}", id);
                                }
                                if let Some(qty) = po1.quantity_ordered {
                                    println!("   - Quantity: {}", qty);
                                }
                                if let Some(uom) = &po1.unit_of_measurement_code {
                                    println!("   - Unit of Measure: {}", uom);
                                }
                                if let Some(price) = po1.unit_price {
                                    println!("   - Unit Price: ${:.2}", price);
                                }
                                if !po1.product_service_ids.is_empty() {
                                    println!("   - Product IDs:");
                                    for (i, product_id) in po1.product_service_ids.iter().enumerate() {
                                        println!("     {}: {} = {}", i + 1, product_id.qualifier, product_id.id);
                                    }
                                }
                            }
                            Err(e) => println!("❌ Failed to parse PO1 segment: {}", e),
                        }
                    }
                    "N1" => {
                        println!("\n👤 N1 - Party Identification");
                        
                        let validation_result = validator.validate_segment(segment);
                        if !validation_result.is_valid {
                            println!("❌ Validation errors:");
                            for error in &validation_result.errors {
                                println!("   - {}", error.message);
                            }
                            continue;
                        }
                        
                        match N1Segment::from_segment(segment, version.clone()) {
                            Ok(n1) => {
                                println!("✅ Successfully parsed N1 segment:");
                                println!("   - Entity Code: {}", n1.entity_identifier_code);
                                if let Some(name) = &n1.name {
                                    println!("   - Name: {}", name);
                                }
                                if let Some(qual) = &n1.identification_code_qualifier {
                                    println!("   - ID Qualifier: {}", qual);
                                }
                                if let Some(id) = &n1.identification_code {
                                    println!("   - ID Code: {}", id);
                                }
                            }
                            Err(e) => println!("❌ Failed to parse N1 segment: {}", e),
                        }
                    }
                    "CTT" => {
                        println!("\n📊 CTT - Transaction Totals");
                        
                        let validation_result = validator.validate_segment(segment);
                        if !validation_result.is_valid {
                            println!("❌ Validation errors:");
                            for error in &validation_result.errors {
                                println!("   - {}", error.message);
                            }
                            continue;
                        }
                        
                        match CttSegment::from_segment(segment, version.clone()) {
                            Ok(ctt) => {
                                println!("✅ Successfully parsed CTT segment:");
                                println!("   - Number of Line Items: {}", ctt.number_of_line_items);
                                if let Some(hash_total) = ctt.hash_total {
                                    println!("   - Hash Total: {}", hash_total);
                                }
                            }
                            Err(e) => println!("❌ Failed to parse CTT segment: {}", e),
                        }
                    }
                    _ => {
                        println!("\n📋 {} - Generic Segment (no structured implementation yet)", segment.id);
                        let validation_result = validator.validate_segment(segment);
                        if validation_result.is_valid {
                            println!("✅ Segment is valid");
                        } else {
                            println!("❌ Validation issues:");
                            for error in &validation_result.errors {
                                println!("   - {}", error.message);
                            }
                        }
                        println!("   - Elements: {:?}", segment.elements);
                    }
                }
            }
        }
    }

    println!("\n=== Segment Registry Information ===");
    let validator = SegmentValidator::new(X12Version::V4010);
    let registered_segments = validator.get_registered_segments();
    println!("Registered segments for X12 v4010: {:?}", registered_segments);

    // Show detailed information about a segment definition
    if let Some(beg_def) = validator.get_segment_definition("BEG") {
        println!("\nBEG Segment Definition:");
        println!("  Name: {}", beg_def.name);
        println!("  Description: {}", beg_def.description);
        println!("  Usage: {} to {}", beg_def.min_usage, 
                 beg_def.max_usage.map(|u| u.to_string()).unwrap_or("unlimited".to_string()));
        println!("  Elements:");
        for (i, element) in beg_def.elements.iter().enumerate() {
            println!("    {}: {} ({:?}) - {} chars - {}",
                     i + 1,
                     element.name,
                     element.data_type,
                     format!("{}-{}", 
                             element.min_length.map(|l| l.to_string()).unwrap_or("?".to_string()),
                             element.max_length.map(|l| l.to_string()).unwrap_or("?".to_string())),
                     if element.required { "Required" } else { "Optional" });
        }
    }

    println!("\n=== Creating Segments Programmatically ===");
    
    // Create a new BEG segment programmatically
    let new_beg = BegSegment {
        transaction_set_purpose_code: "00".to_string(),
        purchase_order_type_code: "NE".to_string(),
        purchase_order_number: "PO-NEW-001".to_string(),
        release_number: Some("REL-001".to_string()),
        date: "20250825".to_string(),
    };

    println!("Created new BEG segment:");
    println!("  PO Number: {}", new_beg.purchase_order_number);
    println!("  Release: {:?}", new_beg.release_number);

    // Validate the new segment
    let validation_result = new_beg.validate(X12Version::V4010);
    if validation_result.is_valid {
        println!("✅ New BEG segment is valid!");
    } else {
        println!("❌ New BEG segment has validation issues:");
        for error in &validation_result.errors {
            println!("   - {}", error.message);
        }
    }

    // Convert to generic segment for transmission
    let generic_segment = new_beg.to_segment();
    println!("  Generic segment: {} with {} elements", generic_segment.id, generic_segment.elements.len());
    println!("  Elements: {:?}", generic_segment.elements);

    Ok(())
}
