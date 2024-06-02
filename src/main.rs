#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path::Path};

use crate::path::find_exec;

mod builtins;
mod path;
mod run_exec;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let path_var = env::var("PATH").unwrap();

        let cmd = input.trim();
        let cmd_vec: Vec<&str> = cmd.split(" ").collect();

        let cmd_type = cmd_vec.get(0);
        let args = cmd_vec.get(1..).unwrap();

        match cmd_type.unwrap().to_string().as_str() {
            "echo" => builtins::echo(args),
            "exit" => builtins::exit_fn(args),
            "type" => builtins::type_fn(args, &path_var),
            "cd" => {
                let path = Path::new(args.get(0).unwrap());
                builtins::cd(path.to_path_buf())
            }
            _ => {
                let exec = cmd_type.unwrap().to_string();
                let exec_path = find_exec(path_var.as_ref(), exec.as_ref());

                match exec_path {
                    Some(ep) => run_exec::run(ep, args),
                    None => println!("{}: not found", exec),
                }
            }
        }
    }
}
