// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

#![allow(unused_braces)]

mod audio_format;
mod video_format;

pub use audio_format::{
    SampleRate,
    BitDepth,
};

pub use video_format::{
    FrameRate,
};
