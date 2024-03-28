use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum FlywayNaimngCheckerError {
    #[error("cannot find any file in this path")]
    CannotFindAnyFile,

    #[error("Error reading directory")]
    ErrorReadingDirectory,

    #[error("no path provided")]
    NoPathProvideded,

    #[error("file reading error")]
    FileReadError,

    #[error("naming error - expected: {expected}, found: {found}")]
    FlywayNamingPrefixError { expected: String, found: String },

    #[error("naming error - expected: {expected}, found: {found}")]
    FlywayNamingVersionError { expected: String, found: String },

    #[error("naming error - expected: {expected}, found: {found}")]
    FlywayNamingSeparatorError { expected: String, found: String },

    #[error("naming error - expected: {expected}, found: {found}")]
    FlywayNamingSufixError { expected: String, found: String },
}
