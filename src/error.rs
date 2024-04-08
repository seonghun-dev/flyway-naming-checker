use std::io;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum FlywayNaimngCheckerError {
    #[error("cannot find any file in this path")]
    CannotFindAnyFile,

    #[error("no path provided")]
    NoPathProvideded,

    #[error("file io error")]
    FileIoError,

    #[error("naming error - expected: {expected}, found: {found}")]
    FlywayNamingPrefixError { expected: String, found: String },

    #[error("naming error - expected: {expected}, found: {found}")]
    FlywayNamingVersionError { expected: String, found: String },

    #[error("naming error - expected: {expected}, found: {found}")]
    FlywayNamingSeparatorError { expected: String, found: String },

    #[error("naming error - expected: {expected}, found: {found}")]
    FlywayNamingSufixError { expected: String, found: String },
}

impl From<io::Error> for FlywayNaimngCheckerError {
    fn from(_error: io::Error) -> Self {
        FlywayNaimngCheckerError::FileIoError
    }
}
