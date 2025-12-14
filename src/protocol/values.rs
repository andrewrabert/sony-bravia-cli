#![allow(dead_code)]

use crate::protocol::error::RangeError;

macro_rules! bounded_value {
    ($name:ident, $min:expr, $max:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(u8);

        impl $name {
            pub const MIN: u8 = $min;
            pub const MAX: u8 = $max;

            pub fn new(v: u8) -> Result<Self, RangeError> {
                if v < Self::MIN {
                    Err(RangeError::BelowMin {
                        min: Self::MIN,
                        got: v,
                    })
                } else if v > Self::MAX {
                    Err(RangeError::AboveMax {
                        max: Self::MAX,
                        got: v,
                    })
                } else {
                    Ok(Self(v))
                }
            }

            pub fn get(&self) -> u8 {
                self.0
            }
        }
    };
}

bounded_value!(BrightnessValue, 0, 50);
bounded_value!(ContrastValue, 0, 100);
bounded_value!(ColorValue, 0, 100);
bounded_value!(SharpnessValue, 0, 100);
bounded_value!(VolumeValue, 0, 100);
bounded_value!(HueValue, 0, 100);
bounded_value!(SleepMinutes, 0, 255);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brightness_valid() {
        assert!(BrightnessValue::new(0).is_ok());
        assert!(BrightnessValue::new(25).is_ok());
        assert!(BrightnessValue::new(50).is_ok());
    }

    #[test]
    fn test_brightness_invalid() {
        assert!(BrightnessValue::new(51).is_err());
        assert!(BrightnessValue::new(100).is_err());
    }

    #[test]
    fn test_volume_range() {
        assert!(VolumeValue::new(0).is_ok());
        assert!(VolumeValue::new(100).is_ok());
        assert!(VolumeValue::new(101).is_err());
    }
}
