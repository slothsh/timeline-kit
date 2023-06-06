// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

use crate::edl::protools::*;
use crate::chrono::{
    Timecode,
    FrameRate,
};

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLMediaFile` Implementation --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Clone, Eq)]
pub struct EDLMediaFile {
    pub file_name: String,
    pub location: String,
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLClip` Implementation --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Clone, Eq)]
pub struct EDLClip {
    pub clip_name: String,
    pub source_file: String,
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLFileList` Implementation --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, PartialOrd, Ord, Clone, Eq)]
pub struct EDLFileList {
    pub online_files: Vec<EDLMediaFile>,
    pub offline_files: Vec<EDLMediaFile>,
    pub online_clips: Vec<EDLClip>,
}

impl Default for EDLFileList {
    fn default() -> Self {
        Self {
            online_files: Vec::<_>::default(),
            offline_files: Vec::<_>::default(),
            online_clips: Vec::<_>::default(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLTrack` Implementation --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Clone, Eq)]
pub struct EDLTrack {
    pub name: String,
    pub comment: String,
    pub delay: u32,
    pub state: (),
    pub events: Vec<EDLTrackEvent>,
}

impl EDLTrack {
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Self::default()
        }
    }
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLEvent` Implementation --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Clone, Eq)]
pub struct EDLTrackEvent {
    pub channel: u32,
    pub event: u32,
    pub name: String,
    pub time_in: Timecode,
    pub time_out: Timecode,
    pub state: bool,
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLMarker` Implementation --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Clone, Eq)]
pub struct EDLMarker {
    pub id: u32,
    pub location: Timecode,
    pub time_reference: u32,
    pub unit: EDLUnit,
    pub name: String,
    pub comment: String,
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLUnit` Implementation --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum EDLUnit {
    // TODO: Figure out what other units are acceptable
    // in Protools EDL
    BarsBeats,
    FeetFrames,
    MinutesSeconds,
    #[default]
    Samples,
    Timecode,
}

impl EDLUnit {
    pub fn from_str(unit_string: &str) -> Option<Self> {
        match unit_string.trim() {
            "Bars|Beats" => Some(EDLUnit::BarsBeats),
            "Feet+Frames" => Some(EDLUnit::FeetFrames),
            "Min:Sec" => Some(EDLUnit::MinutesSeconds),
            "Samples" => Some(EDLUnit::Samples),
            "Timecode" => Some(EDLUnit::Timecode),
            _ => None,
        }
    }
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLPlugin` Implementation --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct EDLPlugin {
    pub manufacturer: String,
    pub name: String,
    pub version: String,
    pub format: EDLPluginFormat,
    pub stems: String,
    pub total_instances: EDLPluginInstance,
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLPluginFormat` Implementation --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum EDLPluginFormat {
    // TODO: Figure out all possible formats
    #[default]
    AAXNative,
    AAXDSP,
}

impl EDLPluginFormat {
    pub fn from_str(format_string: &str) -> Option<Self> {
        match format_string.trim() {
            "AAX Native" => Some(EDLPluginFormat::AAXNative),
            "AAX DSP" => Some(EDLPluginFormat::AAXDSP),
            _ => None,
        }
    }
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLPluginInstance` Implementation --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct EDLPluginInstance {
    pub total_active: u32,
}
