use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

pub fn run(exec_path: PathBuf, args: &[&str], print_stdout: bool) -> String {
    let mut child: Child;
    let mut stdout_str: String = String::new();

    if exec_path.to_str().unwrap().contains("./") {
        child = Command::new("sh")
            .arg("-c")
            .arg(exec_path)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");
    } else {
        child = Command::new(exec_path)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");
    }

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take();

    if stderr.is_some() {
        let lines = BufReader::new(stderr.unwrap()).lines();
        for line in lines {
            eprintln!("{}", line.unwrap());
        }
    }

    let lines = BufReader::new(stdout).lines();
    for line in lines {
        let l = line.unwrap();
        stdout_str.push_str(format!("{}\n", &l).as_ref());
        if print_stdout {
            println!("{}", &l);
        }
    }

    return stdout_str;
}
