use std::error;
use std::fmt;
use std::num;

#[derive(Debug, Clone, PartialEq)]
pub enum InvalidColourFormat {
    ArgOutOfBoundsError,
    FailedConversionError,
    FormatNotRecognisedError,
    HexParseError(num::ParseIntError),
}

impl fmt::Display for InvalidColourFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid arguments provided to colour format constructor")
    }
}

impl error::Error for InvalidColourFormat {}

impl From<num::ParseIntError> for InvalidColourFormat {
    fn from(error: num::ParseIntError) -> Self {
        InvalidColourFormat::HexParseError(error)
    }
}
