pub struct ResultValue {
    pub expected: String,
    pub found: String,
}

pub fn is_valid_prefix(file_name: &str) -> (bool, ResultValue) {
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

                return (
                    false,
                    ResultValue {
                        found: prefix.to_string(),
                        expected: expected_prefix.to_string(),
                    },
                );
            } else {
                return (
                    true,
                    ResultValue {
                        found: "V".to_string(),
                        expected: 'V'.to_string(),
                    },
                );
            }
        }
        None => (
            false,
            ResultValue {
                found: " ".to_string(),
                expected: 'V'.to_string(),
            },
        ),
    }
}

pub fn is_valid_suffix(file_name: &str) -> bool {
    file_name.ends_with(".sql")
}
