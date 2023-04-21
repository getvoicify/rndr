use std::fmt;
use std::fmt::Formatter;
use std::error::Error;

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub enum RNDRError {
    AuthError,
    MissingParam(String),
    Duplicate,
    IOError(String),
    GenericError(String)
}

impl fmt::Display for RNDRError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RNDRError::AuthError => write!(f, "[RUST]: Error - An authentication error has occurred"),
            RNDRError::MissingParam(msg) => write!(f, "{}", msg),
            RNDRError::Duplicate => write!(f, "[RUST]: Error - A duplicate error"),
            RNDRError::IOError(msg) => write!(f, "{}", msg),
            RNDRError::GenericError(msg) => write!(f, "{}", msg)
        }
    }
}

impl Error for RNDRError {}

impl From<String> for RNDRError {
    fn from(s: String) -> Self {
        match s {
            _ => RNDRError::GenericError(s)
        }
    }
}