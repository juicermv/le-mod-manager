use crate::panic::panic_location::PanicLocation;
use core::fmt;
use std::backtrace::Backtrace;
use serde::ser::Error;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Default, Deserialize, Serialize, Clone, Debug)]
pub struct PanicInfo {
    payload: String,
    location: PanicLocation,

    backtrace: String,
}

impl PanicInfo {
    pub fn from_hook(info: &std::panic::PanicHookInfo) -> Self {
        Self {
            payload: if let Some(s) = info.payload().downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = info.payload().downcast_ref::<String>() {
                s.clone()
            } else {
                "".into()
            },

            location: match info.location() {
                Some(location) => PanicLocation::Known {
                    line: location.line(),
                    file: location.file().into(),
                },

                None => PanicLocation::Unknown,
            },

            backtrace: "".into(),
        }
    }
    
    pub fn set_backtrace(&mut self, backtrace: Backtrace) {
        self.backtrace = backtrace.to_string();
    }
}

impl Display for PanicInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string_pretty(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => Err(fmt::Error::custom(e)),
        }
    }
}

impl From<String> for PanicInfo {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap_or_default()
    }
}
