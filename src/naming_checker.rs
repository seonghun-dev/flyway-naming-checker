use crate::error::FlywayNaimngCheckerError;
use std::collections::HashSet;

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

                Err(FlywayNaimngCheckerError::FlywayNamingPrefixError {
                    file: file_name.to_string(),
                    expected: expected_prefix.to_string(),
                    found: prefix.to_string(),
                })
            } else {
                Ok(())
            }
        }
        None => Err(FlywayNaimngCheckerError::FlywayNamingCanNotFindPrefix {
            file: file_name.to_string(),
        }),
    }
}

pub fn is_valid_suffix(file_name: &str) -> Result<(), FlywayNaimngCheckerError> {
    if let Some(dot_index) = file_name.rfind('.') {
        let suffix = &file_name[dot_index + 1..];
        if suffix != "sql" {
            return Err(FlywayNaimngCheckerError::FlywayNamingSufixError {
                file: file_name.to_string(),
                expected: ".sql".to_owned(),
                found: suffix.to_owned(),
            });
        }
    }
    Ok(())
}

pub fn is_valid_sepeartor(file_name: &str) -> Result<(), FlywayNaimngCheckerError> {
    let parts: Vec<&str> = file_name.split("__").collect();

    if parts.len() != 2 {
        Err(FlywayNaimngCheckerError::FlywayNamingSeparatorError {
            file: file_name.to_string(),
        })
    } else {
        Ok(())
    }
}

pub fn is_valid_version(file_name: &str) -> Result<(), FlywayNaimngCheckerError> {
    match file_name.chars().next() {
        Some(first_char) => match first_char.to_owned() {
            'V' | 'U' => {
                let second_char = match file_name.chars().nth(1) {
                    Some(c) => c,
                    None => {
                        return Err(FlywayNaimngCheckerError::FlywayNamingCanNotFindVersion {
                            file: file_name.to_string(),
                        })
                    }
                };
                if second_char.is_ascii_digit() {
                    Ok(())
                } else {
                    Err(FlywayNaimngCheckerError::FlywayNamingVersionError {
                        file: file_name.to_string(),
                        prefix: first_char.to_string(),
                        found: second_char.to_string(),
                    })
                }
            }
            'R' => Ok(()),
            _ => Err(FlywayNaimngCheckerError::FlywayNamingPrefixError {
                file: file_name.to_string(),
                expected: "V".to_string(),
                found: first_char.to_string(),
            }),
        },
        None => Err(FlywayNaimngCheckerError::FlywayNamingCanNotFindPrefix {
            file: file_name.to_string(),
        }),
    }
}

pub fn check_for_duplicate_versions(file_names: &[String]) -> Result<(), FlywayNaimngCheckerError> {
    let mut versions = HashSet::new();

    for file_name in file_names {
        if let Some(version) = extract_version(file_name) {
            if !versions.insert(version) {
                return Err(FlywayNaimngCheckerError::DuplicateVersion {
                    file: file_name.to_string(),
                });
            }
        }
    }

    Ok(())
}

pub fn check_for_skipped_versions(file_names: &[String]) -> Result<(), FlywayNaimngCheckerError> {
    let mut versions: Vec<u32> = file_names
        .iter()
        .filter_map(|file_name| extract_version(file_name))
        .collect();

    if versions.is_empty() {
        return Ok(());
    }

    versions.sort_unstable();

    for window in versions.windows(2) {
        if window[1] != window[0] + 1 {
            return Err(FlywayNaimngCheckerError::SkippedVersion {
                file: format!("From V{} to V{}", window[0], window[1]),
            });
        }
    }

    Ok(())
}

fn extract_version(file_name: &str) -> Option<u32> {
    let parts: Vec<&str> = file_name.split("__").collect();
    if parts.len() != 2 {
        return None;
    }
    let version_part = parts[0].trim_start_matches(|c: char| !c.is_ascii_digit());
    version_part.parse::<u32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::FlywayNaimngCheckerError;

    #[test]
    fn test_valid_prefix() {
        assert_eq!(is_valid_prefix("V1__some_migration.sql"), Ok(()));
        assert_eq!(
            is_valid_prefix("X1__some_migration.sql"),
            Err(FlywayNaimngCheckerError::FlywayNamingPrefixError {
                file: "X1__some_migration.sql".to_string(),
                expected: "V".to_string(),
                found: "X".to_string(),
            })
        );
        assert_eq!(
            is_valid_prefix("v1__some_migration.sql"),
            Err(FlywayNaimngCheckerError::FlywayNamingPrefixError {
                file: "v1__some_migration.sql".to_string(),
                expected: "V".to_string(),
                found: "v".to_string(),
            })
        );
    }

    #[test]
    fn test_valid_suffix() {
        assert_eq!(is_valid_suffix("V1__some_migration.sql"), Ok(()));
        assert_eq!(
            is_valid_suffix("V1__some_migration.sqlx"),
            Err(FlywayNaimngCheckerError::FlywayNamingSufixError {
                file: "V1__some_migration.sqlx".to_string(),
                expected: ".sql".to_owned(),
                found: "sqlx".to_owned(),
            })
        );
    }

    #[test]
    fn test_valid_separator() {
        assert_eq!(is_valid_sepeartor("V1__some_migration.sql"), Ok(()));
        assert_eq!(
            is_valid_sepeartor("V1_some_migration.sql"),
            Err(FlywayNaimngCheckerError::FlywayNamingSeparatorError {
                file: "V1_some_migration.sql".to_string(),
            })
        );
    }

    #[test]
    fn test_valid_version() {
        assert_eq!(is_valid_version("V1__some_migration.sql"), Ok(()));
        assert_eq!(
            is_valid_version("X1__some_migration.sql"),
            Err(FlywayNaimngCheckerError::FlywayNamingPrefixError {
                file: "X1__some_migration.sql".to_string(),
                expected: "V".to_string(),
                found: "X".to_string(),
            })
        );
        assert_eq!(
            is_valid_version("Vb__some_migration.sql"),
            Err(FlywayNaimngCheckerError::FlywayNamingVersionError {
                file: "Vb__some_migration.sql".to_string(),
                prefix: "V".to_string(),
                found: "b".to_string(),
            })
        );
    }

    #[test]
    fn test_duplicate_versions() {
        let files = vec![
            "V1__init.sql".to_string(),
            "V2__add_table.sql".to_string(),
            "V1__duplicate.sql".to_string(),
        ];
        assert_eq!(
            check_for_duplicate_versions(&files),
            Err(FlywayNaimngCheckerError::DuplicateVersion {
                file: "V1__duplicate.sql".to_string(),
            })
        );
    }

    #[test]
    fn test_skipped_versions() {
        let files = vec![
            "V1__init.sql".to_string(),
            "V3__add_table.sql".to_string(),
        ];
        assert_eq!(
            check_for_skipped_versions(&files),
            Err(FlywayNaimngCheckerError::SkippedVersion {
                file: "From V1 to V3".to_string(),
            })
        );
    }
}
