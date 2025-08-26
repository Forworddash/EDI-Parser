use edi_parser::models::X12Version;
use edi_parser::models::segment_definition::{
    SegmentRegistry, SegmentDefinition, ElementDefinition, 
    ElementDataType, ElementRequirement
};
use edi_parser::models::schema_engine::{
    TradingPartnerAgreement, SchemaCustomization, CustomizationType
};
use edi_parser::parsers::schema_validating_parser::{
    SchemaValidatingParser, ParsingConfig, ValidationLevel
};
use edi_parser::error::EdiError;

fn main() -> Result<(), EdiError> {
    println!("🛠️  EDI Schema Extension Example");
    println!("=================================\n");

    // Example 1: Adding a new TAX segment to the registry
    println!("📦 Step 1: Adding a new TAX segment");
    let mut segment_registry = SegmentRegistry::new();
    add_tax_segment(&mut segment_registry);
    
    // Verify the TAX segment was added
    if let Some(tax_def) = segment_registry.get_definition(&X12Version::V4010, "TAX") {
        println!("✅ TAX segment added successfully!");
        println!("   - Segment ID: {}", tax_def.id);
        println!("   - Name: {}", tax_def.name);
        println!("   - Elements: {}", tax_def.elements.len());
        
        for (i, element) in tax_def.elements.iter().enumerate() {
            println!("   - TAX{:02} = Element ID {} ({})", 
                i + 1, element.element_id, element.name);
        }
    }

    println!("\n📋 Step 2: Creating partner-specific customizations");
    
    // Example: Home Depot specific requirements for 850 Purchase Orders
    let home_depot_agreement = create_home_depot_customizations();
    
    println!("✅ Home Depot customizations created:");
    for custom in &home_depot_agreement.customizations {
        println!("   - {}: {}", custom.segment_id, custom.description);
    }

    // Example: Costco specific requirements
    let costco_agreement = create_costco_customizations();
    
    println!("✅ Costco customizations created:");
    for custom in &costco_agreement.customizations {
        println!("   - {}: {}", custom.segment_id, custom.description);
    }

    println!("\n🧪 Step 3: Testing with sample EDI data");
    
    // Sample EDI with TAX segment
    let sample_edi_with_tax = r#"ISA*00*          *00*          *ZZ*SENDER_ID      *ZZ*RECEIVER_ID    *210308*1430*U*00401*000000001*0*P*>~
GS*PO*SENDER_ID*RECEIVER_ID*20210308*1430*1*X*004010~
ST*850*0001~
BEG*00*SA*PO123456**20210308~
CUR*BY*USD~
REF*DP*DEPT001~
PO1*001*100*EA*25.50~
TAX*ST*2.55*10.00~
SE*8*0001~
GE*1*1~
IEA*1*000000001~"#;

    // Test parsing with different configurations
    test_parsing_example("Basic parsing", &sample_edi_with_tax)?;

    println!("\n🎯 Step 4: Demonstrating element tracking");
    demonstrate_element_tracking(&sample_edi_with_tax)?;

    println!("\n🎉 Schema Extension Example Complete!");
    println!("✨ You can now:");
    println!("   - Add custom segments to any transaction type");
    println!("   - Create partner-specific validation rules");
    println!("   - Track elements across the entire document");
    println!("   - Support multiple X12 versions with different rules");

    Ok(())
}

/// Add a custom TAX segment to the registry
fn add_tax_segment(registry: &mut SegmentRegistry) {
    let tax_definition = SegmentDefinition {
        id: "TAX".to_string(),
        name: "Tax Reference".to_string(),
        elements: vec![
            ElementDefinition {
                element_id: 963,  // Tax Type Code
                name: "Tax Type Code".to_string(),
                data_type: ElementDataType::ID,
                min_length: Some(2),
                max_length: Some(3),
                requirement: ElementRequirement::Mandatory,
                description: "Code identifying the type of tax".to_string(),
                valid_codes: Some(vec![
                    "ST".to_string(), // State Tax
                    "LT".to_string(), // Local Tax
                    "FT".to_string(), // Federal Tax
                    "GS".to_string(), // Goods and Services Tax
                    "VA".to_string(), // Value Added Tax
                ]),
            },
            ElementDefinition {
                element_id: 782,  // Monetary Amount
                name: "Tax Amount".to_string(),
                data_type: ElementDataType::R,
                min_length: Some(1),
                max_length: Some(18),
                requirement: ElementRequirement::Mandatory,
                description: "Monetary amount of the tax".to_string(),
                valid_codes: None,
            },
            ElementDefinition {
                element_id: 954,  // Percent
                name: "Tax Rate".to_string(),
                data_type: ElementDataType::R,
                min_length: Some(1),
                max_length: Some(10),
                requirement: ElementRequirement::Optional,
                description: "Tax rate expressed as a percentage".to_string(),
                valid_codes: None,
            },
        ],
        min_usage: 0,
        max_usage: Some(10),
        description: "To specify tax information".to_string(),
    };

    // Register for all versions
    registry.register_segment(X12Version::V4010, tax_definition.clone());
    registry.register_segment(X12Version::V5010, tax_definition.clone());
    registry.register_segment(X12Version::V8010, tax_definition);
    
    println!("   📝 TAX segment registered for all X12 versions");
}

/// Create Home Depot specific customizations
fn create_home_depot_customizations() -> TradingPartnerAgreement {
    TradingPartnerAgreement {
        partner_id: "HOME_DEPOT_US".to_string(),
        agreement_name: "Home Depot EDI Requirements v3.2".to_string(),
        customizations: vec![
            // Home Depot requires REF segment with vendor number
            SchemaCustomization {
                segment_id: "REF".to_string(),
                element_position: None,
                customization_type: CustomizationType::MakeMandatory,
                description: "Home Depot requires REF segment with vendor identification".to_string(),
            },
            // Home Depot only accepts specific reference qualifiers
            SchemaCustomization {
                segment_id: "REF".to_string(),
                element_position: Some(0), // REF01
                customization_type: CustomizationType::RestrictValidCodes(vec![
                    "VN".to_string(), // Vendor Number (required)
                    "DP".to_string(), // Department Number
                    "IA".to_string(), // Internal Vendor Number
                ]),
                description: "Home Depot accepts only VN, DP, and IA reference qualifiers".to_string(),
            },
            // Home Depot requires TAX segment for taxable items
            SchemaCustomization {
                segment_id: "TAX".to_string(),
                element_position: None,
                customization_type: CustomizationType::MakeMandatory,
                description: "Home Depot requires tax information for compliance".to_string(),
            },
        ],
    }
}

/// Create Costco specific customizations
fn create_costco_customizations() -> TradingPartnerAgreement {
    TradingPartnerAgreement {
        partner_id: "COSTCO".to_string(),
        agreement_name: "Costco Wholesale EDI Standards".to_string(),
        customizations: vec![
            // Costco requires minimum order quantities
            SchemaCustomization {
                segment_id: "PO1".to_string(),
                element_position: Some(1), // PO102 - Quantity
                customization_type: CustomizationType::ChangeLengthConstraints { 
                    min: Some(3), // Minimum 3 digits (100+ quantity)
                    max: Some(10) 
                },
                description: "Costco requires minimum order quantities of 100+ units".to_string(),
            },
            // Costco only accepts case quantities
            SchemaCustomization {
                segment_id: "PO1".to_string(),
                element_position: Some(2), // PO103 - Unit of Measure
                customization_type: CustomizationType::RestrictValidCodes(vec![
                    "CS".to_string(), // Case
                    "PL".to_string(), // Pallet
                ]),
                description: "Costco only accepts case and pallet quantities".to_string(),
            },
        ],
    }
}

/// Test parsing with different configurations
fn test_parsing_example(test_name: &str, edi_data: &str) -> Result<(), EdiError> {
    println!("\n   🧪 Testing: {}", test_name);
    
    let config = ParsingConfig {
        version: X12Version::V4010,
        trading_partner_id: None,
        validation_level: ValidationLevel::Standard,
        continue_on_error: true,
        collect_validation_details: true,
    };

    let mut parser = SchemaValidatingParser::new(config);
    
    match parser.parse_with_schema(edi_data) {
        Ok(result) => {
            println!("      ✅ Parsing Status: {:?}", result.status);
            
            let segment_count = result.segments.len();
            let transaction_count = result.transaction_summaries.len();
            println!("      📊 Found {} segments in {} transaction(s)", segment_count, transaction_count);
            
            for (i, summary) in result.transaction_summaries.iter().enumerate() {
                println!("         Transaction {}: {} with {} segments", 
                    i + 1, summary.transaction_type, summary.segment_count);
            }
        }
        Err(e) => {
            println!("      ❌ Parsing failed: {}", e);
        }
    }
    
    Ok(())
}

/// Demonstrate element tracking capabilities
fn demonstrate_element_tracking(edi_data: &str) -> Result<(), EdiError> {
    println!("   🔍 Demonstrating element tracking...");
    
    let config = ParsingConfig::default();
    let mut parser = SchemaValidatingParser::new(config);
    
    if let Ok(result) = parser.parse_with_schema(edi_data) {
        // Look for specific elements in the parsed data
        println!("      💰 Currency elements found:");
        for validated_segment in &result.segments {
            let segment = &validated_segment.segment;
            if segment.id == "CUR" && !segment.elements.is_empty() {
                println!("         CUR01 = '{}' (Currency Entity)", segment.elements[0]);
                if segment.elements.len() > 1 {
                    println!("         CUR02 = '{}' (Currency Code)", segment.elements[1]);
                }
            }
        }
        
        println!("      📋 Reference elements found:");
        for validated_segment in &result.segments {
            let segment = &validated_segment.segment;
            if segment.id == "REF" && !segment.elements.is_empty() {
                println!("         REF01 = '{}' (Reference Qualifier)", segment.elements[0]);
                if segment.elements.len() > 1 && !segment.elements[1].is_empty() {
                    println!("         REF02 = '{}' (Reference ID)", segment.elements[1]);
                }
            }
        }
        
        println!("      🧾 Tax elements found:");
        for validated_segment in &result.segments {
            let segment = &validated_segment.segment;
            if segment.id == "TAX" && !segment.elements.is_empty() {
                println!("         TAX01 = '{}' (Tax Type)", segment.elements[0]);
                if segment.elements.len() > 1 {
                    println!("         TAX02 = '{}' (Tax Amount)", segment.elements[1]);
                }
                if segment.elements.len() > 2 && !segment.elements[2].is_empty() {
                    println!("         TAX03 = '{}' (Tax Rate)", segment.elements[2]);
                }
            }
        }
    }
    
    Ok(())
}
