// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

#![allow(dead_code)]

// Video Format Specifiers
#[derive(Debug, Default, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum FrameRate {
    Fps24(bool),
    #[default]
    Fps25,
    Fps30(bool),
    Fps48,
    Fps50,
    Fps60(bool),
    Fps120,
}

impl FrameRate {
    pub fn as_float(&self) -> f32 {
        match self {
            &FrameRate::Fps24(is_dropframe) => if is_dropframe { 23.976 } else { 24.0 },
            &FrameRate::Fps25 => 25.0,
            &FrameRate::Fps30(is_dropframe) => if is_dropframe { 29.97 } else { 30.0 },
            &FrameRate::Fps48 => 48.0,
            &FrameRate::Fps50 => 50.0,
            &FrameRate::Fps60(is_dropframe) => if is_dropframe { 59.94 } else { 60.0 },
            &FrameRate::Fps120 => 120.0,
        }
    }

    pub fn from_str(fps_string: &str) -> Result<Self, ()> { // TODO: Better error reporting
        match fps_string.trim() {
            "23.976" => Ok(FrameRate::Fps24(true)),
            "24" => Ok(FrameRate::Fps24(false)),
            "25" => Ok(FrameRate::Fps25),
            "29.97" => Ok(FrameRate::Fps30(true)),
            "30" => Ok(FrameRate::Fps30(false)),
            "48" => Ok(FrameRate::Fps48),
            "50" => Ok(FrameRate::Fps50),
            "59.94" => Ok(FrameRate::Fps60(true)),
            "60" => Ok(FrameRate::Fps60(false)),
            "120" => Ok(FrameRate::Fps120),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for FrameRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_float())
    }
}

// Audio Format Specifiers
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum SampleRate {
    Khz22,
    Khz44p1,
    Khz48,
    Khz88p1,
    Khz96,
    Khz192,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum BitDepth {
    Bit8,
    Bit16,
    Bit24,
    Bit32,
    Bit32Float,
    Bit64,
    Bit64Float,
}
