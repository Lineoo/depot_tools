#![warn(clippy::allow_attributes)]
//! `depot_core` is the main supporting lib for depot tools.

pub mod entry;
pub mod entryspace;
pub mod stack;
pub mod implement;
mod search;
pub mod dylib;

/// ## Expected Functions ##
/// - `EntrySpace`: use input to search, cached
/// - `Enum`: choose variants
/// - `Calculator`: parse input to result
/// - `Files`: result an *infinite* number of entries
/// - `FFmpeg Util`: need to choose multiple files/parameters
/// - `Color Picker`: completely control the UI pass
/// ## Dependencies ##
/// Try `Tantivy` or hand-written `memchr` as searching backend
mod plan {}