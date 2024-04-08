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

    #[error(
        "Flyway file prefix naming error found in file : {file} - expected: {expected}, found: {found}"
    )]
    FlywayNamingPrefixError {
        file: String,
        expected: String,
        found: String,
    },

    #[error("Flyway file version naming error found in file : {file} - expected: {expected}, found: {found}")]
    FlywayNamingVersionError {
        file: String,
        expected: String,
        found: String,
    },

    #[error("Flyway file seperator naming error found in file : {file} - expected: {expected}, found: {found}")]
    FlywayNamingSeparatorError {
        file: String,
        expected: String,
        found: String,
    },

    #[error(
        "Flyway file suffix naming error found in file : {file} - expected: {expected}, found: {found} "
    )]
    FlywayNamingSufixError {
        file: String,
        expected: String,
        found: String,
    },
}

impl From<io::Error> for FlywayNaimngCheckerError {
    fn from(_error: io::Error) -> Self {
        FlywayNaimngCheckerError::FileIoError
    }
}
