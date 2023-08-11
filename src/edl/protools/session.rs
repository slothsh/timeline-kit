// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

use crate::edl::protools::*;
use crate::chrono::{
    Timecode,
};
use crate::format::{
    BitDepth,
    FrameRate,
    SampleRate,
};

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLSession` Implementation --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct EDLSession {
    pub name: String,
    pub sample_rate: SampleRate,
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
    flags: u64,
}

impl EDLSession {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            sample_rate: SampleRate::default(),
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
            flags: EDLSESSION_FLAG_DEFAULT,
        }
    }

    pub fn check_flag(&self, flag: u64) -> bool {
        self.flags & flag == flag
    }

    pub fn set_flag(&mut self, flag: u64) {
        self.flags |= flag;
    }

    pub fn reset_flag(&mut self, flag: u64) {
        self.flags &= !flag;
    }
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLSession` Flags --
//
///////////////////////////////////////////////////////////////////////////

pub const EDLSESSION_FLAG_DEFAULT: u64 = 0;
pub const EDLSESSION_FLAG_CONTAINS_PLUGIN: u64 = 1 << 1;
