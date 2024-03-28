use std::env;
use std::fs::write;
use std::process::exit;

fn main() {
    let github_output_path = env::var("GITHUB_OUTPUT").unwrap();

    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    if !path.is_empty() {
        write(github_output_path, format!("files={path}")).unwrap();
        exit(0);
    } else {
        exit(1);
    }
}
