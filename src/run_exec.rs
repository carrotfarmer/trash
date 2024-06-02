use std::path::PathBuf;
use std::process::Command;
use std::str::from_utf8;

pub fn run(exec_path: PathBuf, args: &[&str]) {
    let output = Command::new(exec_path)
        .args(args)
        .output()
        .expect("failed to execute process");

    let output_str = match from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("{}", output_str);
}
