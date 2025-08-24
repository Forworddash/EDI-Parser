use super::EdiParser;
use crate::{
    models::{InterchangeControl, FunctionalGroup, Transaction, Segment},
    error::EdiError,
};

pub struct X12Parser {
    element_separator: char,
    segment_separator: char,
    sub_element_separator: char,
}

impl Default for X12Parser {
    fn default() -> Self {
        Self {
            element_separator: '*',
            segment_separator: '~',
            sub_element_separator: '>',
        }
    }
}

impl X12Parser {
    pub fn with_delimiters(
        element_separator: char,
        segment_separator: char,
        sub_element_separator: char,
    ) -> Self {
        Self {
            element_separator,
            segment_separator,
            sub_element_separator,
        }
    }

    fn trim_whitespace(&self, s: &str) -> String {
        s.trim().to_string()
    }

    fn parse_segment(&self, line: &str) -> Result<Segment, EdiError> {
        let elements: Vec<String> = line
            .split(self.element_separator)
            .map(|s| self.trim_whitespace(s)) // Trim whitespace from each element
            .collect();
        
        if elements.is_empty() {
            return Err(EdiError::InvalidSegmentFormat(line.to_string()));
        }

        Ok(Segment::new(elements[0].clone(), elements[1..].to_vec()))
    }

    fn extract_delimiters_from_isa(&self, isa_segment: &str) -> Result<(char, char, char), EdiError> {
        let elements: Vec<&str> = isa_segment.split('*').collect();
        if elements.len() < 16 {
            return Err(EdiError::InvalidSegmentFormat(isa_segment.to_string()));
        }

        let element_separator = '*';
        let segment_separator = elements[15].chars().next().unwrap_or('~');
        let sub_element_separator = elements[16].chars().next().unwrap_or('>');

        Ok((element_separator, segment_separator, sub_element_separator))
    }

    fn parse_isa_segment(&self, isa_line: &str) -> Result<Segment, EdiError> {
        // ISA segment has fixed-width fields, handle it specially
        let trimmed = self.trim_whitespace(isa_line);
        let elements: Vec<String> = trimmed
            .split('*')
            .map(|s| self.trim_whitespace(s))
            .collect();
        
        if elements.len() < 16 {
            return Err(EdiError::InvalidSegmentFormat(isa_line.to_string()));
        }

        Ok(Segment::new("ISA".to_string(), elements[1..].to_vec()))
    }
}

impl EdiParser for X12Parser {
    fn parse(&self, input: &str) -> Result<InterchangeControl, EdiError> {
        let segments: Vec<&str> = input
            .split(self.segment_separator)
            .filter(|s| !s.trim().is_empty())
            .collect();

        if segments.is_empty() {
            return Err(EdiError::InvalidSegmentFormat("Empty input".to_string()));
        }

        // Parse ISA segment first to get actual delimiters
        let isa_segment = self.parse_isa_segment(segments[0])?;
        let (actual_element_sep, actual_segment_sep, actual_sub_element_sep) = 
            self.extract_delimiters_from_isa(segments[0])?; // Fixed: use self. instead of Self::

        let parser = if actual_element_sep != self.element_separator || 
            actual_segment_sep != self.segment_separator {
            X12Parser::with_delimiters(actual_element_sep, actual_segment_sep, actual_sub_element_sep)
        } else {
            self.clone()
        };

        let mut functional_groups = Vec::new();
        let mut current_fg: Option<FunctionalGroup> = None;
        let mut current_transaction: Option<Transaction> = None;
        let mut transaction_count = 0;

        for segment_str in segments.iter().skip(1) {
            let segment = parser.parse_segment(segment_str)?;
            
            match segment.id.as_str() {
                "GS" => {
                    if let Some(fg) = current_fg.take() {
                        functional_groups.push(fg);
                    }
                    current_fg = Some(FunctionalGroup {
                        gs_segment: segment,
                        ge_segment: None,
                        transactions: Vec::new(),
                    });
                }
                "GE" => {
                    if let Some(mut fg) = current_fg.take() {
                        fg.ge_segment = Some(segment);
                        functional_groups.push(fg);
                    }
                }
                "ST" => {
                    if let Some(transaction) = current_transaction.take() {
                        if let Some(fg) = current_fg.as_mut() {
                            fg.transactions.push(transaction);
                        }
                    }
                    let transaction_set_id = segment.elements.get(0)
                        .ok_or_else(|| EdiError::InvalidSegmentFormat(segment_str.to_string()))?
                        .clone();
                    let control_number = segment.elements.get(1)
                        .ok_or_else(|| EdiError::InvalidSegmentFormat(segment_str.to_string()))?
                        .clone();
                    
                    current_transaction = Some(Transaction::new(
                        vec![segment],
                        transaction_set_id,
                        control_number,
                    ));
                    transaction_count += 1;
                }
                "SE" => {
                    if let Some(mut transaction) = current_transaction.take() {
                        transaction.segments.push(segment);
                        if let Some(fg) = current_fg.as_mut() {
                            fg.transactions.push(transaction);
                        }
                    }
                }
                _ => {
                    if let Some(transaction) = current_transaction.as_mut() {
                        transaction.segments.push(segment);
                    }
                }
            }
        }

        // Handle any remaining transactions or functional groups
        if let Some(transaction) = current_transaction.take() {
            if let Some(fg) = current_fg.as_mut() {
                fg.transactions.push(transaction);
            }
        }

        if let Some(fg) = current_fg.take() {
            functional_groups.push(fg);
        }

        let iea_segment = segments.iter()
            .find(|s| s.starts_with("IEA"))
            .map(|s| parser.parse_segment(s))
            .transpose()?;

        Ok(InterchangeControl {
            isa_segment,
            iea_segment,
            functional_groups,
        })
    }

    fn validate(&self, interchange: &InterchangeControl) -> Result<(), EdiError> {
        // Basic validation logic
        if interchange.isa_segment.id != "ISA" {
            return Err(EdiError::MissingRequiredSegment("ISA".to_string()));
        }

        if let Some(iea) = &interchange.iea_segment {
            if iea.id != "IEA" {
                return Err(EdiError::InvalidControlStructure);
            }
        }

        Ok(())
    }
}

impl Clone for X12Parser {
    fn clone(&self) -> Self {
        Self {
            element_separator: self.element_separator,
            segment_separator: self.segment_separator,
            sub_element_separator: self.sub_element_separator,
        }
    }
}