use super::{Segment, TransactionType};

#[derive(Debug, Clone)]
pub struct PartyLoop {
    pub n1_segment: Segment,           // Name
    pub n2_segments: Vec<Segment>,     // Additional Names
    pub n3_segments: Vec<Segment>,     // Address Lines
    pub n4_segment: Option<Segment>,   // City/State/Zip
    pub per_segments: Vec<Segment>,    // Contact Information
}

#[derive(Debug, Clone)]
pub struct LineItemLoop {
    pub po1_segment: Segment,          // Purchase Order Line Item
    pub pid_segments: Vec<Segment>,    // Product Description
    pub sac_segments: Vec<Segment>,    // Service/Promotion Allowance/Charge
    pub dtm_segments: Vec<Segment>,    // Date/Time References
}

#[derive(Debug, Clone)]
pub struct PurchaseOrder850 {
    pub transaction_type: TransactionType,
    pub header_segments: Vec<Segment>,     // BEG, DTM, REF, etc.
    pub party_loops: Vec<PartyLoop>,       // N1 loops
    pub line_item_loops: Vec<LineItemLoop>, // PO1 loops
    pub summary_segments: Vec<Segment>,    // CTT, SE
}

impl PurchaseOrder850 {
    pub fn parse_from_transaction(transaction: &super::Transaction) -> Result<Self, String> {
        if !matches!(transaction.transaction_type, TransactionType::PurchaseOrder850) {
            return Err("Not a valid 850 transaction".to_string());
        }

        let mut header_segments = Vec::new();
        let mut party_loops = Vec::new();
        let mut line_item_loops = Vec::new();
        let mut summary_segments = Vec::new();

        let mut segments = transaction.segments.iter().peekable();

        // Parse header segments (everything before first N1 or PO1)
        while let Some(segment) = segments.peek() {
            match segment.id.as_str() {
                "N1" | "PO1" => break,
                "CTT" | "SE" => {
                    summary_segments.push(segments.next().unwrap().clone());
                }
                _ => {
                    header_segments.push(segments.next().unwrap().clone());
                }
            }
        }

        // Parse party loops (N1 groups)
        while let Some(segment) = segments.peek() {
            if segment.id != "N1" {
                break;
            }

            let mut party_loop = PartyLoop {
                n1_segment: segments.next().unwrap().clone(),
                n2_segments: Vec::new(),
                n3_segments: Vec::new(),
                n4_segment: None,
                per_segments: Vec::new(),
            };

            // Parse additional party segments
            while let Some(segment) = segments.peek() {
                match segment.id.as_str() {
                    "N2" => party_loop.n2_segments.push(segments.next().unwrap().clone()),
                    "N3" => party_loop.n3_segments.push(segments.next().unwrap().clone()),
                    "N4" => {
                        party_loop.n4_segment = Some(segments.next().unwrap().clone());
                    }
                    "PER" => party_loop.per_segments.push(segments.next().unwrap().clone()),
                    "N1" => break, // Next party - don't consume it
                    "PO1" | "CTT" | "SE" => break, // End of parties
                    _ => {
                        // Skip unknown segments that might be between parties
                        segments.next();
                    }
                }
            }

            party_loops.push(party_loop);
        }

        // Parse line item loops (PO1 groups)
        while let Some(segment) = segments.peek() {
            if segment.id != "PO1" {
                break;
            }

            let mut line_loop = LineItemLoop {
                po1_segment: segments.next().unwrap().clone(),
                pid_segments: Vec::new(),
                sac_segments: Vec::new(),
                dtm_segments: Vec::new(),
            };

            // Parse line item detail segments
            while let Some(segment) = segments.peek() {
                match segment.id.as_str() {
                    "PID" => line_loop.pid_segments.push(segments.next().unwrap().clone()),
                    "SAC" => line_loop.sac_segments.push(segments.next().unwrap().clone()),
                    "DTM" => line_loop.dtm_segments.push(segments.next().unwrap().clone()),
                    "TD5" => line_loop.sac_segments.push(segments.next().unwrap().clone()), // TD5 can be part of line item
                    "PO1" => break, // Next line item - don't consume it
                    "CTT" | "SE" => break, // End of transaction
                    _ => {
                        // Skip unknown segments
                        segments.next();
                    }
                }
            }

            line_item_loops.push(line_loop);
        }

        // Add remaining summary segments
        while let Some(segment) = segments.next() {
            summary_segments.push(segment.clone());
        }

        Ok(PurchaseOrder850 {
            transaction_type: transaction.transaction_type.clone(),
            header_segments,
            party_loops,
            line_item_loops,
            summary_segments,
        })
    }

    pub fn get_total_line_items(&self) -> usize {
        self.line_item_loops.len()
    }

    pub fn get_total_quantity(&self) -> f64 {
        self.line_item_loops.iter()
            .filter_map(|item| {
                item.po1_segment.elements.get(1)
                    .and_then(|qty| qty.parse::<f64>().ok())
            })
            .sum()
    }

    pub fn get_parties_by_type(&self, entity_type: &str) -> Vec<&PartyLoop> {
        self.party_loops.iter()
            .filter(|party| {
                party.n1_segment.elements.get(0)
                    .map(|et| et == entity_type)
                    .unwrap_or(false)
            })
            .collect()
    }
}
