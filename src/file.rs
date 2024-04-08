use std::fs::read_dir;

use crate::error::FlywayNaimngCheckerError;

pub fn get_file_names(path: &str) -> Result<Vec<String>, FlywayNaimngCheckerError> {
    let mut files = Vec::new();
    for entry in read_dir(path)? {
        let entry = entry?;
        if let Some(file_name) = entry.file_name().to_str() {
            files.push(file_name.to_owned());
        }
    }
    Ok(files)
}
