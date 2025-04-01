// OK to ignore during early development, to be removed later
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![feature(coerce_unsized)]

pub mod colors;
pub mod effects;
pub mod fixtures;
pub mod parameters;
pub mod show;
pub mod utils;

use flexi_logger::{AdaptiveFormat, FileSpec, Logger, WriteMode};

pub fn setup_logger() {
    let _ = Logger::try_with_env_or_str("debug")
        .unwrap()
        .adaptive_format_for_stderr(AdaptiveFormat::WithThread)
        // .adaptive_format_for_stderr(AdaptiveFormat::Detailed)
        .set_palette("196;208;82;8;8".into())
        .start();
}
