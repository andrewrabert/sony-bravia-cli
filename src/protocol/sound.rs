// src/protocol/sound.rs
use crate::protocol::{Command, ProtocolError};

// === Sound Mode (0x30) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundModeAction {
    Toggle,
    Standard,
    Cinema,
    Sports,
    Music,
    Game,
}

pub struct SoundMode;

impl Command for SoundMode {
    type Action = SoundModeAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x30;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            SoundModeAction::Toggle => vec![0x00],
            SoundModeAction::Standard => vec![0x01, 0x01],
            SoundModeAction::Cinema => vec![0x01, 0x04],
            SoundModeAction::Sports => vec![0x01, 0x05],
            SoundModeAction::Music => vec![0x01, 0x06],
            SoundModeAction::Game => vec![0x01, 0x07],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === Speaker Off (0x36) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeakerAction {
    Toggle,
    On,
    Off,
}

pub struct Speaker;

impl Command for Speaker {
    type Action = SpeakerAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x36;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            SpeakerAction::Toggle => vec![0x00],
            SpeakerAction::On => vec![0x01, 0x00], // Speaker OFF = Off means speakers ON
            SpeakerAction::Off => vec![0x01, 0x01], // Speaker OFF = On means speakers OFF
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
    fn test_sound_mode_build_bytes() {
        assert_eq!(SoundMode::build_bytes(&SoundModeAction::Toggle), vec![0x00]);
        assert_eq!(
            SoundMode::build_bytes(&SoundModeAction::Standard),
            vec![0x01, 0x01]
        );
        assert_eq!(
            SoundMode::build_bytes(&SoundModeAction::Cinema),
            vec![0x01, 0x04]
        );
        assert_eq!(
            SoundMode::build_bytes(&SoundModeAction::Game),
            vec![0x01, 0x07]
        );
    }

    #[test]
    fn test_speaker_build_bytes() {
        assert_eq!(Speaker::build_bytes(&SpeakerAction::Toggle), vec![0x00]);
        assert_eq!(Speaker::build_bytes(&SpeakerAction::On), vec![0x01, 0x00]);
        assert_eq!(Speaker::build_bytes(&SpeakerAction::Off), vec![0x01, 0x01]);
    }
}
