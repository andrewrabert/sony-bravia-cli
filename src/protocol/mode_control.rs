#![allow(dead_code)]

use crate::protocol::{Command, ProtocolError, SleepMinutes, VolumeValue};

// === Power (0x00) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerAction {
    Off,
    On,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    Off,
    On,
}

pub struct Power;

impl Command for Power {
    type Action = PowerAction;
    type Response = PowerState;

    const FUNCTION_CODE: u8 = 0x00;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            PowerAction::Off => vec![0x00],
            PowerAction::On => vec![0x01],
        }
    }

    fn parse_response(bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        match bytes.first() {
            Some(0x00) => Ok(PowerState::Off),
            Some(0x01) => Ok(PowerState::On),
            _ => Err(ProtocolError::InvalidResponse),
        }
    }

    fn supports_query() -> bool {
        true
    }
}

// === Standby (0x01) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandbyAction {
    Disable,
    Enable,
}

pub struct Standby;

impl Command for Standby {
    type Action = StandbyAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x01;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            StandbyAction::Disable => vec![0x00],
            StandbyAction::Enable => vec![0x01],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === Input Select (0x02) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Toggle,
    Video(u8),       // 1-3
    Component(u8),   // 1-3
    Hdmi(u8),        // 1-5
    Pc(u8),          // 1
    SharedInput(u8), // 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputState {
    pub input_type: u8,
    pub input_num: u8,
}

pub struct InputSelect;

impl Command for InputSelect {
    type Action = InputType;
    type Response = InputState;

    const FUNCTION_CODE: u8 = 0x02;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            InputType::Toggle => vec![0x00],
            InputType::Video(n) => vec![0x02, *n],
            InputType::Component(n) => vec![0x03, *n],
            InputType::Hdmi(n) => vec![0x04, *n],
            InputType::Pc(n) => vec![0x05, *n],
            InputType::SharedInput(n) => vec![0x07, *n],
        }
    }

    fn parse_response(bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        if bytes.len() >= 2 {
            Ok(InputState {
                input_type: bytes[0],
                input_num: bytes[1],
            })
        } else {
            Err(ProtocolError::InvalidResponse)
        }
    }

    fn supports_query() -> bool {
        true
    }
}

// === Volume Control (0x05) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeAction {
    Up,
    Down,
    Set(VolumeValue),
}

pub struct Volume;

impl Command for Volume {
    type Action = VolumeAction;
    type Response = u8;

    const FUNCTION_CODE: u8 = 0x05;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            VolumeAction::Up => vec![0x00, 0x00],
            VolumeAction::Down => vec![0x00, 0x01],
            VolumeAction::Set(v) => vec![0x01, v.get()],
        }
    }

    fn parse_response(bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        bytes.first().copied().ok_or(ProtocolError::InvalidResponse)
    }

    fn supports_query() -> bool {
        true
    }
}

// === Muting (0x06) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MuteAction {
    Toggle,
    Unmute,
    Mute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MuteState {
    Unmuted,
    Muted,
}

pub struct Muting;

impl Command for Muting {
    type Action = MuteAction;
    type Response = MuteState;

    const FUNCTION_CODE: u8 = 0x06;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            MuteAction::Toggle => vec![0x00],
            MuteAction::Unmute => vec![0x01, 0x00],
            MuteAction::Mute => vec![0x01, 0x01],
        }
    }

    fn parse_response(bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        match bytes.first() {
            Some(0x00) => Ok(MuteState::Unmuted),
            Some(0x01) => Ok(MuteState::Muted),
            _ => Err(ProtocolError::InvalidResponse),
        }
    }

    fn supports_query() -> bool {
        true
    }
}

// === Off Timer / Sleep (0x0C) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepAction {
    Toggle,
    Set(SleepMinutes),
}

pub struct OffTimer;

impl Command for OffTimer {
    type Action = SleepAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x0C;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            SleepAction::Toggle => vec![0x00],
            SleepAction::Set(m) => vec![0x01, m.get()],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === Picture OFF (0x0D) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PictureOffAction {
    Toggle,
    Off,
    On,
}

pub struct PictureOff;

impl Command for PictureOff {
    type Action = PictureOffAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x0D;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            PictureOffAction::Toggle => vec![0x00],
            PictureOffAction::Off => vec![0x01, 0x00],
            PictureOffAction::On => vec![0x01, 0x01],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === Display (0x0F) ===

pub struct Display;

impl Command for Display {
    type Action = ();
    type Response = ();

    const FUNCTION_CODE: u8 = 0x0F;

    fn build_bytes(_action: &Self::Action) -> Vec<u8> {
        vec![0x00]
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
    fn test_power_build_bytes() {
        assert_eq!(Power::build_bytes(&PowerAction::Off), vec![0x00]);
        assert_eq!(Power::build_bytes(&PowerAction::On), vec![0x01]);
    }

    #[test]
    fn test_power_parse_response() {
        assert_eq!(Power::parse_response(&[0x00]).unwrap(), PowerState::Off);
        assert_eq!(Power::parse_response(&[0x01]).unwrap(), PowerState::On);
        assert!(Power::parse_response(&[0x02]).is_err());
    }

    #[test]
    fn test_input_select_build_bytes() {
        assert_eq!(InputSelect::build_bytes(&InputType::Toggle), vec![0x00]);
        assert_eq!(
            InputSelect::build_bytes(&InputType::Hdmi(1)),
            vec![0x04, 0x01]
        );
        assert_eq!(
            InputSelect::build_bytes(&InputType::Hdmi(3)),
            vec![0x04, 0x03]
        );
    }

    #[test]
    fn test_volume_build_bytes() {
        assert_eq!(Volume::build_bytes(&VolumeAction::Up), vec![0x00, 0x00]);
        assert_eq!(Volume::build_bytes(&VolumeAction::Down), vec![0x00, 0x01]);
        let vol = VolumeValue::new(50).unwrap();
        assert_eq!(Volume::build_bytes(&VolumeAction::Set(vol)), vec![0x01, 50]);
    }

    #[test]
    fn test_mute_build_bytes() {
        assert_eq!(Muting::build_bytes(&MuteAction::Toggle), vec![0x00]);
        assert_eq!(Muting::build_bytes(&MuteAction::Mute), vec![0x01, 0x01]);
        assert_eq!(Muting::build_bytes(&MuteAction::Unmute), vec![0x01, 0x00]);
    }
}
