use edi_parser::models::{X12Version, SchemaEngine};
use edi_parser::parsers::schema_validating_parser::{
    SchemaValidatingParser, ParsingConfig, ValidationLevel
};
use edi_parser::error::EdiError;

fn main() -> Result<(), EdiError> {
    println!("🚀 EDI Schema-Driven Parser Demo");
    println!("==================================");

    // Sample 850 Purchase Order with comprehensive data
    let sample_850 = r#"ISA*00*          *00*          *ZZ*SENDER_ID      *ZZ*RECEIVER_ID    *210308*1430*U*00401*000000001*0*P*>~
GS*PO*SENDER_ID*RECEIVER_ID*20210308*1430*1*X*004010~
ST*850*0001~
BEG*00*SA*PO123456**20210308~
CUR*BY*USD~
REF*DP*DEPT001~
REF*VN*VENDOR123~
PER*BD*John Doe*TE*555-1234~
DTM*002*20210315~
N1*ST*ACME CORPORATION~
N2*RECEIVING DEPT~
N3*123 MAIN STREET~
N4*ANYTOWN*NY*12345~
N1*BT*ACME CORPORATION~
N2*ACCOUNTS PAYABLE~
N3*456 BILLING STREET~
N4*BILLING CITY*NY*54321~
PO1*001*100*EA*25.50*CP*VP*WIDGET001~
CUR*BY*USD~
PID*F****Widget Description - Premium Quality~
PO1*002*50*EA*15.75*CP*VP*GADGET002~
PID*F****Gadget Description - Standard Model~
CTT*2*150~
SE*19*0001~
GE*1*1~
IEA*1*000000001~"#;

    // Create schema engine and demonstrate its capabilities
    let schema_engine = SchemaEngine::new();
    
    println!("\n📊 Available Transaction Types:");
    for version in [X12Version::V4010, X12Version::V5010, X12Version::V8010] {
        let transaction_types = schema_engine.get_available_transaction_types(&version);
        println!("  {:?}: {:?}", version, transaction_types);
    }

    println!("\n🔍 Element Dictionary Examples:");
    if let Some(element_98) = schema_engine.get_element_definition(98) {
        println!("  Element {}: {}", element_98.element_id, element_98.name);
        println!("    Description: {}", element_98.description);
        println!("    Data Type: {:?}", element_98.data_type);
        println!("    Length: {:?} - {:?}", element_98.min_length, element_98.max_length);
        
        // Show version-specific implementations
        for (version, impl_info) in &element_98.version_implementations {
            println!("    {:?}:", version);
            if let Some(ref codes) = impl_info.valid_codes {
                println!("      Valid codes: {:?}", codes);
            }
            if let Some(ref notes) = impl_info.notes {
                println!("      Notes: {}", notes);
            }
        }
    }

    if let Some(element_100) = schema_engine.get_element_definition(100) {
        println!("\n  Element {}: {}", element_100.element_id, element_100.name);
        println!("    Description: {}", element_100.description);
    }

    // Test different validation levels
    println!("\n🔧 Testing Different Validation Levels:");
    for validation_level in [ValidationLevel::Basic, ValidationLevel::Standard, ValidationLevel::Strict] {
        println!("\n  🎯 Validation Level: {:?}", validation_level);
        
        let config = ParsingConfig {
            version: X12Version::V4010,
            validation_level,
            continue_on_error: true,
            collect_validation_details: true,
            trading_partner_id: Some("ACME_CORP".to_string()),
        };

        let mut parser = SchemaValidatingParser::new(config);
        
        match parser.parse_with_schema(sample_850) {
            Ok(result) => {
                println!("    ✅ Parsing Status: {:?}", result.status);
                println!("    📈 Metrics:");
                println!("      - Segments Processed: {}", result.metrics.segments_processed);
                println!("      - Elements Validated: {}", result.metrics.elements_validated);
                println!("      - Parsing Time: {}ms", result.metrics.parsing_time_ms);

                println!("    📋 Transaction Summaries:");
                for summary in &result.transaction_summaries {
                    println!("      - Type: {} ({:?})", summary.transaction_type, summary.version);
                    println!("        Status: {:?}", summary.validation_status);
                    println!("        Segments: {}", summary.segment_count);
                    
                    if let Some(ref doc_id) = summary.business_context.document_id {
                        println!("        Document ID: {}", doc_id);
                    }
                    if let Some(ref currency) = summary.business_context.currency {
                        println!("        Currency: {}", currency);
                    }
                    if let Some(ref partner) = summary.business_context.trading_partner {
                        println!("        Trading Partner: {}", partner);
                    }
                }

                // Show validation results summary
                let error_count = result.validation_results.iter()
                    .filter(|r| matches!(r.result_type, edi_parser::models::schema_engine::ValidationResultType::Error))
                    .count();
                let warning_count = result.validation_results.iter()
                    .filter(|r| matches!(r.result_type, edi_parser::models::schema_engine::ValidationResultType::Warning))
                    .count();
                
                println!("    🚨 Validation Results: {} errors, {} warnings", error_count, warning_count);

                // Show first few validation issues
                for (i, result) in result.validation_results.iter().take(3).enumerate() {
                    println!("      {}. {:?}: {}", i + 1, result.result_type, result.message);
                }

                // Demonstrate element ID search
                println!("\n    🔎 Element ID Search Examples:");
                
                // Look for Currency Code (element 100)
                let currency_elements = parser.get_elements_by_id(&result, 100);
                if !currency_elements.is_empty() {
                    println!("      Currency Code (100) found in:");
                    for element_ref in currency_elements {
                        println!("        - Segment {}: {} = '{}'", 
                            element_ref.segment_id, 
                            element_ref.element_name, 
                            element_ref.value
                        );
                    }
                }

                // Look for Entity Identifier Code (element 98) 
                let entity_elements = parser.get_elements_by_id(&result, 98);
                if !entity_elements.is_empty() {
                    println!("      Entity Identifier Code (98) found in:");
                    for element_ref in entity_elements {
                        println!("        - Segment {}: {} = '{}'", 
                            element_ref.segment_id, 
                            element_ref.element_name, 
                            element_ref.value
                        );
                    }
                }

                // Look for Reference Identification Qualifier (element 128)
                let ref_elements = parser.get_elements_by_id(&result, 128);
                if !ref_elements.is_empty() {
                    println!("      Reference ID Qualifier (128) found in:");
                    for element_ref in ref_elements {
                        println!("        - Segment {}: {} = '{}'", 
                            element_ref.segment_id, 
                            element_ref.element_name, 
                            element_ref.value
                        );
                    }
                }
            }
            Err(e) => {
                println!("    ❌ Parsing failed: {}", e);
            }
        }
    }

    // Demonstrate schema information
    println!("\n📚 Schema Information:");
    if let Some(schema_850) = schema_engine.get_transaction_schema("850", &X12Version::V4010) {
        println!("  850 Purchase Order Schema (v4010):");
        println!("    Name: {}", schema_850.name);
        println!("    Description: {}", schema_850.description);
        println!("    Segments defined: {}", schema_850.segments.len());
        
        println!("\n    📋 Segment Overview:");
        for segment in &schema_850.segments {
            let req_text = match segment.requirement {
                edi_parser::models::schema_engine::SegmentRequirement::Mandatory => "Required",
                edi_parser::models::schema_engine::SegmentRequirement::Optional => "Optional", 
                edi_parser::models::schema_engine::SegmentRequirement::Conditional(_) => "Conditional",
            };
            
            let max_text = segment.max_use
                .map(|m| m.to_string())
                .unwrap_or_else(|| "Unlimited".to_string());
                
            println!("      {} - {} (Level {}, {}, Max: {})", 
                segment.segment_id, 
                segment.name, 
                segment.hierarchy_level,
                req_text,
                max_text
            );
        }
    }

    // Demonstrate version differences
    println!("\n🔄 Version Differences:");
    let versions = schema_engine.get_supported_versions("850");
    println!("  850 Purchase Order supported in versions: {:?}", versions);

    // Demonstrate advanced features
    println!("\n🎯 Advanced Features:");
    println!("  ✅ Version-aware element validation");
    println!("  ✅ Transaction structure validation");
    println!("  ✅ Business context extraction");
    println!("  ✅ Trading partner customizations (framework ready)");
    println!("  ✅ Hierarchical segment validation");
    println!("  ✅ Element ID cross-referencing");
    println!("  ✅ Comprehensive validation reporting");
    println!("  ✅ Performance metrics");

    println!("\n✨ Schema Engine successfully demonstrates comprehensive");
    println!("   version-aware EDI parsing with element ID validation!");

    Ok(())
}

/// Additional utility functions for demonstration
fn demonstrate_element_mapping() {
    println!("\n🗺️  Element ID Mapping Examples:");
    println!("   CUR01 = Element ID 98 (Entity Identifier Code)");
    println!("   CUR02 = Element ID 100 (Currency Code)"); 
    println!("   REF01 = Element ID 128 (Reference Identification Qualifier)");
    println!("   REF02 = Element ID 127 (Reference Identification)");
    println!("   PO102 = Element ID 330 (Quantity Ordered)");
    println!("   PO103 = Element ID 355 (Unit or Basis for Measurement Code)");
    println!("   PO104 = Element ID 212 (Unit Price)");
}

/// Show schema capabilities for different transaction types
fn show_supported_transactions() {
    let engine = SchemaEngine::new();
    
    println!("\n📜 Supported Transaction Types by Version:");
    for version in [X12Version::V4010, X12Version::V5010, X12Version::V8010] {
        println!("  {:?}:", version);
        let types = engine.get_available_transaction_types(&version);
        for tx_type in types {
            if let Some(schema) = engine.get_transaction_schema(&tx_type, &version) {
                println!("    {} - {}", tx_type, schema.name);
            }
        }
    }
}
