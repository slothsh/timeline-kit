#![allow(dead_code)]

// Video Format Specifiers
#[derive(Debug, Default, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum FrameRate {
    Fps24DropFrame,
    Fps24,
    #[default]
    Fps25,
    Fps30,
    Fps30DropFrame,
    Fps48,
    Fps50,
    Fps60,
    Fps60DropFrame,
    Fps120,
}

impl FrameRate {
    pub fn as_usize(&self) -> usize {
        match self {
            FrameRate::Fps24DropFrame => 24,
            FrameRate::Fps24 => 24,
            FrameRate::Fps25 => 25,
            FrameRate::Fps30 => 30,
            FrameRate::Fps30DropFrame => 30,
            FrameRate::Fps48 => 48,
            FrameRate::Fps50 => 50,
            FrameRate::Fps60 => 60,
            FrameRate::Fps60DropFrame => 60,
            FrameRate::Fps120 => 120,
        }
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
