mod error;
mod file;
mod naming_checker;

use std::env;
use std::process::exit;

use error::FlywayNaimngCheckerError;
use file::get_file_names;
use naming_checker::{is_valid_prefix, is_valid_sepeartor, is_valid_suffix, is_valid_version};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    if let Err(err) = run(path) {
        eprintln!("Error: {}", err);
        exit(1);
    } else {
        exit(0);
    }
}

fn run(path: &str) -> Result<(), FlywayNaimngCheckerError> {
    if path.is_empty() {
        return Err(FlywayNaimngCheckerError::NoPathProvideded);
    }
    let files = get_file_names(path)?;
    if files.is_empty() {
        return Err(FlywayNaimngCheckerError::CannotFindAnyFile);
    }
    for file in files {
        is_valid_prefix(file.as_str())?;
        is_valid_suffix(file.as_str())?;
        is_valid_sepeartor(file.as_str())?;
        is_valid_version(file.as_str())?;
    }
    Ok(())
}
