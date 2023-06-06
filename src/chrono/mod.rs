// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

mod formats;
mod timecode;

pub use timecode::Timecode;
pub use formats::{
    FrameRate,
    SampleRate,
    BitDepth,
};
