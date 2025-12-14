// src/protocol/screen.rs
#![allow(dead_code)]

use crate::protocol::{Command, ProtocolError};

// === H Shift (0x41) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HShiftAction {
    Up,
    Down,
    SetPlus(u8),
    SetMinus(u8),
}

pub struct HShift;

impl Command for HShift {
    type Action = HShiftAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x41;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            HShiftAction::Up => vec![0x00, 0x00],
            HShiftAction::Down => vec![0x00, 0x01],
            HShiftAction::SetPlus(v) => vec![0x01, 0x00, *v],
            HShiftAction::SetMinus(v) => vec![0x01, 0x01, *v],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === V Size (0x42) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VSizeAction {
    Up,
    Down,
    SetPlus(u8),
    SetMinus(u8),
}

pub struct VSize;

impl Command for VSize {
    type Action = VSizeAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x42;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            VSizeAction::Up => vec![0x00, 0x00],
            VSizeAction::Down => vec![0x00, 0x01],
            VSizeAction::SetPlus(v) => vec![0x01, 0x00, *v],
            VSizeAction::SetMinus(v) => vec![0x01, 0x01, *v],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === V Shift (0x43) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VShiftAction {
    Up,
    Down,
    SetPlus(u8),
    SetMinus(u8),
}

pub struct VShift;

impl Command for VShift {
    type Action = VShiftAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x43;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            VShiftAction::Up => vec![0x00, 0x00],
            VShiftAction::Down => vec![0x00, 0x01],
            VShiftAction::SetPlus(v) => vec![0x01, 0x00, *v],
            VShiftAction::SetMinus(v) => vec![0x01, 0x01, *v],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === Wide (0x44) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WideAction {
    Toggle,
    WideZoom,
    Full,
    Zoom,
    Normal,
    PcNormal,
    PcFull1,
    PcFull2,
}

pub struct Wide;

impl Command for Wide {
    type Action = WideAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x44;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            WideAction::Toggle => vec![0x00],
            WideAction::WideZoom => vec![0x01, 0x00],
            WideAction::Full => vec![0x01, 0x01],
            WideAction::Zoom => vec![0x01, 0x02],
            WideAction::Normal => vec![0x01, 0x03],
            WideAction::PcNormal => vec![0x01, 0x05],
            WideAction::PcFull1 => vec![0x01, 0x06],
            WideAction::PcFull2 => vec![0x01, 0x07],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === Auto Wide (0x45) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoWideAction {
    Toggle,
    Off,
    On,
}

pub struct AutoWide;

impl Command for AutoWide {
    type Action = AutoWideAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x45;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            AutoWideAction::Toggle => vec![0x00],
            AutoWideAction::Off => vec![0x01, 0x00],
            AutoWideAction::On => vec![0x01, 0x01],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === 4:3 Mode (0x46) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FourThreeModeAction {
    Toggle,
    Off,
    WideZoom,
    Normal,
}

pub struct FourThreeMode;

impl Command for FourThreeMode {
    type Action = FourThreeModeAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x46;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            FourThreeModeAction::Toggle => vec![0x00],
            FourThreeModeAction::Off => vec![0x01, 0x00],
            FourThreeModeAction::WideZoom => vec![0x01, 0x03],
            FourThreeModeAction::Normal => vec![0x01, 0x04],
        }
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
    fn test_wide_build_bytes() {
        assert_eq!(Wide::build_bytes(&WideAction::Toggle), vec![0x00]);
        assert_eq!(Wide::build_bytes(&WideAction::Full), vec![0x01, 0x01]);
        assert_eq!(Wide::build_bytes(&WideAction::Normal), vec![0x01, 0x03]);
    }

    #[test]
    fn test_auto_wide_build_bytes() {
        assert_eq!(AutoWide::build_bytes(&AutoWideAction::Toggle), vec![0x00]);
        assert_eq!(AutoWide::build_bytes(&AutoWideAction::On), vec![0x01, 0x01]);
    }

    #[test]
    fn test_h_shift_build_bytes() {
        assert_eq!(HShift::build_bytes(&HShiftAction::Up), vec![0x00, 0x00]);
        assert_eq!(
            HShift::build_bytes(&HShiftAction::SetPlus(10)),
            vec![0x01, 0x00, 10]
        );
    }
}
