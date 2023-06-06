// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::{println, marker};

use encoding_rs_io::DecodeReaderBytesBuilder;

use crate::edl::protools::*;
use crate::chrono::Timecode;
use crate::chrono::FrameRate;


///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLParser` Declaration --
//
///////////////////////////////////////////////////////////////////////////

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

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLParser` Public Implementation --
//
///////////////////////////////////////////////////////////////////////////

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

                        // fill_header
                        if current_section == EDLSection::Header {
                            if let EDLValue::Field(_, field, value) = edl_parser.parse_edl_field(trimmed_line)? {
                                edl_parser.fill_session_header(&mut edl_session, &field, &value);
                            }
                        }

                        else if current_section == EDLSection::OnlineFiles {
                            let mut online_file_value_buffer: [&str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]] = [""; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]];
                            let data = edl_parser.parse_edl_online_file(trimmed_line, &mut online_file_value_buffer)?;
                            edl_parser.fill_online_files(&mut edl_session, &data);
                        }

                        else if current_section == EDLSection::OfflineFiles {
                            let mut offline_file_value_buffer: [&str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]] = [""; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]];
                            let data = edl_parser.parse_edl_offline_file(trimmed_line, &mut offline_file_value_buffer)?;
                            edl_parser.fill_offline_files(&mut edl_session, &data);
                        }

                        else if current_section == EDLSection::OnlineClips {
                            let mut online_clip_value_buffer: [&str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]] = [""; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]];
                            let data = edl_parser.parse_edl_online_clip(trimmed_line, &mut online_clip_value_buffer)?;
                            edl_parser.fill_online_clips(&mut edl_session, &data);
                        }

                        else if current_section == EDLSection::TrackListing {
                            if let EDLValue::Field(_, field, value) = edl_parser.parse_edl_field(trimmed_line)? {
                                edl_parser.fill_track_listing(&mut current_track, &field, &value);
                            }
                        }

                        else if current_section == EDLSection::TrackEvent {
                            let mut event_value_buffer: [&str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]] = [""; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]];
                            let data = edl_parser.parse_edl_track_event(trimmed_line, &mut event_value_buffer)?;
                            edl_parser.fill_track_events(&mut edl_session, &mut current_track, &data);
                        }

                        else if current_section == EDLSection::MarkersListing {
                            let mut marker_listing_value_buffer: [&str; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]] = [""; EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS[EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS.len() - 1]];
                            // TODO: Find better way to ensure that line is not trimmed before
                            // being parsed
                            let data = edl_parser.parse_edl_marker_listing(line.as_str(), &mut marker_listing_value_buffer)?;
                            edl_parser.fill_marker_listings(&mut edl_session, &data);
                        }
                    }
                }

                Err(error) => panic!("error: could not read line: {}", error),
            }

            edl_parser.file_position += 1;
        }

        Ok(edl_session)
    }

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLParser` Private Implementation --
//
///////////////////////////////////////////////////////////////////////////

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

    fn fill_session_header(&self, edl_session: &mut EDLSession, field: &EDLField, value: &str) {
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

    fn fill_online_files(&self, edl_session: &mut EDLSession, data: &EDLValue) {
        match data {
            EDLValue::TableHeader(_, _) => {
                // TODO: Maybe validate names of columns?
                // TODO: Set state for event table width?
            }

            EDLValue::TableEntry(_, cells) => {
                let online_file = EDLMediaFile::new(cells[0], cells[1]);
                edl_session.files.online_files.push(online_file);
            }

            _ => panic!("failed to parse EDLValue token, an unexpected or invalid token was encountered when parsing an EDL online file entry"),
        }
    }

    fn fill_offline_files(&self, edl_session: &mut EDLSession, data: &EDLValue) {
        match data {
            EDLValue::TableHeader(_, _) => {
                // TODO: Maybe validate names of columns?
                // TODO: Set state for event table width?
            }

            EDLValue::TableEntry(_, cells) => {
                let offline_file = EDLMediaFile::new(cells[0], cells[1]);
                edl_session.files.offline_files.push(offline_file);
            }

            _ => panic!("failed to parse EDLValue token, an unexpected or invalid token was encountered when parsing an EDL offline file entry"),
        }
    }

    fn fill_online_clips(&self, edl_session: &mut EDLSession, data: &EDLValue) {
        match data {
            EDLValue::TableHeader(_, _) => {
                // TODO: Maybe validate names of columns?
                // TODO: Set state for event table width?
            }

            EDLValue::TableEntry(_, cells) => {
                let online_clip = EDLClip::new(cells[0], cells[1]);
                edl_session.files.online_clips.push(online_clip);
            }

            _ => panic!("failed to parse EDLValue token, an unexpected or invalid token was encountered when parsing an EDL online clip entry"),
        }
    }

    fn fill_track_listing(&self, current_track: &mut Option<EDLTrack>, field: &EDLField, value: &str) {
        if let Some(track) = current_track {
            match field {
                EDLField::TrackName    => track.name    = value.to_string(),
                EDLField::TrackComment => track.comment = value.to_string(),
                EDLField::TrackDelay   => track.delay   = value.strip_suffix(" Samples").unwrap_or(value).parse::<u32>().expect("bit depth field must be valid integral number"),
                EDLField::TrackState   => track.state   = (),
                _ => eprintln!("parsing not implemented for field \"{:?}\" with value {}", field, value),
            }
        }

        else if *field == EDLField::TrackName {
            *current_track = Some(EDLTrack::with_name(value));
        }

        else {
            panic!("cannot create a new EDLTrack without specifying a name first: the header of the current track, \"{}\", is most likely in the incorrect order.", value);
        }
    }

    fn fill_track_events(&self, edl_session: &mut EDLSession, current_track: &mut Option<EDLTrack>, data: &EDLValue) {
        match data {
            EDLValue::TableHeader(_, _) => {
                // TODO: Maybe validate names of columns?
                // TODO: Set state for event table width?
            }

            EDLValue::TableEntry(_, cells) => {
                if let Some(track) = current_track {
                    let event = EDLTrackEvent::from((*cells, edl_session.fps));
                    track.events.push(event);
                } 

                else {
                    panic!("failed to parse EDLValue table event token because the current track was invalid");
                }
            }

            _ => panic!("failed to parse EDLValue token, an unexpected or invalid token was encountered when parsing an EDL track event table"),
        }
    }

    fn fill_marker_listings(&self, edl_session: &mut EDLSession, data: &EDLValue) {
        match data {
            EDLValue::TableHeader(_, _) => {
                // TODO: Maybe validate names of columns?
                // TODO: Set state for event table width?
            }

            EDLValue::TableEntry(_, cells) => {
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
