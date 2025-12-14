// src/protocol/signage.rs
#![allow(dead_code)]

use crate::protocol::{Command, ProtocolError};

// === Product Info 1 (0x6E) ===

pub struct ProductInfo1;

impl Command for ProductInfo1 {
    type Action = ();
    type Response = Vec<u8>;

    const FUNCTION_CODE: u8 = 0x6E;

    fn build_bytes(_action: &Self::Action) -> Vec<u8> {
        vec![]
    }

    fn parse_response(bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(bytes.to_vec())
    }

    fn supports_query() -> bool {
        true
    }
}

// === Product Info 2 (0x6D) ===

pub struct ProductInfo2;

impl Command for ProductInfo2 {
    type Action = ();
    type Response = Vec<u8>;

    const FUNCTION_CODE: u8 = 0x6D;

    fn build_bytes(_action: &Self::Action) -> Vec<u8> {
        vec![]
    }

    fn parse_response(bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(bytes.to_vec())
    }

    fn supports_query() -> bool {
        true
    }
}

// === Product Info 3 (0x6C) ===

pub struct ProductInfo3;

impl Command for ProductInfo3 {
    type Action = ();
    type Response = Vec<u8>;

    const FUNCTION_CODE: u8 = 0x6C;

    fn build_bytes(_action: &Self::Action) -> Vec<u8> {
        vec![]
    }

    fn parse_response(bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(bytes.to_vec())
    }

    fn supports_query() -> bool {
        true
    }
}

// === ID Command (0x6F) ===

pub struct IdCommand;

impl Command for IdCommand {
    type Action = ();
    type Response = Vec<u8>;

    const FUNCTION_CODE: u8 = 0x6F;

    fn build_bytes(_action: &Self::Action) -> Vec<u8> {
        vec![]
    }

    fn parse_response(bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(bytes.to_vec())
    }

    fn supports_query() -> bool {
        true
    }
}
