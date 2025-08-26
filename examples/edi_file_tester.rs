use edi_parser::models::X12Version;
use edi_parser::parsers::schema_validating_parser::{
    SchemaValidatingParser, ParsingConfig, ValidationLevel
};
use edi_parser::parsers::x12::X12Parser;
use edi_parser::parsers::EdiParser;
use edi_parser::error::EdiError;
use std::env;
use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 EDI File Tester - Test Any EDI File");
    println!("======================================\n");

    // Get file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let file_path = &args[1];
    
    // Check if file exists
    if !Path::new(file_path).exists() {
        eprintln!("❌ Error: File '{}' does not exist!", file_path);
        return Ok(());
    }

    println!("📁 Testing file: {}", file_path);

    // Read the EDI file
    let edi_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading file: {}", e);
            return Ok(());
        }
    };

    println!("📊 File size: {} bytes", edi_content.len());
    println!("📋 First 200 characters preview:");
    println!("   {}\n", &edi_content.chars().take(200).collect::<String>());

    // Test with multiple configurations
    test_basic_parsing(&edi_content)?;
    test_schema_validation(&edi_content)?;
    test_element_tracking(&edi_content)?;
    test_different_versions(&edi_content)?;

    println!("🎉 Testing Complete!");
    println!("\n💡 Tips:");
    println!("   - Use --help for more options");
    println!("   - Check validation results for compliance issues");
    println!("   - Try different X12 versions if parsing fails");

    Ok(())
}

fn print_usage() {
    println!("Usage: cargo run --example edi_file_tester <path_to_edi_file>");
    println!();
    println!("Examples:");
    println!("   cargo run --example edi_file_tester tests/test_files/sample_850.edi");
    println!("   cargo run --example edi_file_tester \"C:\\path\\to\\your\\file.edi\"");
    println!("   cargo run --example edi_file_tester \"C:\\path\\to\\your\\file.x12\"");
    println!();
    println!("Supported file types: .edi, .x12, .txt, or any text file with EDI content");
}

fn test_basic_parsing(edi_content: &str) -> Result<(), EdiError> {
    println!("🔍 Test 1: Basic X12 Parsing");
    println!("============================");

    let start_time = Instant::now();
    let mut parser = X12Parser::new();
    
    match parser.parse(edi_content) {
        Ok(interchange) => {
            let duration = start_time.elapsed();
            println!("✅ Basic parsing successful!");
            println!("   ⏱️  Parsing time: {:?}", duration);
            
            // Extract sender and receiver from ISA segment
            if interchange.isa_segment.elements.len() >= 8 {
                println!("   📊 Sender ID: {}", interchange.isa_segment.elements[5].trim());
                println!("   📊 Receiver ID: {}", interchange.isa_segment.elements[7].trim());
            }
            
            println!("   📊 Functional Groups: {}", interchange.functional_groups.len());
            
            let mut total_transactions = 0;
            for (i, group) in interchange.functional_groups.iter().enumerate() {
                total_transactions += group.transactions.len();
                println!("   📋 Group {}: {} transaction(s)", i + 1, group.transactions.len());
                
                for (j, transaction) in group.transactions.iter().enumerate() {
                    println!("      📄 Transaction {}: Type {} with {} segments", 
                        j + 1, transaction.transaction_set_id, transaction.segments.len());
                }
            }
            println!("   📊 Total Transactions: {}\n", total_transactions);
        }
        Err(e) => {
            println!("❌ Basic parsing failed: {}", e);
            println!("   💡 This might be due to:");
            println!("      - Malformed EDI structure");
            println!("      - Missing required segments");
            println!("      - Invalid segment terminators");
            println!("      - Encoding issues\n");
        }
    }

    Ok(())
}

fn test_schema_validation(edi_content: &str) -> Result<(), EdiError> {
    println!("🔬 Test 2: Schema Validation (Standard Level)");
    println!("============================================");

    let config = ParsingConfig {
        version: X12Version::V4010,
        trading_partner_id: None,
        validation_level: ValidationLevel::Standard,
        continue_on_error: true,
        collect_validation_details: true,
    };

    let start_time = Instant::now();
    let mut parser = SchemaValidatingParser::new(config);
    
    match parser.parse_with_schema(edi_content) {
        Ok(result) => {
            let duration = start_time.elapsed();
            println!("✅ Schema validation completed!");
            println!("   ⏱️  Validation time: {:?}", duration);
            println!("   📊 Parsing Status: {:?}", result.status);
            println!("   📊 Segments Found: {}", result.segments.len());
            println!("   📊 Transaction Summaries: {}", result.transaction_summaries.len());
            
            // Count validation results by type
            let errors = result.validation_results.iter()
                .filter(|r| matches!(r.result_type, edi_parser::models::schema_engine::ValidationResultType::Error))
                .count();
            let warnings = result.validation_results.iter()
                .filter(|r| matches!(r.result_type, edi_parser::models::schema_engine::ValidationResultType::Warning))
                .count();
            let info = result.validation_results.iter()
                .filter(|r| matches!(r.result_type, edi_parser::models::schema_engine::ValidationResultType::Info))
                .count();

            println!("   📊 Validation Results: {} errors, {} warnings, {} info", errors, warnings, info);

            // Show transaction summaries
            for (i, summary) in result.transaction_summaries.iter().enumerate() {
                println!("   📄 Transaction {}: {} ({} segments)", 
                    i + 1, summary.transaction_type, summary.segment_count);
            }

            // Show sample validation issues
            if errors > 0 {
                println!("\n   🚨 Sample Errors (first 3):");
                for (i, validation_result) in result.validation_results.iter()
                    .filter(|r| matches!(r.result_type, edi_parser::models::schema_engine::ValidationResultType::Error))
                    .take(3)
                    .enumerate() {
                    println!("      {}. {}", i + 1, validation_result.message);
                }
            }

            if warnings > 0 {
                println!("\n   ⚠️  Sample Warnings (first 3):");
                for (i, validation_result) in result.validation_results.iter()
                    .filter(|r| matches!(r.result_type, edi_parser::models::schema_engine::ValidationResultType::Warning))
                    .take(3)
                    .enumerate() {
                    println!("      {}. {}", i + 1, validation_result.message);
                }
            }

            println!();
        }
        Err(e) => {
            println!("❌ Schema validation failed: {}", e);
            println!("   💡 This might indicate:");
            println!("      - Unknown transaction types");
            println!("      - Schema compliance issues");
            println!("      - Version mismatch\n");
        }
    }

    Ok(())
}

fn test_element_tracking(edi_content: &str) -> Result<(), EdiError> {
    println!("🔍 Test 3: Element ID Tracking");
    println!("==============================");

    let config = ParsingConfig::default();
    let mut parser = SchemaValidatingParser::new(config);
    
    match parser.parse_with_schema(edi_content) {
        Ok(result) => {
            println!("✅ Element tracking analysis:");
            
            // Track common elements
            let mut element_stats = std::collections::HashMap::new();
            
            for validated_segment in &result.segments {
                let segment = &validated_segment.segment;
                
                // Track specific elements we know about
                match segment.id.as_str() {
                    "CUR" => {
                        if segment.elements.len() >= 2 {
                            println!("   💰 Currency: CUR01='{}' (Element 98), CUR02='{}' (Element 100)", 
                                segment.elements[0], segment.elements[1]);
                            *element_stats.entry("Currency").or_insert(0) += 1;
                        }
                    }
                    "REF" => {
                        if segment.elements.len() >= 2 {
                            println!("   📋 Reference: REF01='{}' (Element 128), REF02='{}' (Element 127)", 
                                segment.elements[0], segment.elements[1]);
                            *element_stats.entry("Reference").or_insert(0) += 1;
                        }
                    }
                    "BEG" => {
                        if segment.elements.len() >= 3 {
                            println!("   📄 Purchase Order: BEG01='{}' (Element 353), BEG03='{}' (Element 324)", 
                                segment.elements[0], segment.elements[2]);
                            *element_stats.entry("Purchase Order").or_insert(0) += 1;
                        }
                    }
                    "PO1" => {
                        if segment.elements.len() >= 4 {
                            println!("   📦 Item: PO101='{}', PO102='{}' (Element 330), PO104='{}' (Element 212)", 
                                segment.elements[0], segment.elements[1], segment.elements[3]);
                            *element_stats.entry("Line Item").or_insert(0) += 1;
                        }
                    }
                    "N1" => {
                        if segment.elements.len() >= 2 {
                            println!("   🏢 Name: N101='{}' (Element 98), N102='{}' (Element 93)", 
                                segment.elements[0], segment.elements[1]);
                            *element_stats.entry("Name/Address").or_insert(0) += 1;
                        }
                    }
                    _ => {}
                }
            }

            println!("\n   📊 Element Summary:");
            for (element_type, count) in element_stats {
                println!("      - {} segments: {}", element_type, count);
            }

            println!();
        }
        Err(e) => {
            println!("❌ Element tracking failed: {}", e);
            println!();
        }
    }

    Ok(())
}

fn test_different_versions(edi_content: &str) -> Result<(), EdiError> {
    println!("📋 Test 4: Multi-Version Compatibility");
    println!("======================================");

    let versions = vec![
        ("004010", X12Version::V4010),
        ("005010", X12Version::V5010),
        ("008010", X12Version::V8010),
    ];

    for (version_name, version) in versions {
        println!("   Testing X12 version {}:", version_name);
        
        let config = ParsingConfig {
            version,
            trading_partner_id: None,
            validation_level: ValidationLevel::Basic,
            continue_on_error: true,
            collect_validation_details: false,
        };

        let mut parser = SchemaValidatingParser::new(config);
        
        match parser.parse_with_schema(edi_content) {
            Ok(result) => {
                let errors = result.validation_results.iter()
                    .filter(|r| matches!(r.result_type, edi_parser::models::schema_engine::ValidationResultType::Error))
                    .count();
                
                if errors == 0 {
                    println!("      ✅ Compatible - {} segments parsed", result.segments.len());
                } else {
                    println!("      ⚠️  Partial - {} segments, {} errors", result.segments.len(), errors);
                }
            }
            Err(_) => {
                println!("      ❌ Incompatible");
            }
        }
    }

    println!();
    Ok(())
}
