#![allow(unused_imports, unused_variables, unused_mut, dead_code)]

use encoding_rs_io::DecodeReaderBytesBuilder;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::{println, marker};
use crate::chrono::Timecode;
use crate::chrono::FrameRate;

const EDL_HEADER_LINE_SIZE: u32 = 8;
const EDL_TRACK_LISTING_LINE_SIZE: u32 = 4;
const EDL_SECTION_TERMINATOR_LENGTH: u32 = 2;
const EDL_FIELD_PARTS_LENGTH: u32 = 2;
const EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS: [usize; 4] = [2, 6, 7, 8];
const EDLPARSER_MASK_SECTION_PLUGINSLISTING: u8 = 0b00000001;

const EDLSECTION_SIZE: usize = EDLSection::Ignore as usize + 1;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
enum EDLSection {
    Header,
    PluginsListing,
    OnlineFiles,
    OfflineFiles,
    OnlineClips,
    TrackListing,
    TrackEvent,
    MarkersListing,
    #[default]
    Ignore,
}

impl EDLSection {
    fn section_name(&self) -> &'static str {
        match self {
            EDLSection::Header => "__header__",
            EDLSection::PluginsListing => "P L U G - I N S  L I S T I N G",
            EDLSection::OnlineFiles => "O N L I N E  F I L E S  I N  S E S S I O N",
            EDLSection::OfflineFiles => "O F F L I N E  F I L E S  I N  S E S S I O N",
            EDLSection::OnlineClips => "O N L I N E  C L I P S  I N  S E S S I O N",
            EDLSection::TrackListing => "T R A C K  L I S T I N G",
            EDLSection::TrackEvent => "__track_event__",
            EDLSection::MarkersListing => "M A R K E R S  L I S T I N G",
            EDLSection::Ignore => "__ignore__",
        }
    }

    const fn all_variants() -> &'static [EDLSection; EDLSECTION_SIZE] {
        use EDLSection::*;
        &[
            Header,
            PluginsListing,
            OnlineFiles,
            OfflineFiles,
            OnlineClips,
            TrackListing,
            TrackEvent,
            MarkersListing,
            Ignore,
        ]
    }

    const fn as_usize(self) -> usize {
        self as usize
    }
}

const EDLTRACKEVENTCOLUMN_SIZE: usize = EDLTrackEventColumn::State as usize + 1;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum EDLTrackEventColumn {
    Channel,
    Event,
    ClipName,
    StartTime,
    EndTime,
    Duration,
    Timestamp,
    State,
}

const EDLFIELD_SIZE: usize = EDLField::Unknown as usize + 1;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum EDLField {
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
    fn field_name(&self) -> &'static str {
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

    const fn all_variants() -> &'static [EDLField; EDLFIELD_SIZE] {
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

    const fn is_voidable(&self) -> bool {
        use EDLField::*;
        match self {
            TrackComment => true,
            TrackState => true,
            Unknown => true,
            _ => false,
        }
    }
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum EDLValue<'a> {
    Field(EDLSection, EDLField, &'a str),
    TableHeader(EDLSection, &'a [&'a str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]]),
    TableEntry(EDLSection, &'a [&'a str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]]),
}

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

#[derive(Debug, Default)]
pub struct EDLParser<'a> {
    file_path: &'a str,
    section_flags: u8,
    file_position: usize,
    section_position: usize,
    trailing_new_lines: usize,
    current_section: EDLSection,
    previous_section: EDLSection,
    section_ended: bool,
}

impl<'a> EDLParser<'a> {
    pub fn parse(input_path: &'a str, encoding: &'static encoding_rs::Encoding) -> Result<EDLSession, String> {
        // TODO: Putting parsing into separate parsing function to be
        // used by other constructor type functions

        let mut edl_parser = EDLParser {
            file_path: input_path,
            current_section: EDLSection::Header,
            section_ended: false,
            ..EDLParser::default()
        };

        let input_file = File::open(input_path).unwrap();
        let input_file_decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(encoding))
            .build(input_file);
        let input_file_handle = BufReader::new(input_file_decoder);
        let mut all_lines = input_file_handle.lines();

        let mut edl_session = EDLSession::new();
        let mut current_track: Option<EDLTrack> = None;

        while let Some(line_result) = all_lines.next() {
            match line_result {
                Ok(line) => {
                    let trimmed_line = line.trim();
                    if trimmed_line == "" { edl_parser.trailing_new_lines += 1; }

                    let (current_section, skip_line, reset_section) = edl_parser.check_section(trimmed_line)
                        .unwrap_or((EDLSection::Ignore, true, edl_parser.trailing_new_lines >= EDL_SECTION_TERMINATOR_LENGTH as usize));

                    if current_section != EDLSection::Ignore { edl_parser.current_section = current_section; edl_parser.section_ended = false; }
                    else { edl_parser.section_ended = true; }

                    if reset_section {
                        edl_parser.section_position = 0;
                        edl_parser.trailing_new_lines = 0;
                        edl_parser.section_ended = false;

                        if edl_parser.current_section == EDLSection::TrackEvent {
                            if let Some(track) = &current_track {
                                edl_session.tracks.push(track.clone());
                            }

                            current_track = None;
                        }
                    }

                    if !skip_line {
                        edl_parser.section_position += 1;

                        if current_section == EDLSection::Header {
                            if let EDLValue::Field(section, field, value) = edl_parser.parse_edl_field(trimmed_line)? {
                                match field {
                                    EDLField::SessionName           => edl_session.name             = value.to_string(),
                                    EDLField::SessionSampleRate     => edl_session.sample_rate      = value.parse::<f32>().expect("sample rate field must be valid float-point number"),
                                    EDLField::SessionBitDepth       => edl_session.bit_depth        = value.strip_suffix("-bit").unwrap_or(value).parse::<u32>().expect("bit depth field must be valid integral number"),
                                    EDLField::SessionStartTimecode  => edl_session.start_timecode   = Timecode::from_str(value, FrameRate::default()).expect("session start timecode field must be a valid frame-rate string"),
                                    EDLField::SessionTimecodeFormat => {
                                        edl_session.fps = FrameRate::from_str(value.strip_suffix(Self::select_fps_suffix(value).unwrap()).expect("session timecode format must specify unit type")).expect("session timecode format field must be a valid frame-rate string");
                                        edl_session.start_timecode.set_frame_rate(edl_session.fps);
                                    },
                                    EDLField::SessionNumAudioTracks => edl_session.num_audio_tracks = value.parse::<u32>().expect("number of audio tracks field must be valid integral number"),
                                    EDLField::SessionNumAudioClips  => edl_session.num_audio_clips  = value.parse::<u32>().expect("number of audio clips field must be valid integral number"),
                                    EDLField::SessionNumAudioFiles  => edl_session.num_audio_files  = value.parse::<u32>().expect("number of audio files field must be valid integral number"),
                                    _ => eprintln!("parsing not implemented for field \"{:?}\" with value {}", field, value),
                                }
                            }
                        }

                        else if current_section == EDLSection::OnlineFiles {
                            let mut online_file_value_buffer: [&str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]] = [""; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]];
                            let data = edl_parser.parse_edl_online_file(trimmed_line, &mut online_file_value_buffer)?;

                            match data {
                                EDLValue::TableHeader(section, columns) => {
                                    // TODO: Maybe validate names of columns?
                                    // TODO: Set state for event table width?
                                }

                                EDLValue::TableEntry(section, cells) => {
                                    let online_file = EDLMediaFile::new(cells[0], cells[1]);
                                    edl_session.files.online_files.push(online_file);
                                }

                                _ => panic!("failed to parse EDLValue token, an unexpected or invalid token was encountered when parsing an EDL online file entry"),
                            }
                        }

                        else if current_section == EDLSection::OfflineFiles {
                            let mut offline_file_value_buffer: [&str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]] = [""; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]];
                            let data = edl_parser.parse_edl_offline_file(trimmed_line, &mut offline_file_value_buffer)?;

                            match data {
                                EDLValue::TableHeader(section, columns) => {
                                    // TODO: Maybe validate names of columns?
                                    // TODO: Set state for event table width?
                                }

                                EDLValue::TableEntry(section, cells) => {
                                    let offline_file = EDLMediaFile::new(cells[0], cells[1]);
                                    edl_session.files.offline_files.push(offline_file);
                                }

                                _ => panic!("failed to parse EDLValue token, an unexpected or invalid token was encountered when parsing an EDL offline file entry"),
                            }
                        }

                        else if current_section == EDLSection::OnlineClips {
                            let mut online_clip_value_buffer: [&str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]] = [""; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]];
                            let data = edl_parser.parse_edl_online_clip(trimmed_line, &mut online_clip_value_buffer)?;

                            match data {
                                EDLValue::TableHeader(section, columns) => {
                                    // TODO: Maybe validate names of columns?
                                    // TODO: Set state for event table width?
                                }

                                EDLValue::TableEntry(section, cells) => {
                                    let online_clip = EDLClip::new(cells[0], cells[1]);
                                    edl_session.files.online_clips.push(online_clip);
                                }

                                _ => panic!("failed to parse EDLValue token, an unexpected or invalid token was encountered when parsing an EDL online clip entry"),
                            }
                        }

                        else if current_section == EDLSection::TrackListing {
                            if let EDLValue::Field(section, field, value) = edl_parser.parse_edl_field(trimmed_line)? {
                                if let Some(track) = &mut current_track {
                                    match field {
                                        EDLField::TrackName    => track.name    = value.to_string(),
                                        EDLField::TrackComment => track.comment = value.to_string(),
                                        EDLField::TrackDelay   => track.delay   = value.strip_suffix(" Samples").unwrap_or(value).parse::<u32>().expect("bit depth field must be valid integral number"),
                                        EDLField::TrackState   => track.state   = (),
                                        _ => eprintln!("parsing not implemented for field \"{:?}\" with value {}", field, value),
                                    }
                                }

                                else if field == EDLField::TrackName {
                                    current_track = Some(EDLTrack::with_name(value));
                                }

                                else {
                                    panic!("cannot create a new EDLTrack without specifying a name first: the header of the current track, \"{}\", is most likely in the incorrect order.", value);
                                }
                            }
                        }

                        else if current_section == EDLSection::TrackEvent {
                            let mut event_value_buffer: [&str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]] = [""; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]];
                            let data = edl_parser.parse_edl_track_event(trimmed_line, &mut event_value_buffer)?;
                            match data {
                                EDLValue::TableHeader(section, columns) => {
                                    // TODO: Maybe validate names of columns?
                                    // TODO: Set state for event table width?
                                }

                                EDLValue::TableEntry(section, cells) => {
                                    if let Some(track) = &mut current_track {
                                        let event = EDLEvent::from((cells, edl_session.fps));
                                        track.events.push(event);
                                    } 

                                    else {
                                        panic!("failed to parse EDLValue table event token because the current track was invalid");
                                    }
                                }

                                _ => panic!("failed to parse EDLValue token, an unexpected or invalid token was encountered when parsing an EDL track event table"),
                            }
                        }

                        else if current_section == EDLSection::MarkersListing {
                            let mut marker_listing_value_buffer: [&str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]] = [""; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]];
                            // TODO: Find better way to ensure that line is not trimmed before
                            // being parsed
                            let data = edl_parser.parse_edl_marker_listing(line.as_str(), &mut marker_listing_value_buffer)?;

                            match data {
                                EDLValue::TableHeader(section, columns) => {
                                    // TODO: Maybe validate names of columns?
                                    // TODO: Set state for event table width?
                                }

                                EDLValue::TableEntry(section, cells) => {
                                    let marker = EDLMarker::new(
                                        cells[0].trim().parse::<u32>().expect("marker listing ID must be a valid number"),
                                        Timecode::from_str(cells[1].trim(), edl_session.fps).expect("marker listing timecode column must containt a valid timecode string"),
                                        cells[2].trim().parse::<u32>().expect("marker listing time reference must be a valid number"),
                                        EDLUnit::from_str(cells[3].trim()).expect("marker listing unit type must be a valid option"),
                                        cells[4].trim().to_string(),
                                        cells[5].trim().to_string(),
                                    );

                                    edl_session.markers.push(marker);
                                }

                                _ => panic!("failed to parse EDLValue token, an unexpected or invalid token was encountered when parsing an EDL marker listing entry"),
                            }
                        }
                    }
                }

                Err(error) => panic!("error: could not read line: {}", error),
            }

            edl_parser.file_position += 1;
        }

        Ok(edl_session)
    }

    fn check_section(&mut self, line: &str) -> Option<(EDLSection, bool, bool)> {
        use EDLSection::*;
        // let reset_section = if self.trailing_new_lines >= EDL_SECTION_TERMINATOR_LENGTH as usize { true } else { false };

        // An empty line should be ignored
        if line == "" {
            return None;
        }

        // Verify if overall file position is is less than the entire EDL Header section
        // since the header should be the first section in the EDL.
        // No need to check against the EDLSection
        if self.file_position < EDL_HEADER_LINE_SIZE as usize {
            return Some((Header, false, false));
        }

        // Handle the section declaration lines within the EDL file.
        // These lines declare the start of a new section, but should be skipped.
        for section_variant in EDLSection::all_variants() {
            // Modify behaviour of parser based on sections as they
            // are discovered.
            if line == section_variant.section_name() {
                self.enable_section(*section_variant);
                return Some((*section_variant, true, true));
            }
        }

        if self.section_ended { return Some((self.current_section, true, false)); }

        if self.current_section == OnlineFiles {
            return Some((OnlineFiles, false, false))
        }

        if self.current_section == OfflineFiles {
            return Some((OfflineFiles, false, false))
        }

        if self.current_section == OnlineClips {
            return Some((OnlineClips, false, false))
        }

        if self.current_section == TrackListing {
            // If within the track-listing section beyond the header size,
            // it means we're in the track event table of the current track
            const SECTION: usize = TrackListing.as_usize();
            let track_listing_size = self.get_section_size::<SECTION>();
            if self.section_position >= track_listing_size as usize {
                return Some((TrackEvent, false, false))
            }

            return Some((TrackListing, false, false));
        }

        if self.current_section == TrackEvent {
            let line_parts_count = line.split("\t").count();

            if line_parts_count == EDL_FIELD_PARTS_LENGTH as usize {
                return Some((TrackListing, false, true));
            }

            else if EDLParser::is_valid_event_table_column_width(line_parts_count) {
                return Some((TrackEvent, false, false));
            }
        }

        if self.current_section == MarkersListing {
            return Some((MarkersListing, false, false))
        }

        None
    }

    fn parse_edl_field<'z>(&mut self, line: &'z str) -> Result<EDLValue<'z>, String> {
        let line_parts = line.split(":\t").collect::<Vec<&'z str>>();

        for field in EDLField::all_variants() {
            if line_parts[0] == field.field_name() {
                if line_parts.len() == EDL_FIELD_PARTS_LENGTH as usize {
                    return Ok(EDLValue::Field(self.current_section, *field, line_parts[1]));
                }
            }

            else if let Some(_) = line_parts[0].rfind(":") {
                if field.is_voidable() {
                    return Ok(EDLValue::Field(self.current_section, *field, ""));
                }
            }
        }

        Err(format!("edl field could not be parsed, either because no recognized fields were found or the field format was incorrect: these are the fields that were parsed: {:?}", line_parts))
    }

    fn parse_edl_online_file<'z>(&mut self, line: &'z str, online_file_value_buffer: &'z mut [&'z str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]]) -> Result<EDLValue<'z>, String> {
        let online_file_cells = line.split("\t").collect::<Vec<&'z str>>();

        if EDLParser::is_valid_event_table_column_width(online_file_cells.len()) {
            for (i, value) in online_file_cells.iter().enumerate() {
                online_file_value_buffer[i] = value.trim();
            }

            const SECTION: usize = EDLSection::OnlineFiles.as_usize();
            if self.section_position > self.get_section_size::<SECTION>() + 1 {
                return Ok(EDLValue::TableEntry(self.current_section, online_file_value_buffer));
            }

            return Ok(EDLValue::TableHeader(self.current_section, online_file_value_buffer));
        }

        Err(format!("edl online file entry could not be parsed, either because there aren't enough columns, or because an invalid format was encountered: this was the event that caused an error: {:?}", line))
    }

    fn parse_edl_offline_file<'z>(&mut self, line: &'z str, offline_file_value_buffer: &'z mut [&'z str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]]) -> Result<EDLValue<'z>, String> {
        let offline_file_cells = line.split("\t").collect::<Vec<&'z str>>();

        if EDLParser::is_valid_event_table_column_width(offline_file_cells.len()) {
            for (i, value) in offline_file_cells.iter().enumerate() {
                offline_file_value_buffer[i] = value.trim();
            }

            const SECTION: usize = EDLSection::OfflineFiles.as_usize();
            if self.section_position > self.get_section_size::<SECTION>() + 1 {
                return Ok(EDLValue::TableEntry(self.current_section, offline_file_value_buffer));
            }

            return Ok(EDLValue::TableHeader(self.current_section, offline_file_value_buffer));
        }

        Err(format!("edl offline file entry could not be parsed, either because there aren't enough columns, or because an invalid format was encountered: this was the event that caused an error: {:?}", line))
    }

    fn parse_edl_online_clip<'z>(&mut self, line: &'z str, online_clip_value_buffer: &'z mut [&'z str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]]) -> Result<EDLValue<'z>, String> {
        let online_clip_cells = line.split("\t").collect::<Vec<&'z str>>();

        if EDLParser::is_valid_event_table_column_width(online_clip_cells.len()) {
            for (i, value) in online_clip_cells.iter().enumerate() {
                online_clip_value_buffer[i] = value.trim();
            }

            const SECTION: usize = EDLSection::OnlineClips.as_usize();
            if self.section_position > self.get_section_size::<SECTION>() + 1 {
                return Ok(EDLValue::TableEntry(self.current_section, online_clip_value_buffer));
            }

            return Ok(EDLValue::TableHeader(self.current_section, online_clip_value_buffer));
        }

        Err(format!("edl online clip entry could not be parsed, either because there aren't enough columns, or because an invalid format was encountered: this was the event that caused an error: {:?}", line))
    }

    fn parse_edl_track_event<'z>(&mut self, line: &'z str, event_value_buffer: &'z mut [&'z str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]]) -> Result<EDLValue<'z>, String>
    {
        let event_cells = line.split("\t").collect::<Vec<&'z str>>();

        if EDLParser::is_valid_event_table_column_width(event_cells.len()) {
            for (i, value) in event_cells.iter().enumerate() {
                event_value_buffer[i] = value.trim();
            }

            const SECTION: usize = EDLSection::TrackListing.as_usize();
            if self.section_position > self.get_section_size::<SECTION>() + 1 {
                return Ok(EDLValue::TableEntry(self.current_section, event_value_buffer));
            }

            return Ok(EDLValue::TableHeader(self.current_section, event_value_buffer));
        }

        Err(format!("edl track event could not be parsed, either because there aren't enough columns, or because an invalid format was encountered: this was the event that caused an error: {:?}", line))
    }

    fn parse_edl_marker_listing<'z>(&mut self, line: &'z str, marker_listing_value_buffer: &'z mut [&'z str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]]) -> Result<EDLValue<'z>, String> {
        let marker_listing_cells = line.split("\t").collect::<Vec<&'z str>>();

        if EDLParser::is_valid_event_table_column_width(marker_listing_cells.len()) {
            for (i, value) in marker_listing_cells.iter().enumerate() {
                marker_listing_value_buffer[i] = value.trim();
            }

            const SECTION: usize = EDLSection::MarkersListing.as_usize();
            if self.section_position > self.get_section_size::<SECTION>() + 1 {
                return Ok(EDLValue::TableEntry(self.current_section, marker_listing_value_buffer));
            }

            return Ok(EDLValue::TableHeader(self.current_section, marker_listing_value_buffer));
        }

        Err(format!("edl marker listing entry could not be parsed, either because there aren't enough columns, or because an invalid format was encountered: this was the event that caused an error: {:?}", line))
    }

    fn enable_section(&mut self, section: EDLSection) {
        match section {
            EDLSection::PluginsListing => { self.section_flags |= EDLPARSER_MASK_SECTION_PLUGINSLISTING },
            _ => {}, // TODO: Handle other sections
        }
    }

    fn disable_section(&mut self, section: EDLSection) {
        match section {
            EDLSection::PluginsListing => { self.section_flags ^= self.section_flags & EDLPARSER_MASK_SECTION_PLUGINSLISTING},
            _ => {}, // TODO: Handle other sections
        }
    }

    const fn get_section_size<const SECTION: usize>(&self) -> usize {
        if SECTION == EDLSection::TrackListing.as_usize() {
            if (self.section_flags & EDLPARSER_MASK_SECTION_PLUGINSLISTING) == EDLPARSER_MASK_SECTION_PLUGINSLISTING {
                return 5usize;
            }

            return 4usize;
        }

        0usize
    }

    fn is_valid_event_table_column_width(width: usize) -> bool {
        for valid_width in &EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS {
            if width == *valid_width { return true; }
        }

        false
    }

    fn select_fps_suffix(fps_string: &str) -> Option<&str> {
        // The order matters here, make sure to test for
        // longer suffixes first, otherwise it we get a
        // false-positive on "Frame"
        if fps_string.contains(" Drop Frame") {
            return Some(" Drop Frame")
        }

        else if fps_string.contains(" Frame") {
            return Some(" Frame")
        }

        None
    }
}

#[derive(Debug, Default)]
pub struct EDLSession {
    pub name: String,
    pub sample_rate: f32,
    pub bit_depth: u32,
    pub start_timecode: Timecode,
    pub fps: FrameRate,
    pub num_audio_tracks: u32,
    pub num_audio_clips: u32,
    pub num_audio_files: u32,
    pub files: EDLFileList,
    pub markers: Vec<EDLMarker>,
    pub tracks: Vec<EDLTrack>,
}

impl EDLSession {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            sample_rate: 0.0,
            bit_depth: 0,
            start_timecode: Timecode::with_fps(FrameRate::default()),
            fps: FrameRate::default(),
            num_audio_tracks: 0,
            num_audio_clips: 0,
            num_audio_files: 0,
            files: EDLFileList::default(),
            markers: Vec::<EDLMarker>::default(),
            tracks: Vec::<EDLTrack>::with_capacity(16),
        }
    }
}

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

#[derive(Debug, PartialEq, PartialOrd, Ord, Clone, Eq)]
pub struct EDLTrack {
    pub name: String,
    pub comment: String,
    pub delay: u32,
    pub state: (),
    pub events: Vec<EDLEvent>,
}

impl EDLTrack {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            comment: "".to_string(),
            delay: 0,
            state: (),
            events: Vec::<EDLEvent>::with_capacity(16),
        }
    }

    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Self::new()
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Clone, Eq)]
pub struct EDLEvent {
    pub channel: u32,
    pub event: u32,
    pub name: String,
    pub time_in: Timecode,
    pub time_out: Timecode,
    pub state: bool,
}

impl EDLEvent {
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

impl<'a> From<(&'a [&'a str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]], FrameRate)> for EDLEvent {
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

impl IntoIterator for EDLSession {
    type Item = EDLTrack;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.tracks.into_iter()
    }
}

impl IntoIterator for EDLTrack {
    type Item = EDLEvent;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.events.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}