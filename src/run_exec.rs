use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use anyhow::{anyhow, Result};

pub fn run(exec_path: PathBuf, args: Vec<String>, print_stdout: bool) -> Result<String> {
    let mut child: Child;
    let mut stdout_str: String = String::new();
    let mut stderr_str: Option<String> = None;

    if exec_path.to_str().unwrap().contains("./") {
        child = Command::new("sh")
            .arg("-c")
            .arg(exec_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute process");
    } else {
        child = Command::new(exec_path)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute process");
    }

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    if stderr.is_some() {
        let lines = BufReader::new(stderr.unwrap()).lines();
        for line in lines {
            eprintln!("{}", line.as_ref().unwrap());

            if stderr_str.is_none() {
                stderr_str = Some(line.unwrap());
            } else {
                stderr_str
                    .as_mut()
                    .unwrap()
                    .push_str(format!("{}\n", line.unwrap()).as_ref());
            }
        }
    }

    let lines = BufReader::new(stdout.unwrap()).lines();
    for line in lines {
        let l = line.unwrap();
        stdout_str.push_str(format!("{}\n", &l).as_ref());
        if print_stdout {
            println!("{}", &l);
        }
    }

    if stderr_str.is_some() {
        return Err(anyhow!("{}", stderr_str.unwrap()));
    }

    Ok(stdout_str)
}
