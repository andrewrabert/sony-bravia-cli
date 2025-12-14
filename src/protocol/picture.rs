#![allow(dead_code)]

use crate::protocol::{
    BrightnessValue, ColorValue, Command, ContrastValue, HueValue, ProtocolError, SharpnessValue,
};

// === Picture Mode (0x20) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PictureModeAction {
    Toggle,
    Vivid,
    Standard,
    Cinema,
    Custom,
    Game,
    Graphics,
}

pub struct PictureMode;

impl Command for PictureMode {
    type Action = PictureModeAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x20;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            PictureModeAction::Toggle => vec![0x00],
            PictureModeAction::Vivid => vec![0x01, 0x00],
            PictureModeAction::Standard => vec![0x01, 0x01],
            PictureModeAction::Cinema => vec![0x01, 0x02],
            PictureModeAction::Custom => vec![0x01, 0x03],
            PictureModeAction::Game => vec![0x01, 0x08],
            PictureModeAction::Graphics => vec![0x01, 0x09],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === Picture/Contrast (0x23) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContrastAction {
    Up,
    Down,
    Set(ContrastValue),
}

pub struct Contrast;

impl Command for Contrast {
    type Action = ContrastAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x23;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            ContrastAction::Up => vec![0x00, 0x00],
            ContrastAction::Down => vec![0x00, 0x01],
            ContrastAction::Set(v) => vec![0x01, v.get()],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === Brightness (0x24) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrightnessAction {
    Up,
    Down,
    Set(BrightnessValue),
}

pub struct Brightness;

impl Command for Brightness {
    type Action = BrightnessAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x24;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            BrightnessAction::Up => vec![0x00, 0x00],
            BrightnessAction::Down => vec![0x00, 0x01],
            BrightnessAction::Set(v) => vec![0x01, v.get()],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === Color (0x25) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorAction {
    Up,
    Down,
    Set(ColorValue),
}

pub struct Color;

impl Command for Color {
    type Action = ColorAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x25;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            ColorAction::Up => vec![0x00, 0x00],
            ColorAction::Down => vec![0x00, 0x01],
            ColorAction::Set(v) => vec![0x01, v.get()],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === Hue (0x26) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HueChannel {
    Red,
    Green,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HueAction {
    Up(HueChannel),
    Down(HueChannel),
    Set(HueChannel, HueValue),
}

pub struct Hue;

impl Command for Hue {
    type Action = HueAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x26;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            HueAction::Up(HueChannel::Red) => vec![0x00, 0x00, 0x00],
            HueAction::Down(HueChannel::Red) => vec![0x00, 0x00, 0x01],
            HueAction::Up(HueChannel::Green) => vec![0x00, 0x01, 0x00],
            HueAction::Down(HueChannel::Green) => vec![0x00, 0x01, 0x01],
            HueAction::Set(HueChannel::Red, v) => vec![0x01, 0x00, v.get()],
            HueAction::Set(HueChannel::Green, v) => vec![0x01, 0x01, v.get()],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === Sharpness (0x28) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharpnessAction {
    Up,
    Down,
    Set(SharpnessValue),
}

pub struct Sharpness;

impl Command for Sharpness {
    type Action = SharpnessAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x28;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            SharpnessAction::Up => vec![0x00, 0x00],
            SharpnessAction::Down => vec![0x00, 0x01],
            SharpnessAction::Set(v) => vec![0x01, v.get()],
        }
    }

    fn parse_response(_bytes: &[u8]) -> Result<Self::Response, ProtocolError> {
        Ok(())
    }

    fn supports_query() -> bool {
        false
    }
}

// === Cine Motion (0x2A) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CineMotionAction {
    Off,
    Auto,
}

pub struct CineMotion;

impl Command for CineMotion {
    type Action = CineMotionAction;
    type Response = ();

    const FUNCTION_CODE: u8 = 0x2A;

    fn build_bytes(action: &Self::Action) -> Vec<u8> {
        match action {
            CineMotionAction::Off => vec![0x00],
            CineMotionAction::Auto => vec![0x01],
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
    fn test_picture_mode_build_bytes() {
        assert_eq!(
            PictureMode::build_bytes(&PictureModeAction::Toggle),
            vec![0x00]
        );
        assert_eq!(
            PictureMode::build_bytes(&PictureModeAction::Vivid),
            vec![0x01, 0x00]
        );
        assert_eq!(
            PictureMode::build_bytes(&PictureModeAction::Cinema),
            vec![0x01, 0x02]
        );
        assert_eq!(
            PictureMode::build_bytes(&PictureModeAction::Game),
            vec![0x01, 0x08]
        );
    }

    #[test]
    fn test_brightness_build_bytes() {
        assert_eq!(
            Brightness::build_bytes(&BrightnessAction::Up),
            vec![0x00, 0x00]
        );
        assert_eq!(
            Brightness::build_bytes(&BrightnessAction::Down),
            vec![0x00, 0x01]
        );
        let val = BrightnessValue::new(25).unwrap();
        assert_eq!(
            Brightness::build_bytes(&BrightnessAction::Set(val)),
            vec![0x01, 25]
        );
    }

    #[test]
    fn test_hue_build_bytes() {
        assert_eq!(
            Hue::build_bytes(&HueAction::Up(HueChannel::Red)),
            vec![0x00, 0x00, 0x00]
        );
        assert_eq!(
            Hue::build_bytes(&HueAction::Down(HueChannel::Green)),
            vec![0x00, 0x01, 0x01]
        );
        let val = HueValue::new(50).unwrap();
        assert_eq!(
            Hue::build_bytes(&HueAction::Set(HueChannel::Red, val)),
            vec![0x01, 0x00, 50]
        );
    }
}
