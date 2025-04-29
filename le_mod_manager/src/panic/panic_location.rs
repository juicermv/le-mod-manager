use core::fmt;
use std::fmt::Display;
use serde::{Deserialize, Serialize};
use serde::ser::Error;
use crate::panic::panic_info::PanicInfo;

#[derive(Default, Deserialize, Serialize,  Clone, Debug)]
pub enum PanicLocation {
    Known {
        line: u32,
        file: String,
    },
    
    #[default]
    Unknown,
}

impl Display for PanicLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => Err(fmt::Error::custom(e)),
        }
    }
}

impl From<String> for PanicLocation {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap_or_default()
    }
}