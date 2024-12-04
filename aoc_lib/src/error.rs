use core::num::ParseIntError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdventOfCodeError {
    #[error(transparent)]
    IoError(#[from] IoError),
    #[error(transparent)]
    IntegerParseError(#[from] ParseIntError),
    #[error("Failed to parse 2 location ids from a line in PairedLocationIdTsv Reader")]
    LocationPairParseError,
    #[error("Debug error print: {s}")]
    DebugPrint { s: String },
}
