#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path::Path};

use crate::path::find_exec;

mod builtins;
mod path;
mod run_exec;
mod utils;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let path_var = env::var("PATH").unwrap();

        let cmd = input.trim();
        let has_redir = cmd.contains(">");

        let cmd_vec: Vec<&str>;
        let cmd_type: Option<&str>;
        let args: Vec<&str>;

        let output_file: &str;

        if has_redir {
            cmd_vec = cmd.trim().split(">").collect();
            let mut cmd_split = cmd_vec.get(0).expect("err: no command provided").split(" ");

            cmd_type = cmd_split.next();

            args = cmd_split.collect();

            output_file = cmd_vec.last().expect("err: no file provided");
        } else {
            cmd_vec = cmd.split(" ").collect();
            cmd_type = cmd_vec.get(0).copied();
            args = cmd_vec.get(1..).unwrap().to_vec();
            output_file = "";
        }

        let stdout: String = match cmd_type.unwrap().to_string().as_str() {
            "echo" => builtins::echo(&args),
            "exit" => builtins::exit_fn(&args),
            "type" => builtins::type_fn(&args, &path_var),
            "cd" => {
                let path = Path::new(args.get(0).unwrap());
                builtins::cd(path.to_path_buf());
                continue;
            }
            "pwd" => builtins::pwd(),
            _ => {
                let exec = cmd_type.unwrap().to_string();
                let exec_path = find_exec(path_var.as_ref(), exec.as_ref());

                let cmd_args: Vec<&str>;

                if args.len() == 1 && args.get(0).unwrap().is_empty() {
                    cmd_args = vec![];
                } else {
                    cmd_args = args.clone();
                }

                match exec_path {
                    Some(ep) => run_exec::run(ep, &cmd_args),
                    None => format!("{}: not found", exec),
                }
            }
        };

        if has_redir {
            if output_file.is_empty() {
                println!("err: no file provided");
                continue;
            }

            utils::write_to_file(output_file, &stdout).unwrap();
        } else {
            println!("{}", stdout);
        }
    }
}
