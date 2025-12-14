pub mod error;
pub mod language;
pub mod mode_control;
pub mod picture;
pub mod screen;
pub mod signage;
pub mod sircs;
pub mod sound;
pub mod values;

pub use error::{ProtocolError, ResponseCode};
pub use language::*;
pub use mode_control::*;
pub use picture::*;
pub use screen::*;
pub use signage::*;
pub use sircs::*;
pub use sound::*;
pub use values::*;

pub const CONTROL_HEADER: u8 = 0x8C;
pub const QUERY_HEADER: u8 = 0x83;
pub const CATEGORY: u8 = 0x00;
pub const RESPONSE_HEADER: u8 = 0x70;

pub trait Command {
    type Action;
    type Response;

    const FUNCTION_CODE: u8;

    fn build_bytes(action: &Self::Action) -> Vec<u8>;
    fn parse_response(bytes: &[u8]) -> Result<Self::Response, ProtocolError>;
    fn supports_query() -> bool;
}

pub fn checksum(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
}

pub fn build_control_packet(function: u8, data: &[u8]) -> Vec<u8> {
    let length = (data.len() + 1) as u8; // data + checksum
    let mut packet = vec![CONTROL_HEADER, CATEGORY, function, length];
    packet.extend_from_slice(data);
    let cs = checksum(&packet);
    packet.push(cs);
    packet
}

pub fn build_query_packet(function: u8) -> Vec<u8> {
    let packet_without_cs = vec![QUERY_HEADER, CATEGORY, function, 0xFF, 0xFF];
    let cs = checksum(&packet_without_cs);
    let mut packet = packet_without_cs;
    packet.push(cs);
    packet
}

#[cfg(test)]
mod tests;
