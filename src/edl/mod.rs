// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

mod protools;

pub use protools::{
    EDLParser as EDLProtoolsParser,
    EDLSession as EDLProtoolsSession,
    EDLTrack as EDLProtoolsTrack,
    EDLTrackEvent as EDLProtoolsEvent,
};

pub mod encoding {
    pub use encoding_rs::*;
}
