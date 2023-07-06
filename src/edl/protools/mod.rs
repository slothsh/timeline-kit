// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

#![allow(dead_code, unused_imports)]

mod parser;
mod parser_types;
mod parser_traits;
mod session;
mod session_types;

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLParser` Module Interface --
//
///////////////////////////////////////////////////////////////////////////

pub use parser::EDLParser;

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLParser` Internal Types Module Interface --
//
///////////////////////////////////////////////////////////////////////////

use parser_types::{
    EDLSection,
    EDLField,
    EDLValue,
    EDLTrackEventColumn,
};

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLParser` Internal Types Module Interface --
//
///////////////////////////////////////////////////////////////////////////

use parser_types::{
    EDL_HEADER_LINE_SIZE,
    EDL_TRACK_LISTING_LINE_SIZE,
    EDL_SECTION_TERMINATOR_LENGTH,
    EDL_FIELD_PARTS_LENGTH,
    EDL_FIELD_NAME_INDEX,
    EDL_FIELD_VALUE_INDEX,
    EDL_TRACK_EVENT_VALID_COLUMN_WIDTHS,
    EDLPARSER_MASK_SECTION_PLUGINSLISTING,
};

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLParser` Traits Interface --
//
///////////////////////////////////////////////////////////////////////////

pub use parser_traits::ParseField;

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLSession` Module Interface --
//
///////////////////////////////////////////////////////////////////////////

pub use session::EDLSession;

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLSession` Internal Types Module Interface --
//
///////////////////////////////////////////////////////////////////////////

pub use session_types::{
    EDLClip,
    EDLFileList,
    EDLMarker,
    EDLMediaFile,
    EDLPlugin,
    EDLPluginFormat,
    EDLPluginInstance,
    EDLTrack,
    EDLTrackEvent,
    EDLUnit,
};
