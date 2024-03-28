pub fn is_valid_prefix(file_name: &str) -> bool {
    // 파일명의 첫 번째 문자를 가져옴
    let first_char = file_name.chars().next();

    // 첫 번째 문자가 없거나, Prefix가 아니면 유효하지 않은 파일명
    match first_char {
        Some(prefix) => {
            if !(prefix == 'V' || prefix == 'U' || prefix == 'R') {
                return false;
            }
        }
        None => return false,
    }
    true
}

pub fn is_valid_suffix(file_name: &str) -> bool {
    file_name.ends_with(".sql")
}
