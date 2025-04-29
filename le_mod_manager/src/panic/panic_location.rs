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