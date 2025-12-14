#![allow(dead_code)]

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseCode {
    Success = 0x00,
    LimitOverMax = 0x01,
    LimitOverMin = 0x02,
    Canceled = 0x03,
    ParseError = 0x04,
}

impl TryFrom<u8> for ResponseCode {
    type Error = ProtocolError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ResponseCode::Success),
            0x01 => Ok(ResponseCode::LimitOverMax),
            0x02 => Ok(ResponseCode::LimitOverMin),
            0x03 => Ok(ResponseCode::Canceled),
            0x04 => Ok(ResponseCode::ParseError),
            _ => Err(ProtocolError::InvalidResponseCode(value)),
        }
    }
}

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("invalid response code: {0:#04x}")]
    InvalidResponseCode(u8),
    #[error("invalid response data")]
    InvalidResponse,
}

#[derive(Debug, Error)]
pub enum RangeError {
    #[error("value {got} exceeds maximum {max}")]
    AboveMax { max: u8, got: u8 },
    #[error("value {got} below minimum {min}")]
    BelowMin { min: u8, got: u8 },
}
