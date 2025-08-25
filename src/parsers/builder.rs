use super::X12Parser;
use crate::error::EdiError;

/// Builder for configuring an X12 parser with custom settings
#[derive(Debug, Default)]
pub struct X12ParserBuilder {
    element_separator: Option<char>,
    segment_separator: Option<char>,
    sub_element_separator: Option<char>,
    strict_validation: bool,
    trim_whitespace: bool,
}

impl X12ParserBuilder {
    /// Create a new parser builder with default settings
    pub fn new() -> Self {
        Self {
            element_separator: None,
            segment_separator: None,
            sub_element_separator: None,
            strict_validation: false,
            trim_whitespace: true,
        }
    }

    /// Set the element separator character (default: '*')
    pub fn element_separator(mut self, sep: char) -> Self {
        self.element_separator = Some(sep);
        self
    }

    /// Set the segment separator character (default: '~')
    pub fn segment_separator(mut self, sep: char) -> Self {
        self.segment_separator = Some(sep);
        self
    }

    /// Set the sub-element separator character (default: '>')
    pub fn sub_element_separator(mut self, sep: char) -> Self {
        self.sub_element_separator = Some(sep);
        self
    }

    /// Enable strict validation (validates control numbers, segment counts, etc.)
    pub fn strict_validation(mut self, strict: bool) -> Self {
        self.strict_validation = strict;
        self
    }

    /// Enable/disable automatic whitespace trimming
    pub fn trim_whitespace(mut self, trim: bool) -> Self {
        self.trim_whitespace = trim;
        self
    }

    /// Build the configured X12 parser
    pub fn build(self) -> Result<X12Parser, EdiError> {
        let element_sep = self.element_separator.unwrap_or('*');
        let segment_sep = self.segment_separator.unwrap_or('~');
        let sub_element_sep = self.sub_element_separator.unwrap_or('>');

        // Validate that separators are different
        if element_sep == segment_sep || element_sep == sub_element_sep || segment_sep == sub_element_sep {
            return Err(EdiError::ValidationError("Separators must be different".to_string()));
        }

        Ok(X12Parser::with_config(
            element_sep,
            segment_sep,
            sub_element_sep,
            self.strict_validation,
            self.trim_whitespace,
        ))
    }
}
