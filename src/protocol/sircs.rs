// src/protocol/sircs.rs
use crate::protocol::{Command, ProtocolError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SircsButton {
    Input,
    Power,
    WideMode,
    Dot,
    Display,
    Return,
    Options,
    Home,
    CursorUp,
    CursorDown,
    CursorLeft,
    CursorRight,
    Select,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    ClosedCaption,
    VolumeUp,
    VolumeDown,
    Muting,
    ChannelUp,
    ChannelDown,
    Jump,
}

impl SircsButton {
    pub fn codes(&self) -> (u8, u8) {
        match self {
            SircsButton::Input => (0x01, 0x25),
            SircsButton::Power => (0x01, 0x15),
            SircsButton::WideMode => (0xA4, 0x3D),
            SircsButton::Dot => (0x97, 0x1D),
            SircsButton::Display => (0x01, 0x3A),
            SircsButton::Return => (0x97, 0x23),
            SircsButton::Options => (0x97, 0x36),
            SircsButton::Home => (0x01, 0x60),
            SircsButton::CursorUp => (0x01, 0x74),
            SircsButton::CursorDown => (0x01, 0x75),
            SircsButton::CursorLeft => (0x01, 0x34),
            SircsButton::CursorRight => (0x01, 0x33),
            SircsButton::Select => (0x01, 0x65),
            SircsButton::Num1 => (0x01, 0x00),
            SircsButton::Num2 => (0x01, 0x01),
            SircsButton::Num3 => (0x01, 0x02),
            SircsButton::Num4 => (0x01, 0x03),
            SircsButton::Num5 => (0x01, 0x04),
            SircsButton::Num6 => (0x01, 0x05),
            SircsButton::Num7 => (0x01, 0x06),
            SircsButton::Num8 => (0x01, 0x07),
            SircsButton::Num9 => (0x01, 0x08),
            SircsButton::Num0 => (0x01, 0x09),
            SircsButton::ClosedCaption => (0xA4, 0x10),
            SircsButton::VolumeUp => (0x01, 0x12),
            SircsButton::VolumeDown => (0x01, 0x13),
            SircsButton::Muting => (0x01, 0x14),
            SircsButton::ChannelUp => (0x01, 0x10),
            SircsButton::ChannelDown => (0x01, 0x11),
            SircsButton::Jump => (0x01, 0x3B),
        }
    }
}

pub struct Sircs;

impl Command for Sircs {
    type Action = SircsButton;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x67;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        let (category, data) = action.codes();
        vec![category, data]
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
    fn test_sircs_build_bytes() {
        assert_eq!(Sircs::build_bytes(&SircsButton::Power), vec![0x01, 0x15]);
        assert_eq!(Sircs::build_bytes(&SircsButton::Home), vec![0x01, 0x60]);
        assert_eq!(Sircs::build_bytes(&SircsButton::Select), vec![0x01, 0x65]);
        assert_eq!(Sircs::build_bytes(&SircsButton::Num1), vec![0x01, 0x00]);
        assert_eq!(Sircs::build_bytes(&SircsButton::WideMode), vec![0xA4, 0x3D]);
    }
}
