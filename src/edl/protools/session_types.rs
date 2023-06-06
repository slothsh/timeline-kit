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

impl EDLMediaFile {
    pub fn new(file_name: &str, location: &str) -> Self {
        Self {
            file_name: file_name.to_string(),
            location: location.to_string(),
        }
    }
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

impl EDLClip {
    pub fn new(clip_name: &str, source_file: &str) -> Self {
        Self {
            clip_name: clip_name.to_string(),
            source_file: source_file.to_string(),
        }
    }
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

#[derive(Debug, PartialEq, PartialOrd, Ord, Clone, Eq)]
pub struct EDLTrack {
    pub name: String,
    pub comment: String,
    pub delay: u32,
    pub state: (),
    pub events: Vec<EDLTrackEvent>,
}

impl EDLTrack {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            comment: "".to_string(),
            delay: 0,
            state: (),
            events: Vec::<EDLTrackEvent>::with_capacity(16),
        }
    }

    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Self::new()
        }
    }
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLEvent` Implementation --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, PartialOrd, Ord, Clone, Eq)]
pub struct EDLTrackEvent {
    pub channel: u32,
    pub event: u32,
    pub name: String,
    pub time_in: Timecode,
    pub time_out: Timecode,
    pub state: bool,
}

impl EDLTrackEvent {
    pub fn new() -> Self {
        Self {
            channel: 0,
            event: 0,
            name: "".to_string(),
            time_in: Timecode::default(),
            time_out: Timecode::default(),
            state: false,
        }
    }
}

impl<'a> From<(&'a [&'a str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]], FrameRate)> for EDLTrackEvent {
    fn from((event_values, frame_rate): (&'a [&'a str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]], FrameRate)) -> Self {
        let state_index = EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1;
        Self {
            channel: event_values[0].parse::<u32>().expect("first column of event entry must be a valid number"),
            event: event_values[1].parse::<u32>().expect("second column of event entry must be a valid number"),
            name: event_values[2].to_string(),
            time_in: Timecode::from_str(event_values[3], frame_rate).expect("timecode-in column of event entry must be a valid timecode string"),
            time_out: Timecode::from_str(event_values[4], frame_rate).expect("timecode-out column of event entry must be a valid timecode string"),
            state: event_values.as_slice()[state_index..]
                               .iter()
                               .take(1)
                               .map(|&v| if v == "Unmuted" { true } else { false })
                               .nth(0)
                               .expect(format!("track event value must be one of either \"Muted\" or \"Unmuted\", but instead found: {}", event_values[6]).as_str())
        }
    }
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

impl EDLMarker {
    pub fn new(id: u32, location: Timecode, time_reference: u32, unit: EDLUnit, name: String, comment: String) -> Self {
        Self {
            id,
            location,
            time_reference,
            unit,
            name,
            comment,
        }
    }
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
    #[default]
    Samples,
}

impl EDLUnit {
    pub fn from_str(unit_string: &str) -> Option<Self> {
        match unit_string.trim() {
            "Samples" => Some(EDLUnit::Samples),
            _ => None,
        }
    }
}
