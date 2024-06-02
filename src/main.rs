use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};

mod builtins;
mod path;

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
            _ => println!("{}: command not found", cmd),
        }
    }
}
