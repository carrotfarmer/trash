use std::path::PathBuf;
use std::process::Command;
use std::str::from_utf8_unchecked;

pub fn run(exec_path: PathBuf, args: &[&str]) {
    let output = Command::new(exec_path)
        .args(args)
        .output()
        .expect("failed to execute process");

    unsafe {
        // please be utf8
        let output_str = from_utf8_unchecked(&output.stdout);
        println!("{}", output_str.trim());
    }
}
