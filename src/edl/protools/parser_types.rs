// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLParser` Global Constants --
//
///////////////////////////////////////////////////////////////////////////

pub(super) const EDL_HEADER_LINE_SIZE: u32 = 8;
pub(super) const EDL_TRACK_LISTING_LINE_SIZE: u32 = 4;
pub(super) const EDL_SECTION_TERMINATOR_LENGTH: u32 = 2;
pub(super) const EDL_FIELD_PARTS_LENGTH: u32 = 2;
pub(super) const EDL_FIELD_NAME_INDEX: usize = 0;
pub(super) const EDL_FIELD_VALUE_INDEX: usize = 1;
pub(super) const EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS: [usize; 4] = [2, 6, 7, 8];
pub(super) const EDLPARSER_MASK_SECTION_PLUGINSLISTING: u8 = 0b00000001;

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLSection` Implementation --
//
///////////////////////////////////////////////////////////////////////////

pub(super) const EDLSECTION_SIZE: usize = EDLSection::Unknown as usize + 1;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub(super) enum EDLSection {
    Header,
    OnlineFiles,
    OfflineFiles,
    OnlineClips,
    PluginsListing,
    TrackListing,
    MarkersListing,
    #[default]
    Unknown,
}

impl EDLSection {
    pub(super) const fn section_name(&self) -> &'static str {
        match self {
            EDLSection::Header => "__header__",
            EDLSection::PluginsListing => "P L U G - I N S  L I S T I N G",
            EDLSection::OnlineFiles => "O N L I N E  F I L E S  I N  S E S S I O N",
            EDLSection::OfflineFiles => "O F F L I N E  F I L E S  I N  S E S S I O N",
            EDLSection::OnlineClips => "O N L I N E  C L I P S  I N  S E S S I O N",
            EDLSection::TrackListing => "T R A C K  L I S T I N G",
            EDLSection::MarkersListing => "M A R K E R S  L I S T I N G",
            EDLSection::Unknown => "__unknown__",
        }
    }

    pub(super) const fn all_variants() -> &'static [EDLSection; EDLSECTION_SIZE] {
        use EDLSection::*;
        &[
            Header,
            PluginsListing,
            OnlineFiles,
            OfflineFiles,
            OnlineClips,
            TrackListing,
            MarkersListing,
            Unknown,
        ]
    }

    pub(super) const fn as_usize(self) -> usize {
        self as usize
    }
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLTrackEventColumn` Implementation --
//
///////////////////////////////////////////////////////////////////////////

pub(super) const EDLTRACKEVENTCOLUMN_SIZE: usize = EDLTrackEventColumn::State as usize + 1;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(super) enum EDLTrackEventColumn {
    Channel,
    Event,
    ClipName,
    StartTime,
    EndTime,
    Duration,
    Timestamp,
    State,
}

impl EDLTrackEventColumn {
    fn column_name(&self) -> &'static str {
        match self {
            EDLTrackEventColumn::Channel => "CHANNEL",
            EDLTrackEventColumn::Event => "EVENT",
            EDLTrackEventColumn::ClipName => "CLIP NAME",
            EDLTrackEventColumn::StartTime => "START TIME",
            EDLTrackEventColumn::EndTime => "END TIME",
            EDLTrackEventColumn::Duration => "DURATION",
            EDLTrackEventColumn::Timestamp => "TIMESTAMP",
            EDLTrackEventColumn::State => "STATE",
        }
    }
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLField` Implementation --
//
///////////////////////////////////////////////////////////////////////////

pub(super) const EDLFIELD_SIZE: usize = EDLField::Unknown as usize + 1;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(super) enum EDLField {
    SessionName,
    SessionSampleRate,
    SessionBitDepth,
    SessionStartTimecode,
    SessionTimecodeFormat,
    SessionNumAudioTracks,
    SessionNumAudioClips,
    SessionNumAudioFiles,
    TrackName,
    TrackComment,
    TrackDelay,
    TrackState,
    Unknown,
}

impl EDLField {
    pub(super) const fn field_name(&self) -> &'static str {
        match self {
            EDLField::SessionName => "SESSION NAME",
            EDLField::SessionSampleRate => "SAMPLE RATE",
            EDLField::SessionBitDepth => "BIT DEPTH",
            EDLField::SessionStartTimecode => "SESSION START TIMECODE",
            EDLField::SessionTimecodeFormat => "TIMECODE FORMAT",
            EDLField::SessionNumAudioTracks => "# OF AUDIO TRACKS",
            EDLField::SessionNumAudioClips => "# OF AUDIO CLIPS",
            EDLField::SessionNumAudioFiles => "# OF AUDIO FILES",
            EDLField::TrackName => "TRACK NAME",
            EDLField::TrackComment => "COMMENTS",
            EDLField::TrackDelay => "USER DELAY",
            EDLField::TrackState => "STATE",
            EDLField::Unknown => "__unknown__",
        }
    }

    pub(super) const fn all_variants() -> &'static [EDLField; EDLFIELD_SIZE] {
        use EDLField::*;
        &[
            SessionName,
            SessionSampleRate,
            SessionBitDepth,
            SessionStartTimecode,
            SessionTimecodeFormat,
            SessionNumAudioTracks,
            SessionNumAudioClips,
            SessionNumAudioFiles,
            TrackName,
            TrackComment,
            TrackDelay,
            TrackState,
            Unknown,
        ]
    }

    pub(super) const fn is_voidable(&self) -> bool {
        use EDLField::*;
        match self {
            TrackComment => true,
            TrackState => true,
            Unknown => true,
            _ => false,
        }
    }
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLValue` Implementation --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum EDLValue<'a> {
    Field(EDLField, &'a str),
}
