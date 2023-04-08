#![doc = include_str!("../README.md")]

pub mod fs;
mod osstr;
mod path;

pub use self::osstr::OsStrAnyhow;
pub use self::path::PathAnyhow;
