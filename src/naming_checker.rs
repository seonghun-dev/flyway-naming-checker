use crate::error::FlywayNaimngCheckerError;

pub fn is_valid_prefix(file_name: &str) -> Result<(), FlywayNaimngCheckerError> {
    let first_char = file_name.chars().next();

    match first_char {
        Some(prefix) => {
            if !(prefix == 'V' || prefix == 'U' || prefix == 'R') {
                let mut expected_prefix = "V";

                if prefix == 'u' {
                    expected_prefix = "u";
                }

                if prefix == 'r' {
                    expected_prefix = "r";
                }

                return Err(FlywayNaimngCheckerError::FlywayNamingPrefixError {
                    expected: expected_prefix.to_string(),
                    found: prefix.to_string(),
                });
            } else {
                return Ok(());
            }
        }
        None => {
            return Err(FlywayNaimngCheckerError::FlywayNamingPrefixError {
                expected: "V".to_string(),
                found: " ".to_string(),
            });
        }
    }
}

pub fn is_valid_suffix(file_name: &str) -> Result<(), FlywayNaimngCheckerError> {
    if file_name.ends_with(".sql") {
        return Err(FlywayNaimngCheckerError::FlywayNamingSufixError {
            expected: ".sql".to_owned(),
            found: ".test".to_owned(),
        });
    }
    return Ok(());
}
