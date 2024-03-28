mod error;
mod naming_checker;

use std::env;
use std::fs;
use std::io::{self};
use std::process::exit;

use error::FlywayNaimngCheckerError;
use naming_checker::is_valid_prefix;

fn main() {
    let github_output_path = env::var("GITHUB_OUTPUT").unwrap();

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
    let files = get_file_list(path).unwrap();
    if files.is_empty() {
        return Err(FlywayNaimngCheckerError::CannotFindAnyFile);
    }
    for file in files {
        let (is_valid, result_value) = is_valid_prefix(file.as_str());
        if !is_valid {
            return Err(FlywayNaimngCheckerError::FlywayNamingPrefixError {
                expected: result_value.expected,
                found: result_value.found,
            });
        }
    }
    return Ok(());
}

fn get_file_list(path: &str) -> Result<Vec<String>, io::Error> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(file_name) = entry.file_name().to_str() {
            files.push(file_name.to_owned());
        }
    }
    Ok(files)
}
