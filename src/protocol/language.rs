// src/protocol/language.rs
#![allow(dead_code)]

use crate::protocol::{Command, ProtocolError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageCode([u8; 3]);

impl LanguageCode {
    pub fn new(code: &str) -> Result<Self, ProtocolError> {
        if code.len() != 3 {
            return Err(ProtocolError::InvalidResponse);
        }
        let bytes: [u8; 3] = code.as_bytes().try_into().unwrap();
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8; 3] {
        &self.0
    }

    // Common language codes
    pub fn english() -> Self {
        Self(*b"eng")
    }
    pub fn japanese() -> Self {
        Self(*b"jpn")
    }
    pub fn german() -> Self {
        Self(*b"ger")
    }
    pub fn french() -> Self {
        Self(*b"fre")
    }
    pub fn spanish() -> Self {
        Self(*b"spa")
    }
    pub fn chinese_simplified() -> Self {
        Self(*b"CHS")
    }
    pub fn chinese_traditional() -> Self {
        Self(*b"CHT")
    }
}

pub struct Language;

impl Command for Language {
    type Action = LanguageCode;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x07;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        let mut bytes = vec![0x00];
        bytes.extend_from_slice(action.as_bytes());
        bytes
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_build_bytes() {
        let english = LanguageCode::english();
        assert_eq!(
            Language::build_bytes(&english),
            vec![0x00, b'e', b'n', b'g']
        );

        let japanese = LanguageCode::japanese();
        assert_eq!(
            Language::build_bytes(&japanese),
            vec![0x00, b'j', b'p', b'n']
        );

        let chinese = LanguageCode::chinese_simplified();
        assert_eq!(
            Language::build_bytes(&chinese),
            vec![0x00, b'C', b'H', b'S']
        );
    }

    #[test]
    fn test_language_code_new() {
        assert!(LanguageCode::new("eng").is_ok());
        assert!(LanguageCode::new("en").is_err());
        assert!(LanguageCode::new("english").is_err());
    }
}
