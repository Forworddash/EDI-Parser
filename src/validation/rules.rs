use super::schema::*;
use super::validator::*;

/// Pre-defined validation rules for standard EDI transaction sets
pub struct StandardRules;

impl StandardRules {
    /// Create schema for 850 Purchase Order version 4010
    pub fn purchase_order_850_v4010() -> EdiSchema {
        SchemaBuilder::new("4010", "850", "Purchase Order")
            // ST - Transaction Set Header
            .add_segment(
                SegmentBuilder::new("ST", "Transaction Set Header", SegmentUsage::Required, 1)
                    .sequence(10)
                    .max_repetitions(1)
                    .add_element(
                        ElementBuilder::new(1, "143", "Transaction Set Identifier Code", ElementUsage::Required, DataType::ID)
                            .length(3, 3)
                            .valid_codes(vec!["850".to_string()])
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(2, "329", "Transaction Set Control Number", ElementUsage::Required, DataType::AN)
                            .length(4, 9)
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(3, "1705", "Implementation Convention Reference", ElementUsage::Optional, DataType::AN)
                            .length(1, 35)
                            .build()
                    )
                    .build()
            )
            // BEG - Beginning Segment for Purchase Order
            .add_segment(
                SegmentBuilder::new("BEG", "Beginning Segment for Purchase Order", SegmentUsage::Required, 2)
                    .sequence(20)
                    .max_repetitions(1)
                    .add_element(
                        ElementBuilder::new(1, "353", "Transaction Set Purpose Code", ElementUsage::Required, DataType::ID)
                            .length(2, 2)
                            .valid_codes(vec!["00".to_string(), "01".to_string(), "04".to_string(), "05".to_string()])
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(2, "92", "Purchase Order Type Code", ElementUsage::Required, DataType::ID)
                            .length(2, 2)
                            .valid_codes(vec!["SA".to_string(), "KA".to_string(), "NE".to_string(), "RL".to_string()])
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(3, "324", "Purchase Order Number", ElementUsage::Required, DataType::AN)
                            .length(1, 22)
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(4, "328", "Release Number", ElementUsage::Optional, DataType::AN)
                            .length(1, 30)
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(5, "373", "Date", ElementUsage::Required, DataType::DT)
                            .length(8, 8)
                            .format("CCYYMMDD")
                            .build()
                    )
                    .build()
            )
            // N1 - Name
            .add_segment(
                SegmentBuilder::new("N1", "Name", SegmentUsage::Optional, 3)
                    .sequence(30)
                    .max_repetitions(200)
                    .add_element(
                        ElementBuilder::new(1, "98", "Entity Identifier Code", ElementUsage::Required, DataType::ID)
                            .length(2, 3)
                            .valid_codes(vec![
                                "BY".to_string(), "ST".to_string(), "VN".to_string(), 
                                "SU".to_string(), "BT".to_string(), "RI".to_string()
                            ])
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(2, "93", "Name", ElementUsage::Optional, DataType::AN)
                            .length(1, 60)
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(3, "66", "Identification Code Qualifier", ElementUsage::Optional, DataType::ID)
                            .length(1, 2)
                            .valid_codes(vec!["1".to_string(), "9".to_string(), "92".to_string()])
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(4, "67", "Identification Code", ElementUsage::Optional, DataType::AN)
                            .length(2, 80)
                            .build()
                    )
                    .build()
            )
            // PO1 - Baseline Item Data
            .add_segment(
                SegmentBuilder::new("PO1", "Baseline Item Data", SegmentUsage::Optional, 4)
                    .sequence(100)
                    .max_repetitions(100000)
                    .add_element(
                        ElementBuilder::new(1, "350", "Assigned Identification", ElementUsage::Optional, DataType::AN)
                            .length(1, 20)
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(2, "330", "Quantity Ordered", ElementUsage::Optional, DataType::R)
                            .length(1, 15)
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(3, "355", "Unit or Basis for Measurement Code", ElementUsage::Optional, DataType::ID)
                            .length(2, 2)
                            .valid_codes(vec![
                                "EA".to_string(), "CA".to_string(), "LB".to_string(), 
                                "KG".to_string(), "PC".to_string(), "BX".to_string()
                            ])
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(4, "212", "Unit Price", ElementUsage::Optional, DataType::R)
                            .length(1, 17)
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(5, "639", "Basis of Unit Price Code", ElementUsage::Optional, DataType::ID)
                            .length(2, 2)
                            .valid_codes(vec!["PE".to_string(), "PP".to_string(), "PK".to_string()])
                            .build()
                    )
                    .build()
            )
            // CTT - Transaction Totals
            .add_segment(
                SegmentBuilder::new("CTT", "Transaction Totals", SegmentUsage::Optional, 5)
                    .sequence(200)
                    .max_repetitions(1)
                    .add_element(
                        ElementBuilder::new(1, "354", "Number of Line Items", ElementUsage::Required, DataType::N)
                            .length(1, 6)
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(2, "347", "Hash Total", ElementUsage::Optional, DataType::R)
                            .length(1, 10)
                            .build()
                    )
                    .build()
            )
            // SE - Transaction Set Trailer
            .add_segment(
                SegmentBuilder::new("SE", "Transaction Set Trailer", SegmentUsage::Required, 6)
                    .sequence(210)
                    .max_repetitions(1)
                    .add_element(
                        ElementBuilder::new(1, "96", "Number of Included Segments", ElementUsage::Required, DataType::N)
                            .length(1, 10)
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(2, "329", "Transaction Set Control Number", ElementUsage::Required, DataType::AN)
                            .length(4, 9)
                            .build()
                    )
                    .build()
            )
            .build()
    }
    
    /// Create schema for 850 Purchase Order version 5010 (with additional segments)
    pub fn purchase_order_850_v5010() -> EdiSchema {
        let mut schema = Self::purchase_order_850_v4010();
        schema.version = "5010".to_string();
        
        // Add new segments for 5010
        schema.segments.push(
            SegmentBuilder::new("PWK", "Paperwork", SegmentUsage::Optional, 7)
                .sequence(35)
                .max_repetitions(25)
                .add_element(
                    ElementBuilder::new(1, "755", "Report Type Code", ElementUsage::Required, DataType::ID)
                        .length(2, 2)
                        .valid_codes(vec!["10".to_string(), "25".to_string(), "35".to_string()])
                        .build()
                )
                .add_element(
                    ElementBuilder::new(2, "756", "Report Transmission Code", ElementUsage::Optional, DataType::ID)
                        .length(1, 2)
                        .valid_codes(vec!["BY".to_string(), "CF".to_string(), "EL".to_string()])
                        .build()
                )
                .add_element(
                    ElementBuilder::new(3, "757", "Report Copies Needed", ElementUsage::Optional, DataType::N)
                        .length(1, 2)
                        .build()
                )
                .build()
        );
        
        schema
    }
    
    /// Create schema for 810 Invoice version 4010
    pub fn invoice_810_v4010() -> EdiSchema {
        SchemaBuilder::new("4010", "810", "Invoice")
            // ST - Transaction Set Header
            .add_segment(
                SegmentBuilder::new("ST", "Transaction Set Header", SegmentUsage::Required, 1)
                    .sequence(10)
                    .max_repetitions(1)
                    .add_element(
                        ElementBuilder::new(1, "143", "Transaction Set Identifier Code", ElementUsage::Required, DataType::ID)
                            .length(3, 3)
                            .valid_codes(vec!["810".to_string()])
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(2, "329", "Transaction Set Control Number", ElementUsage::Required, DataType::AN)
                            .length(4, 9)
                            .build()
                    )
                    .build()
            )
            // BIG - Beginning Segment for Invoice
            .add_segment(
                SegmentBuilder::new("BIG", "Beginning Segment for Invoice", SegmentUsage::Required, 2)
                    .sequence(20)
                    .max_repetitions(1)
                    .add_element(
                        ElementBuilder::new(1, "373", "Date", ElementUsage::Required, DataType::DT)
                            .length(8, 8)
                            .format("CCYYMMDD")
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(2, "76", "Invoice Number", ElementUsage::Required, DataType::AN)
                            .length(1, 22)
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(3, "373", "Date", ElementUsage::Optional, DataType::DT)
                            .length(8, 8)
                            .format("CCYYMMDD")
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(4, "324", "Purchase Order Number", ElementUsage::Optional, DataType::AN)
                            .length(1, 22)
                            .build()
                    )
                    .build()
            )
            // SE - Transaction Set Trailer
            .add_segment(
                SegmentBuilder::new("SE", "Transaction Set Trailer", SegmentUsage::Required, 3)
                    .sequence(200)
                    .max_repetitions(1)
                    .add_element(
                        ElementBuilder::new(1, "96", "Number of Included Segments", ElementUsage::Required, DataType::N)
                            .length(1, 10)
                            .build()
                    )
                    .add_element(
                        ElementBuilder::new(2, "329", "Transaction Set Control Number", ElementUsage::Required, DataType::AN)
                            .length(4, 9)
                            .build()
                    )
                    .build()
            )
            .build()
    }
}
