use edi_parser::models::X12Version;
use edi_parser::parsers::schema_validating_parser::{
    SchemaValidatingParser, ParsingConfig, ValidationLevel
};
use edi_parser::parsers::x12::X12Parser;
use edi_parser::parsers::EdiParser;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 EDI Interactive Tester");
    println!("=========================");
    println!("Paste your EDI content below and press Enter twice when done:");
    println!();

    // Read multi-line input
    let mut edi_content = String::new();
    let mut empty_line_count = 0;
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        
        if line.trim().is_empty() {
            empty_line_count += 1;
            if empty_line_count >= 2 {
                break;
            }
        } else {
            empty_line_count = 0;
            edi_content.push_str(&line);
        }
    }

    if edi_content.trim().is_empty() {
        println!("❌ No EDI content provided. Exiting.");
        return Ok(());
    }

    println!("\n🔍 Analyzing your EDI content...\n");

    // Test basic parsing first
    test_basic_parsing(&edi_content)?;
    
    // Then schema validation
    test_schema_validation(&edi_content)?;

    // Ask if user wants detailed analysis
    println!("Would you like detailed element analysis? (y/n): ");
    let mut response = String::new();
    io::stdin().read_line(&mut response)?;
    
    if response.trim().to_lowercase().starts_with('y') {
        test_element_analysis(&edi_content)?;
    }

    Ok(())
}

fn test_basic_parsing(edi_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Basic Parsing Test:");
    println!("----------------------");

    let mut parser = X12Parser::new();
    
    match parser.parse(edi_content) {
        Ok(interchange) => {
            println!("✅ EDI structure is valid!");
            
            // Extract sender and receiver from ISA segment
            if interchange.isa_segment.elements.len() >= 8 {
                println!("   📤 Sender: {}", interchange.isa_segment.elements[5].trim());
                println!("   📥 Receiver: {}", interchange.isa_segment.elements[7].trim());
            }
            
            println!("   📋 Functional Groups: {}", interchange.functional_groups.len());
            
            for (i, group) in interchange.functional_groups.iter().enumerate() {
                println!("   📁 Group {} - {} transaction(s)", i + 1, group.transactions.len());
                for (j, transaction) in group.transactions.iter().enumerate() {
                    println!("      📄 Transaction {} - Type: {}, Segments: {}", 
                        j + 1, transaction.transaction_set_id, transaction.segments.len());
                }
            }
        }
        Err(e) => {
            println!("❌ Parsing failed: {}", e);
            println!("💡 Common issues:");
            println!("   - Check segment terminators (~)");
            println!("   - Verify element separators (*)");
            println!("   - Ensure ISA/GS/ST/SE/GE/IEA structure");
        }
    }
    
    println!();
    Ok(())
}

fn test_schema_validation(edi_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Schema Validation Test:");
    println!("--------------------------");

    let config = ParsingConfig {
        version: X12Version::V4010,
        validation_level: ValidationLevel::Standard,
        continue_on_error: true,
        collect_validation_details: true,
        trading_partner_id: None,
    };

    let mut parser = SchemaValidatingParser::new(config);
    
    match parser.parse_with_schema(edi_content) {
        Ok(result) => {
            println!("✅ Schema validation completed!");
            println!("   📊 Status: {:?}", result.status);
            println!("   📊 Segments: {}", result.segments.len());
            
            let errors = result.validation_results.iter()
                .filter(|r| matches!(r.result_type, edi_parser::models::schema_engine::ValidationResultType::Error))
                .count();
            let warnings = result.validation_results.iter()
                .filter(|r| matches!(r.result_type, edi_parser::models::schema_engine::ValidationResultType::Warning))
                .count();

            if errors == 0 && warnings == 0 {
                println!("   🎉 No validation issues found!");
            } else {
                println!("   📊 Issues: {} errors, {} warnings", errors, warnings);
                
                // Show first few issues
                if errors > 0 {
                    println!("   🚨 Errors:");
                    for (i, validation_result) in result.validation_results.iter()
                        .filter(|r| matches!(r.result_type, edi_parser::models::schema_engine::ValidationResultType::Error))
                        .take(3)
                        .enumerate() {
                        println!("      {}. {}", i + 1, validation_result.message);
                    }
                }
                
                if warnings > 0 {
                    println!("   ⚠️  Warnings:");
                    for (i, validation_result) in result.validation_results.iter()
                        .filter(|r| matches!(r.result_type, edi_parser::models::schema_engine::ValidationResultType::Warning))
                        .take(3)
                        .enumerate() {
                        println!("      {}. {}", i + 1, validation_result.message);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Schema validation failed: {}", e);
        }
    }
    
    println!();
    Ok(())
}

fn test_element_analysis(edi_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Element ID Analysis:");
    println!("-----------------------");

    let config = ParsingConfig::default();
    let mut parser = SchemaValidatingParser::new(config);
    
    match parser.parse_with_schema(edi_content) {
        Ok(result) => {
            let mut found_elements = false;
            
            for validated_segment in &result.segments {
                let segment = &validated_segment.segment;
                
                match segment.id.as_str() {
                    "CUR" if segment.elements.len() >= 2 => {
                        println!("   💰 Currency: CUR01='{}' (Element 98), CUR02='{}' (Element 100)", 
                            segment.elements[0], segment.elements[1]);
                        found_elements = true;
                    }
                    "REF" if segment.elements.len() >= 2 => {
                        println!("   📋 Reference: REF01='{}' (Element 128), REF02='{}' (Element 127)", 
                            segment.elements[0], segment.elements[1]);
                        found_elements = true;
                    }
                    "BEG" if segment.elements.len() >= 3 => {
                        println!("   📄 Purchase Order: BEG01='{}' (Element 353), BEG03='{}' (Element 324)", 
                            segment.elements[0], segment.elements[2]);
                        found_elements = true;
                    }
                    "PO1" if segment.elements.len() >= 4 => {
                        println!("   📦 Line Item: PO101='{}', PO102='{}' (Element 330), PO104='{}' (Element 212)", 
                            segment.elements[0], segment.elements[1], segment.elements[3]);
                        found_elements = true;
                    }
                    "N1" if segment.elements.len() >= 2 => {
                        println!("   🏢 Name: N101='{}' (Element 98), N102='{}' (Element 93)", 
                            segment.elements[0], segment.elements[1]);
                        found_elements = true;
                    }
                    _ => {}
                }
            }

            if !found_elements {
                println!("   ℹ️  No recognized business segments found for element analysis.");
                println!("   📊 Total segments found: {}", result.segments.len());
            }
        }
        Err(e) => {
            println!("❌ Element analysis failed: {}", e);
        }
    }
    
    println!();
    Ok(())
}
