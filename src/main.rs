use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let github_output_path = env::var("GITHUB_OUTPUT").unwrap();

    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    if !path.is_empty() {
        if let Ok(files) = get_file_list(path) {
            eprintln!("files: {:?}", files);

            if let Err(err) = write(&github_output_path, format!("files={:?}", files)) {
                eprintln!("Error writing output: {}", err);
                exit(1);
            } else {
                exit(0);
            }
        } else {
            eprintln!("Error reading directory");
            exit(1);
        }
    } else {
        eprintln!("No path provided");
        exit(1);
    }
}

fn get_file_list(path: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(file_name) = entry.file_name().to_str() {
            files.push(file_name.to_owned());
        }
    }
    Ok(files)
}

fn write(output_path: &str, content: String) -> io::Result<()> {
    let mut file = fs::File::create(output_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
