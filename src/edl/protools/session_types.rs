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

impl ParseTable<Self, ()> for EDLMediaFile {
    const TABLE_TOTAL_COLUMNS: usize = 2;
    fn parse_table(table_data: &[String], _: ()) -> Option<Vec<Self>> {
        let mut edl_media = Vec::<Self>::with_capacity(table_data.len());

        for (i, line) in table_data.iter().enumerate() {
            let parts = line.split("\t").into_iter().collect::<Vec<_>>();
            if parts.len() == Self::TABLE_TOTAL_COLUMNS && i > 0 {
                edl_media.push(
                    Self {
                        file_name: parts[0].trim().to_string(),
                        location: parts[1].trim().to_string(),
                    }
                );
            }

            else { /* TODO: Report? */ }
        }
        
        if edl_media.len() > 0 { return Some(edl_media); }

        None
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

impl ParseTable<Self, ()> for EDLClip {
    const TABLE_TOTAL_COLUMNS: usize = 2;
    fn parse_table(table_data: &[String], _: ()) -> Option<Vec<Self>> {
        let mut edl_clip = Vec::<Self>::with_capacity(table_data.len());

        for (i, line) in table_data.iter().enumerate() {
            let parts = line.split("\t").into_iter().collect::<Vec<_>>();
            if parts.len() == Self::TABLE_TOTAL_COLUMNS && i > 0 {
                edl_clip.push(
                    Self {
                        clip_name: parts[0].trim().to_string(),
                        source_file: parts[1].trim().to_string(),
                    }
                );
            }

            else { /* TODO: Report? */ }
        }
        
        if edl_clip.len() > 0 { return Some(edl_clip); }

        None
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

#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Clone, Eq)]
pub struct EDLTrack {
    pub name: String,
    pub comment: String,
    pub delay: u32,
    pub state: (),
    pub plugins: Vec<String>,
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
    pub timestamp: Timecode,
    pub state: bool,
    flags: u8,
}

impl EDLTrackEvent {
    // TODO: de-duplicate this code
    pub fn check_flag(&self, flag: u8) -> bool {
        self.flags & flag == flag
    }

    pub fn set_flag(&mut self, flag: u8) {
        self.flags |= flag;
    }

    pub fn reset_flag(&mut self, flag: u8) {
        self.flags &= !flag;
    }
}

impl ParseTable<Self, FrameRate> for EDLTrackEvent {
    const TABLE_TOTAL_COLUMNS: usize = 8;
    fn parse_table(table_data: &[String], default_frame_rate: FrameRate) -> Option<Vec<Self>> {
        let mut edl_events = Vec::<Self>::with_capacity(table_data.len());
        let mut contains_timestamp = false;

        for (i, line) in table_data.iter().enumerate() {
            let parts = line.split("\t").into_iter().collect::<Vec<_>>();

            if (parts.len() == Self::TABLE_TOTAL_COLUMNS || parts.len() == Self::TABLE_TOTAL_COLUMNS - 1) && i > 0 {
                let state =
                    if parts[parts.len() - 1].trim() == "Muted" {
                        true
                    } else {
                        false
                    };

                let timestamp =
                    if contains_timestamp {
                        Timecode::from_str(parts[parts.len() - 2].trim(), default_frame_rate).expect("EDLTrackEvent time in column should be a valid timecode string")
                    } else {
                        Timecode::with_fps(default_frame_rate)
                    };

                let edl_event = Self {
                    channel: parts[0].trim().parse::<u32>().expect("EDLTrackEvent channel column should be a valid number"),
                    event: parts[1].trim().parse::<u32>().expect("EDLTrackEvent event column should be a valid number"),
                    name: parts[2].trim().to_string(),
                    time_in: Timecode::from_str(parts[3].trim(), default_frame_rate).expect("EDLTrackEvent time in column should be a valid timecode string"),
                    time_out: Timecode::from_str(parts[4].trim(), default_frame_rate).expect("EDLTrackEvent time in column should be a valid timecode string"),
                    timestamp,
                    state,
                    ..Self::default()
                };

                edl_events.push(edl_event);
            }

            else if (parts.len() == Self::TABLE_TOTAL_COLUMNS || parts.len() == Self::TABLE_TOTAL_COLUMNS - 1) && i == 0 {
                contains_timestamp = parts[parts.len() - 2].trim() == "TIMESTAMP";
            }

            else { /* TODO: Report? */ }
        }
        
        if edl_events.len() > 0 { return Some(edl_events); }

        None
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

impl ParseTable<Self, FrameRate> for EDLMarker {
    const TABLE_TOTAL_COLUMNS: usize = 6;
    fn parse_table(table_data: &[String], default_frame_rate: FrameRate) -> Option<Vec<Self>> {
        let mut edl_markers = Vec::<Self>::with_capacity(table_data.len());

        for (i, line) in table_data.iter().enumerate() {
            let parts = line.split("\t").into_iter().collect::<Vec<_>>();
            if parts.len() == Self::TABLE_TOTAL_COLUMNS && i > 0 {
                edl_markers.push(
                    Self {
                        id: parts[0].trim().parse::<u32>().expect("EDLMarker id column should be a valid number"),
                        location: Timecode::from_str(parts[1].trim(), default_frame_rate).expect("EDLMarker location column should be a valid timecode string"),
                        time_reference: parts[2].trim().parse::<u32>().expect("EDLMarker time reference column should be a valid number"),
                        unit: EDLUnit::from_str(parts[3].trim()).expect("EDLMarker unit column should be valid unit option"),
                        name: parts[4].trim().to_string(),
                        comment: parts[5].trim().to_string(),
                    }
                );
            }

            else { /* TODO: Report? */ }
        }
        
        if edl_markers.len() > 0 { return Some(edl_markers); }

        None
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
    pub total_instances: String,
}

impl ParseTable<Self, ()> for EDLPlugin {
    const TABLE_TOTAL_COLUMNS: usize = 6;
    fn parse_table(table_data: &[String], _: ()) -> Option<Vec<Self>> {
        let mut edl_plugins = Vec::<Self>::with_capacity(table_data.len());

        for (i, line) in table_data.iter().enumerate() {
            let parts = line.split("\t").into_iter().collect::<Vec<_>>();
            if parts.len() == Self::TABLE_TOTAL_COLUMNS && i > 0 {
                edl_plugins.push(
                    EDLPlugin {
                        manufacturer: parts[0].trim().to_string(),
                        name: parts[1].trim().to_string(),
                        version: parts[2].trim().to_string(),
                        format: EDLPluginFormat::from_str(parts[3].trim()).expect("EDLPluginFormat should have a valid plugin format option"),
                        stems: parts[4].trim().to_string(),
                        ..EDLPlugin::default()
                    }
                );
            }

            else { /* TODO: Report? */ }
        }
        
        if edl_plugins.len() > 0 { return Some(edl_plugins); }

        None
    }
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
