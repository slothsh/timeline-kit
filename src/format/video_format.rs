// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

#![allow(dead_code)]

use crate::edl::EDLParseField;

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
