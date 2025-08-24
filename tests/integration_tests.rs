// use edi_parser::{
//     parsers::x12::X12Parser,
//     models::{InterchangeControl, Segment, Transaction},
//     error::EdiError,
// };
use edi_parser::{
    X12Parser,
    EdiParser,  // Add this import for the trait
    InterchangeControl, 
    Segment, 
    Transaction,
    EdiError,
};
use std::fs;

#[test]
fn test_basic_x12_parsing() {
    let parser = X12Parser::default();
    let input = "ISA*00*          *00*          *01*SENDERID     *01*RECEIVERID   *230101*1253*U*00401*000000001*0*T*>~GS*IN*SENDERID*RECEIVERID*20230101*1253*1*X*004010~ST*810*0001~BIG*20230101*INV-001~SE*4*0001~GE*1*1~IEA*1*000000001~";
    
    let result = parser.parse(input);
    assert!(result.is_ok());
    
    let interchange = result.unwrap();
    assert_eq!(interchange.isa_segment.id, "ISA");
    assert_eq!(interchange.functional_groups.len(), 1);
    assert_eq!(interchange.functional_groups[0].transactions.len(), 1);
}

#[test]
fn test_810_invoice_parsing() {
    let content = fs::read_to_string("tests/test_files/sample_810.edi")
        .expect("Failed to read test file");
    
    let parser = X12Parser::default();
    let result = parser.parse(&content);
    
    assert!(result.is_ok());
    
    let interchange = result.unwrap();
    assert_eq!(interchange.isa_segment.elements[5], "SENDERID");
    assert_eq!(interchange.isa_segment.elements[6], "RECEIVERID");
    
    let transaction = &interchange.functional_groups[0].transactions[0];
    assert_eq!(transaction.transaction_set_id, "810");
    assert_eq!(transaction.control_number, "0001");
}

#[test]
fn test_850_purchase_order_parsing() {
    let content = fs::read_to_string("tests/test_files/sample_850.edi")
        .expect("Failed to read test file");
    
    let parser = X12Parser::default();
    let result = parser.parse(&content);
    
    assert!(result.is_ok());
    
    let interchange = result.unwrap();
    let transaction = &interchange.functional_groups[0].transactions[0];
    assert_eq!(transaction.transaction_set_id, "850");
    
    // Verify specific segments exist
    let has_beg = transaction.segments.iter().any(|s| s.id == "BEG");
    let has_po1 = transaction.segments.iter().any(|s| s.id == "PO1");
    assert!(has_beg);
    assert!(has_po1);
}

#[test]
fn test_invalid_input() {
    let parser = X12Parser::default();
    let result = parser.parse("Invalid*Data*Here");
    
    assert!(result.is_err());
}

#[test]
fn test_validation() {
    let parser = X12Parser::default();
    let input = "ISA*00*          *00*          *01*SENDERID     *01*RECEIVERID   *230101*1253*U*00401*000000001*0*T*>~GS*IN*SENDERID*RECEIVERID*20230101*1253*1*X*004010~ST*810*0001~BIG*20230101*INV-001~SE*4*0001~GE*1*1~IEA*1*000000001~";
    
    let interchange = parser.parse(input).unwrap();
    let validation_result = parser.validate(&interchange);
    
    assert!(validation_result.is_ok());
}