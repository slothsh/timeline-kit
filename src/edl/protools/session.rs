// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

use crate::edl::protools::*;
use crate::chrono::{
    BitDepth,
    FrameRate,
    Timecode,
};

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLSession` Implementation --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct EDLSession {
    pub name: String,
    pub sample_rate: f32,
    pub bit_depth: BitDepth,
    pub start_timecode: Timecode,
    pub fps: FrameRate,
    pub num_audio_tracks: u32,
    pub num_audio_clips: u32,
    pub num_audio_files: u32,
    pub files: EDLFileList,
    pub markers: Vec<EDLMarker>,
    pub plugins: Vec<EDLPlugin>,
    pub tracks: Vec<EDLTrack>,
}

impl EDLSession {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            sample_rate: 0.0,
            bit_depth: BitDepth::default(),
            start_timecode: Timecode::with_fps(FrameRate::default()),
            fps: FrameRate::default(),
            num_audio_tracks: 0,
            num_audio_clips: 0,
            num_audio_files: 0,
            files: EDLFileList::default(),
            markers: Vec::<EDLMarker>::default(),
            plugins: Vec::<EDLPlugin>::default(),
            tracks: Vec::<EDLTrack>::with_capacity(16),
        }
    }
}
