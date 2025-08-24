use edi_parser::{
    X12Parser,
    EdiParser,
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
    
    assert!(result.is_ok(), "Parse failed: {:?}", result.err());
    
    let interchange = result.unwrap();
    
    // Check that we have the right sender/receiver (trimmed)
    assert_eq!(interchange.isa_segment.elements[4], "01"); // Authorization qualifier
    assert_eq!(interchange.isa_segment.elements[5], "SENDERID"); // Sender ID (trimmed)
    assert_eq!(interchange.isa_segment.elements[6], "01"); // Receiver qualifier
    assert_eq!(interchange.isa_segment.elements[7], "RECEIVERID"); // Receiver ID (trimmed)
    
    assert!(interchange.functional_groups.len() > 0, "No functional groups found");
    assert!(interchange.functional_groups[0].transactions.len() > 0, "No transactions found");
    
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
    
    assert!(result.is_ok(), "Parse failed: {:?}", result.err());
    
    let interchange = result.unwrap();
    
    // Basic structure validation
    assert!(interchange.functional_groups.len() > 0, "No functional groups found");
    assert!(interchange.functional_groups[0].transactions.len() > 0, "No transactions found");
    
    let transaction = &interchange.functional_groups[0].transactions[0];
    assert_eq!(transaction.transaction_set_id, "850");
    
    // Verify specific segments exist by ID
    let segment_ids: Vec<String> = transaction.segments.iter().map(|s| s.id.clone()).collect();
    assert!(segment_ids.contains(&"ST".to_string()));
    assert!(segment_ids.contains(&"BEG".to_string()));
    assert!(segment_ids.contains(&"N1".to_string()));
    assert!(segment_ids.contains(&"PO1".to_string()));
    assert!(segment_ids.contains(&"CTT".to_string()));
    assert!(segment_ids.contains(&"SE".to_string()));
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