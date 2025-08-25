use edi_parser::{X12ParserBuilder, EdiParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Use default parser
    let default_parser = edi_parser::X12Parser::default();
    
    // Example 2: Use builder pattern for custom configuration
    let custom_parser = X12ParserBuilder::new()
        .element_separator('*')
        .segment_separator('~')
        .sub_element_separator('>')
        .strict_validation(true)
        .trim_whitespace(true)
        .build()?;

    // Sample EDI data
    let edi_data = r#"ISA*00*          *00*          *01*SENDERID     *01*RECEIVERID   *230101*1253*U*00401*000000001*0*T*>~
GS*IN*SENDERID*RECEIVERID*20230101*1253*1*X*004010~
ST*810*0001~
BIG*20230101*INV-001**20230115~
N1*ST*ABC Company*92*12345~
IT1*1*10*EA*25.00**BP*ITEM-001*VP*Vendor Part 001~
TDS*250.00~
CTT*1~
SE*8*0001~
GE*1*1~
IEA*1*000000001~"#;

    // Parse with default parser
    let interchange = default_parser.parse(edi_data)?;
    println!("Parsed interchange with {} functional groups", 
             interchange.functional_groups.len());

    // Validate the structure
    default_parser.validate(&interchange)?;
    println!("Validation passed!");

    // Parse with custom parser
    let interchange2 = custom_parser.parse(edi_data)?;
    custom_parser.validate(&interchange2)?;
    println!("Custom parser also worked!");

    Ok(())
}
