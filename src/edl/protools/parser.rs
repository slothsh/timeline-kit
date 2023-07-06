// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

use std::borrow::Borrow;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;
use std::{println, marker};

use encoding_rs_io::DecodeReaderBytesBuilder;

use crate::edl::protools::*;
use crate::chrono::{
    BitDepth,
    FrameRate,
    Timecode, SampleRate
};

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLParser` Declaration --
//
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct EDLParser<'a> {
    file_path: &'a str,
    file_position: usize,
    section_position: usize,
    current_section: EDLSection,
    flags: u8,
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLParser` Public Implementation --
//
///////////////////////////////////////////////////////////////////////////

impl<'a> EDLParser<'a> {
    pub fn parse(input_path: &'a str, encoding: &'static encoding_rs::Encoding) -> Result<EDLSession, String> {
        let mut edl_parser = EDLParser {
            file_path: input_path,
            current_section: EDLSection::Header,
            ..EDLParser::default()
        };

        let input_file = File::open(input_path).unwrap();
        let input_file_decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(encoding))
            .build(input_file);
        let input_file_handle = BufReader::new(input_file_decoder);
        let mut all_lines = input_file_handle.lines();

        let mut raw_header_lines = Vec::<(usize, String)>::with_capacity(EDL_HEADER_LINE_SIZE as usize);
        let mut raw_tracks_listings_lines = Vec::<(usize, String)>::new();
        let mut raw_markers_listings_lines = Vec::<(usize, String)>::new();
        let mut raw_plugins_listings_lines = Vec::<(usize, String)>::new();
        let mut raw_offline_files_lines = Vec::<(usize, String)>::new();
        let mut raw_online_files_lines = Vec::<(usize, String)>::new();
        let mut raw_online_clips_lines = Vec::<(usize, String)>::new();

        let mut edl_session = EDLSession::new();

        while let Some(line_result) = all_lines.next() {
            let line = line_result.expect("line in EDL file handle should be parseable");
            let trimmed_line = line.as_str().trim();
            let mut skip = line.trim() == "";
            edl_parser.file_position += 1;

            use EDLSection::*;
            if edl_parser.is_section_declaration(trimmed_line) {
                edl_parser.current_section =
                    if trimmed_line == PluginsListing.section_name() { skip = true; PluginsListing }
                    else if trimmed_line == TrackListing.section_name() { skip = true; TrackListing }
                    else if trimmed_line == MarkersListing.section_name() { skip = true; MarkersListing }
                    else if trimmed_line == OfflineFiles.section_name() { skip = true; OfflineFiles }
                    else if trimmed_line == OnlineFiles.section_name() { skip = true; OnlineFiles }
                    else if trimmed_line == OnlineClips.section_name() { skip = true; OnlineClips }
                    else { Unknown };
            }

            if skip { continue; }

            match edl_parser.current_section {
                Header => {
                    raw_header_lines.push((edl_parser.file_position, trimmed_line.to_string()));
                },

                PluginsListing => {
                    // TODO: Set EDLParser flags for plugins listing
                    raw_plugins_listings_lines.push((edl_parser.file_position, trimmed_line.to_string()));
                },

                OnlineFiles => {
                    raw_online_files_lines.push((edl_parser.file_position, trimmed_line.to_string()));
                },

                OfflineFiles => {
                    raw_offline_files_lines.push((edl_parser.file_position, trimmed_line.to_string()));
                },

                OnlineClips => {
                    raw_online_clips_lines.push((edl_parser.file_position, trimmed_line.to_string()));
                },

                TrackListing => {
                    raw_tracks_listings_lines.push((edl_parser.file_position, trimmed_line.to_string()));
                },

                MarkersListing => {
                    raw_markers_listings_lines.push((edl_parser.file_position, trimmed_line.to_string()));
                },

                Unknown => { /* TODO: Report? */ }
            }
        }

        if let Some(_) = edl_parser.parse_header(&mut raw_header_lines, &mut edl_session) {
        }

        if let Some(_) = edl_parser.parse_plugins_listing(&mut raw_plugins_listings_lines, &mut edl_session) {
        }

        // if let Some(_) = edl_parser.parse_offline_files_listing(&mut raw_offline_files_lines, &mut edl_session) {
        // }
        //
        // if let Some(_) = edl_parser.parse_online_files_listing(&mut raw_online_files_lines, &mut edl_session) {
        // }
        //
        // if let Some(_) = edl_parser.parse_online_clips_listing(&mut raw_online_clips_lines, &mut edl_session) {
        // }
        //
        // if let Some(_) = edl_parser.parse_tracks_listing(&mut raw_tracks_listings_lines, &mut edl_session) {
        // }
        //
        // if let Some(_) = edl_parser.parse_markers_listing(&mut raw_markers_listings_lines, &mut edl_session) {
        // }

        Ok(edl_session)
    }
    
    // TODO: Proper error for this function
    fn parse_edl_field<'z>(&self, field_string: &'z str) -> Result<EDLValue<'z>, String> {
        let field_parts = field_string.split(":\t").into_iter().collect::<Vec<&str>>();
        if field_parts.len() == 2 {
            for field_variant in EDLField::all_variants() {
                if field_variant.field_name() == field_parts[EDL_FIELD_NAME_INDEX] {
                    return Ok(EDLValue::Field(*field_variant, field_parts[EDL_FIELD_VALUE_INDEX]));
                }
            }
        }
        Err("".to_string())
    }

    // TODO: Proper errors for parse_* functions
    fn parse_header(&self, raw_header_lines: &mut Vec<(usize, String)>, edl_session: &mut EDLSession) -> Option<()> {
        for field in raw_header_lines {
            if let Ok(EDLValue::Field(field_name, field_value)) = self.parse_edl_field(field.1.as_str()) {
                if field_name == EDLField::SessionName { edl_session.name = field_value.to_string(); }
                else if field_name == EDLField::SessionSampleRate { edl_session.sample_rate = SampleRate::parse_field(field_value).expect("EDL header sample rate field should have a valid floating point value") }
                else if field_name == EDLField::SessionBitDepth { edl_session.bit_depth = BitDepth::parse_field(field_value).expect("EDL header bit depth field should have a valid bit depth option value") }
                else if field_name == EDLField::SessionStartTimecode { edl_session.start_timecode = Timecode::from_str(field_value, edl_session.fps).expect("EDL header start timecode field should have a valid timecode string"); }
                else if field_name == EDLField::SessionTimecodeFormat {
                    let fps = FrameRate::parse_field(field_value).expect("EDL header timecode format field should have a valid fps string");
                    edl_session.start_timecode.set_frame_rate(fps);
                    edl_session.fps = fps;
                }
                else if field_name == EDLField::SessionNumAudioTracks { edl_session.num_audio_tracks = field_value.parse::<u32>().expect("EDL header number audio tracks field should have a valid integer number value"); }
                else if field_name == EDLField::SessionNumAudioClips { edl_session.num_audio_clips = field_value.parse::<u32>().expect("EDL header number audio clips field should have a valid integer number value"); }
                else if field_name == EDLField::SessionNumAudioFiles { edl_session.num_audio_files = field_value.parse::<u32>().expect("EDL header number audio files field should have a valid integer number value"); }
                else { panic!("unexpected field name in EDL header section"); }
            } else {
                return Some(())
            }
        }

        None
    }

    fn parse_plugins_listing(&self, raw_plugins_listings_lines: &mut Vec<(usize, String)>, edl_session: &mut EDLSession) -> Option<()> {
        None
    }

    fn parse_tracks_listing(&self, raw_tracks_listings_lines: &mut Vec<(usize, String)>, edl_session: &mut EDLSession) -> Option<()> {
        todo!()
    }

    fn parse_markers_listing(&self, raw_markers_listings_lines: &mut Vec<(usize, String)>, edl_session: &mut EDLSession) -> Option<()> {
        todo!()
    }

    fn parse_online_files_listing(&self, raw_online_files_lines: &mut Vec<(usize, String)>, edl_session: &mut EDLSession) -> Option<()> {
        todo!()
    }

    fn parse_offline_files_listing(&self, raw_offline_files_lines: &mut Vec<(usize, String)>, edl_session: &mut EDLSession) -> Option<()> {
        todo!()
    }

    fn parse_online_clips_listing(&self, raw_online_clips_lines: &mut Vec<(usize, String)>, edl_session: &mut EDLSession) -> Option<()> {
        todo!()
    }

    fn is_section_declaration(&self, section_string: &str) -> bool {
        let all_parts = section_string.split(' ');
        for part in all_parts {
            if part.len() != 1 { return false; }
        }
        if section_string.trim() == "" { return false; }
        true
    }
}
