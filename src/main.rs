use std::io::{self, Write};

mod builtins;
mod parser;
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

        let tokens = tokenizer::tokenizer(input);
        let (stdout, printed_stdout) = parser::eval_stmt(tokens.clone());
        let has_pipe = parser::has_pipe(&tokens);

        if !has_pipe && !printed_stdout {
            println!("{}", stdout);
        }
    }
}
