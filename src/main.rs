#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path::Path};

use crate::path::find_exec;

mod builtins;
mod path;
mod run_exec;
mod tokenizer;
mod utils;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let path_var = env::var("PATH").unwrap();

        // split input into commands by `&&` and `>`
        let tokens = tokenizer::tokenizer(input);
        println!("{:?}", tokens);

        let mut last_stdout = "".to_string();
        let mut last_status = 0;
    }
}

fn eval_stmt(cmd_type: Option<&str>, args: Vec<&str>, path_var: String, has_redir: bool) -> String {
    let stdout: String = match cmd_type.unwrap().to_string().as_str() {
        "echo" => builtins::echo(&args),
        "exit" => builtins::exit_fn(&args),
        "type" => builtins::type_fn(&args, &path_var),
        "cd" => {
            let path = Path::new(args.get(0).unwrap());
            builtins::cd(path.to_path_buf());
            return "".to_string();
        }
        "pwd" => builtins::pwd(),
        _ => {
            if cmd_type.unwrap().is_empty() {
                return "".to_string();
            }

            let exec = cmd_type.unwrap().to_string();
            let exec_path = find_exec(path_var.as_ref(), exec.as_ref());

            let mut cmd_args: Vec<&str> = vec![];

            for arg in args {
                if arg.is_empty() {
                    continue;
                }
                cmd_args.push(arg);
            }

            match exec_path {
                Some(ep) => {
                    let print_stdout = !has_redir;
                    let so = run_exec::run(ep, &cmd_args, print_stdout);
                    if print_stdout {
                        "".to_string()
                    } else {
                        so
                    }
                }
                None => {
                    eprintln!("{}: not found", exec);
                    "".to_string()
                }
            }
        }
    };

    stdout
}
