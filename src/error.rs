use std::io;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError, Eq, PartialEq)]
pub enum FlywayNaimngCheckerError {
    #[error("cannot find any file in this path")]
    CannotFindAnyFile,

    #[error("no path provided")]
    NoPathProvideded,

    #[error("file io error")]
    FileIoError,

    #[error("Flyway file prefix naming error found in file : {file} - cannot find prefix")]
    FlywayNamingCanNotFindPrefix { file: String },

    #[error(
        "Flyway file prefix naming error found in file : {file} - expected: {expected}, found: {found}"
    )]
    FlywayNamingPrefixError {
        file: String,
        expected: String,
        found: String,
    },

    #[error("Flyway file version naming error found in file : {file} - cannot find version")]
    FlywayNamingCanNotFindVersion { file: String },

    #[error("Flyway file version naming error found in file : {file} - V or U must be followed by digits but prefix : {prefix} after found: {found}")]
    FlywayNamingVersionError {
        file: String,
        prefix: String,
        found: String,
    },

    #[error("Flyway file seperator naming error found in file : {file} - sepeartor between version and desciption is __ (2 underscore)")]
    FlywayNamingSeparatorError { file: String },

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
