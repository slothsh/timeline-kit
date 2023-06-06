// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

mod protools;

pub use protools::{
    EDLParser as EDLProtoolsParser,
    EDLSession as EDLProtoolsSession,
};

pub mod encoding {
    pub use encoding_rs::*;
}
