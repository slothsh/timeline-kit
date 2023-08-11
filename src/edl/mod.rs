// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

#![allow(unused_braces)]

mod protools;

pub use protools::{
    EDLParser as EDLProtoolsParser,
    EDLSession as EDLProtoolsSession,
    ParseField as EDLParseField,
};

pub mod encoding {
    pub use encoding_rs::*;
}
