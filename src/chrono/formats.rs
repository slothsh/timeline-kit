// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

#![allow(dead_code)]

use crate::edl::EDLParseField;

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
}

impl EDLParseField<Self> for FrameRate {
    fn parse_field(fps_string: &str) -> Option<Self> { // TODO: Better error reporting
        match fps_string.trim() {
            "23.976 Drop Frame" => Some(FrameRate::Fps24(true)),
            "24 Frame" => Some(FrameRate::Fps24(false)),
            "25 Frame" => Some(FrameRate::Fps25),
            "29.97 Drop Frame" => Some(FrameRate::Fps30(true)),
            "30 Frame" => Some(FrameRate::Fps30(false)),
            "48 Frame" => Some(FrameRate::Fps48),
            "50 Frame" => Some(FrameRate::Fps50),
            "59.94 Drop Frame" => Some(FrameRate::Fps60(true)),
            "60 Frame" => Some(FrameRate::Fps60(false)),
            "120 Frame" => Some(FrameRate::Fps120),
            _ => None,
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

#[derive(Debug, Default, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum BitDepth {
    Bit8,
    #[default]
    Bit16,
    Bit24,
    Bit32,
    Bit32Float,
    Bit64,
    Bit64Float,
}

impl EDLParseField<Self> for BitDepth {
    fn parse_field(field_string: &str) -> Option<Self> {
        match field_string.trim() {
            "8-bit" => Some(BitDepth::Bit8),
            "16-bit" => Some(BitDepth::Bit16),
            "24-bit" => Some(BitDepth::Bit24),
            "32-bit" => Some(BitDepth::Bit32),
            "32-bit float" => Some(BitDepth::Bit32Float),
            "64-bit" => Some(BitDepth::Bit64),
            "64-bit float" => Some(BitDepth::Bit64Float),
            _ => None,
        }
    }
}
