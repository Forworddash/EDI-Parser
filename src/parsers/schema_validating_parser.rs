use crate::error::EdiError;
use crate::models::segment::Segment;
use crate::models::segment_definition::{X12Version, SegmentRegistry};
use crate::models::schema_engine::{SchemaEngine, ValidationResult, ValidationResultType};
use crate::models::validated_element::ValidatedElement;
use crate::parsers::x12::X12Parser;
use crate::parsers::EdiParser;

/// Enhanced validating parser with comprehensive schema validation
/// Supports version-aware parsing and trading partner customizations
#[derive(Debug)]
pub struct SchemaValidatingParser {
    /// Core X12 parser
    x12_parser: X12Parser,
    /// Schema engine for comprehensive validation
    schema_engine: SchemaEngine,
    /// Segment registry for element-level validation
    segment_registry: SegmentRegistry,
    /// Parsing configuration
    config: ParsingConfig,
}

/// Configuration options for schema-based parsing
#[derive(Debug, Clone)]
pub struct ParsingConfig {
    /// X12 version to use for validation
    pub version: X12Version,
    /// Trading partner ID for customizations
    pub trading_partner_id: Option<String>,
    /// Validation strictness level
    pub validation_level: ValidationLevel,
    /// Whether to continue parsing on validation errors
    pub continue_on_error: bool,
    /// Whether to collect detailed validation results
    pub collect_validation_details: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationLevel {
    /// Basic syntax validation only
    Basic,
    /// Schema-aware validation with element checks
    Standard,
    /// Comprehensive validation including trading partner rules
    Strict,
    /// Full validation with business rule checks
    Complete,
}

/// Comprehensive parsing result with validation details
#[derive(Debug)]
pub struct SchemaParsingResult {
    /// Parsed segments
    pub segments: Vec<ValidatedSegment>,
    /// Validation results
    pub validation_results: Vec<ValidationResult>,
    /// Transaction summaries
    pub transaction_summaries: Vec<TransactionSummary>,
    /// Overall parsing status
    pub status: ParsingStatus,
    /// Performance metrics
    pub metrics: ParsingMetrics,
}

/// Enhanced segment with validation context
#[derive(Debug, Clone)]
pub struct ValidatedSegment {
    /// Original segment
    pub segment: Segment,
    /// Validated elements with metadata
    pub validated_elements: Vec<ValidatedElement>,
    /// Segment-level validation results
    pub validation_results: Vec<ValidationResult>,
    /// Schema context information
    pub schema_context: SegmentSchemaContext,
}

#[derive(Debug, Clone)]
pub struct SegmentSchemaContext {
    /// Transaction type this segment belongs to
    pub transaction_type: Option<String>,
    /// Hierarchy level (0=header, 1=detail, 2=summary)
    pub hierarchy_level: u32,
    /// Position within transaction
    pub transaction_position: usize,
    /// Loop context if applicable
    pub loop_context: Option<String>,
    /// Segment requirement status
    pub requirement_status: SegmentRequirementStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SegmentRequirementStatus {
    Required,
    Optional,
    Conditional,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct TransactionSummary {
    /// Transaction type (850, 810, etc.)
    pub transaction_type: String,
    /// X12 version used
    pub version: X12Version,
    /// Start and end positions in segment list
    pub segment_range: (usize, usize),
    /// Segment count
    pub segment_count: usize,
    /// Validation status
    pub validation_status: ValidationStatus,
    /// Business context
    pub business_context: BusinessContext,
}

#[derive(Debug, Clone)]
pub struct BusinessContext {
    /// Trading partner information
    pub trading_partner: Option<String>,
    /// Document date
    pub document_date: Option<String>,
    /// Document number/ID
    pub document_id: Option<String>,
    /// Currency if specified
    pub currency: Option<String>,
    /// Total amount if applicable
    pub total_amount: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationStatus {
    Valid,
    ValidWithWarnings,
    Invalid,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParsingStatus {
    Success,
    SuccessWithWarnings,
    PartialSuccess,
    Failed,
}

#[derive(Debug, Clone)]
pub struct ParsingMetrics {
    /// Total parsing time
    pub parsing_time_ms: u64,
    /// Validation time
    pub validation_time_ms: u64,
    /// Memory usage
    pub memory_usage_bytes: usize,
    /// Segments processed
    pub segments_processed: usize,
    /// Elements validated
    pub elements_validated: usize,
}

impl SchemaValidatingParser {
    /// Create a new schema validating parser
    pub fn new(config: ParsingConfig) -> Self {
        Self {
            x12_parser: X12Parser::new(),
            schema_engine: SchemaEngine::new(),
            segment_registry: SegmentRegistry::new(),
            config,
        }
    }

    /// Parse EDI data with comprehensive schema validation
    pub fn parse_with_schema(&mut self, edi_data: &str) -> Result<SchemaParsingResult, EdiError> {
        let start_time = std::time::Instant::now();
        
        // Parse basic structure
        let interchange = self.x12_parser.parse(edi_data)?;
        
        let mut all_validation_results = Vec::new();
        let mut all_validated_segments = Vec::new();
        let mut transaction_summaries = Vec::new();
        let mut segments_processed = 0;
        let mut elements_validated = 0;

        // Process each functional group
        for functional_group in &interchange.functional_groups {
            // Process each transaction
            for transaction in &functional_group.transactions {
                let transaction_start = all_validated_segments.len();
                let mut transaction_segments = Vec::new();
                let mut transaction_validation_results = Vec::new();

                // Determine transaction type from ST segment
                let transaction_type = if let Some(st_segment) = transaction.segments.first() {
                    if st_segment.id == "ST" && st_segment.elements.len() > 1 {
                        Some(st_segment.elements[1].clone())
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Validate transaction structure
                if let Some(tx_type) = &transaction_type {
                    let segment_ids: Vec<String> = transaction.segments.iter().map(|s| s.id.clone()).collect();
                    
                    match self.schema_engine.validate_transaction_structure(tx_type, &self.config.version, &segment_ids) {
                        Ok(results) => {
                            transaction_validation_results.extend(results);
                        },
                        Err(e) => {
                            transaction_validation_results.push(ValidationResult {
                                result_type: ValidationResultType::Error,
                                message: format!("Transaction structure validation failed: {}", e),
                                segment_position: None,
                                element_position: None,
                            });
                        }
                    }
                }

                // Process segments
                for (segment_position, segment) in transaction.segments.iter().enumerate() {
                    segments_processed += 1;

                    // Validate segment elements
                    let mut validated_elements = Vec::new();
                    let mut segment_validation_results = Vec::new();

                    for (element_position, element_value) in segment.elements.iter().enumerate() {
                        elements_validated += 1;

                        // Get element definition from segment registry
                        if let Some(segment_def) = self.segment_registry.get_definition(&self.config.version, &segment.id) {
                            if let Some(element_def) = segment_def.elements.get(element_position) {
                                // Validate element
                                match self.segment_registry.validate_element_value(
                                    &self.config.version,
                                    &segment.id,
                                    element_position,
                                    element_value
                                ) {
                                    Ok(_) => {
                                        validated_elements.push(ValidatedElement::new(
                                            element_def.clone(),
                                            Some(element_value.clone()),
                                            element_position,
                                        ));
                                    },
                                    Err(e) => {
                                        segment_validation_results.push(ValidationResult {
                                            result_type: ValidationResultType::Error,
                                            message: format!("Element validation failed: {}", e),
                                            segment_position: Some(segment_position),
                                            element_position: Some(element_position),
                                        });
                                        
                                        // Still create validated element with error
                                        validated_elements.push(ValidatedElement::new(
                                            element_def.clone(),
                                            Some(element_value.clone()),
                                            element_position,
                                        ));
                                    }
                                }
                            } else {
                                // Element position not found in definition
                                segment_validation_results.push(ValidationResult {
                                    result_type: ValidationResultType::Warning,
                                    message: format!("Element position {} not defined in segment {}", element_position + 1, segment.id),
                                    segment_position: Some(segment_position),
                                    element_position: Some(element_position),
                                });
                            }
                        }
                    }

                    // Determine schema context
                    let schema_context = self.determine_schema_context(
                        &transaction_type,
                        &segment.id,
                        segment_position,
                        &transaction.segments,
                    );

                    let validated_segment = ValidatedSegment {
                        segment: segment.clone(),
                        validated_elements,
                        validation_results: segment_validation_results.clone(),
                        schema_context,
                    };

                    transaction_segments.push(validated_segment.clone());
                    all_validated_segments.push(validated_segment);
                    transaction_validation_results.extend(segment_validation_results);
                }

                // Create transaction summary
                let business_context = self.extract_business_context(&transaction_segments);
                let validation_status = self.determine_validation_status(&transaction_validation_results);
                
                transaction_summaries.push(TransactionSummary {
                    transaction_type: transaction_type.unwrap_or_else(|| "UNKNOWN".to_string()),
                    version: self.config.version,
                    segment_range: (transaction_start, all_validated_segments.len()),
                    segment_count: transaction.segments.len(),
                    validation_status,
                    business_context,
                });

                all_validation_results.extend(transaction_validation_results);
            }
        }

        let parsing_time = start_time.elapsed();
        
        // Determine overall parsing status
        let status = self.determine_parsing_status(&all_validation_results);

        let metrics = ParsingMetrics {
            parsing_time_ms: parsing_time.as_millis() as u64,
            validation_time_ms: 0, // Would be measured separately
            memory_usage_bytes: 0, // Would be measured if needed
            segments_processed,
            elements_validated,
        };

        Ok(SchemaParsingResult {
            segments: all_validated_segments,
            validation_results: all_validation_results,
            transaction_summaries,
            status,
            metrics,
        })
    }

    /// Determine schema context for a segment
    fn determine_schema_context(
        &self,
        transaction_type: &Option<String>,
        segment_id: &str,
        position: usize,
        _all_segments: &[Segment],
    ) -> SegmentSchemaContext {
        let hierarchy_level = match segment_id {
            "ST" | "BEG" | "CUR" | "REF" | "PER" | "DTM" => 0, // Header
            "SE" | "CTT" => 2, // Summary
            _ => 1, // Detail
        };

        let requirement_status = match segment_id {
            "ST" | "BEG" | "SE" => SegmentRequirementStatus::Required,
            "PO1" => SegmentRequirementStatus::Required, // For 850
            _ => SegmentRequirementStatus::Optional,
        };

        SegmentSchemaContext {
            transaction_type: transaction_type.clone(),
            hierarchy_level,
            transaction_position: position,
            loop_context: None, // Would be determined by more sophisticated loop detection
            requirement_status,
        }
    }

    /// Extract business context from transaction segments
    fn extract_business_context(&self, segments: &[ValidatedSegment]) -> BusinessContext {
        let mut context = BusinessContext {
            trading_partner: None,
            document_date: None,
            document_id: None,
            currency: None,
            total_amount: None,
        };

        for segment in segments {
            match segment.segment.id.as_str() {
                "BEG" => {
                    // Extract document ID from BEG03
                    if segment.segment.elements.len() > 3 {
                        context.document_id = Some(segment.segment.elements[3].clone());
                    }
                    // Extract document date from BEG05
                    if segment.segment.elements.len() > 5 {
                        context.document_date = Some(segment.segment.elements[5].clone());
                    }
                },
                "CUR" => {
                    // Extract currency from CUR02
                    if segment.segment.elements.len() > 2 {
                        context.currency = Some(segment.segment.elements[2].clone());
                    }
                },
                "N1" => {
                    // Extract trading partner info
                    if segment.segment.elements.len() > 1 && segment.segment.elements[0] == "BY" {
                        if segment.segment.elements.len() > 2 {
                            context.trading_partner = Some(segment.segment.elements[2].clone());
                        }
                    }
                },
                _ => {}
            }
        }

        context
    }

    /// Determine validation status from results
    fn determine_validation_status(&self, results: &[ValidationResult]) -> ValidationStatus {
        let has_errors = results.iter().any(|r| r.result_type == ValidationResultType::Error);
        let has_warnings = results.iter().any(|r| r.result_type == ValidationResultType::Warning);

        if has_errors {
            ValidationStatus::Invalid
        } else if has_warnings {
            ValidationStatus::ValidWithWarnings
        } else {
            ValidationStatus::Valid
        }
    }

    /// Determine overall parsing status
    fn determine_parsing_status(&self, results: &[ValidationResult]) -> ParsingStatus {
        let error_count = results.iter().filter(|r| r.result_type == ValidationResultType::Error).count();
        let warning_count = results.iter().filter(|r| r.result_type == ValidationResultType::Warning).count();

        if error_count == 0 && warning_count == 0 {
            ParsingStatus::Success
        } else if error_count == 0 && warning_count > 0 {
            ParsingStatus::SuccessWithWarnings
        } else if error_count > 0 && self.config.continue_on_error {
            ParsingStatus::PartialSuccess
        } else {
            ParsingStatus::Failed
        }
    }

    /// Get element by ID across all segments in result
    pub fn get_elements_by_id(&self, result: &SchemaParsingResult, element_id: u32) -> Vec<ElementReference> {
        let mut references = Vec::new();

        for (segment_index, validated_segment) in result.segments.iter().enumerate() {
            for (element_index, validated_element) in validated_segment.validated_elements.iter().enumerate() {
                if validated_element.definition.element_id == element_id {
                    references.push(ElementReference {
                        element_id,
                        segment_index,
                        element_index,
                        segment_id: validated_segment.segment.id.clone(),
                        value: validated_element.value.clone().unwrap_or_default(),
                        element_name: validated_element.definition.name.clone(),
                    });
                }
            }
        }

        references
    }

    /// Get schema engine reference for advanced operations
    pub fn schema_engine(&self) -> &SchemaEngine {
        &self.schema_engine
    }

    /// Get configuration
    pub fn config(&self) -> &ParsingConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: ParsingConfig) {
        self.config = config;
    }
}

#[derive(Debug, Clone)]
pub struct ElementReference {
    pub element_id: u32,
    pub segment_index: usize,
    pub element_index: usize,
    pub segment_id: String,
    pub value: String,
    pub element_name: String,
}

impl Default for ParsingConfig {
    fn default() -> Self {
        Self {
            version: X12Version::V4010,
            trading_partner_id: None,
            validation_level: ValidationLevel::Standard,
            continue_on_error: true,
            collect_validation_details: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_validating_parser_creation() {
        let config = ParsingConfig::default();
        let parser = SchemaValidatingParser::new(config);
        
        assert_eq!(parser.config.version, X12Version::V4010);
        assert_eq!(parser.config.validation_level, ValidationLevel::Standard);
    }

    #[test]
    fn test_parsing_config_default() {
        let config = ParsingConfig::default();
        
        assert_eq!(config.version, X12Version::V4010);
        assert_eq!(config.validation_level, ValidationLevel::Standard);
        assert!(config.continue_on_error);
        assert!(config.collect_validation_details);
    }
}
