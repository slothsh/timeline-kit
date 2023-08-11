// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

#![allow(dead_code)]

use crate::edl::EDLParseField;

#[derive(Debug, Default, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum SampleRate {
    Khz22,
    #[default]
    Khz44p1,
    Khz48,
    Khz88p2,
    Khz96,
    Khz192,
}

impl EDLParseField<Self> for SampleRate {
    fn parse_field(field_string: &str) -> Option<Self> {
        match field_string.trim() {
            "22000.000000" => Some(SampleRate::Khz22),
            "44100.000000" => Some(SampleRate::Khz44p1),
            "48000.000000" => Some(SampleRate::Khz48),
            "88200.000000" => Some(SampleRate::Khz88p2),
            "96000.000000" => Some(SampleRate::Khz96),
            "192000.000000" => Some(SampleRate::Khz192),
            _ => None,
        }
    }
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
