#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path::Path};

use tokenizer::Token;

use crate::{parser::eval_stmt, path::find_exec};

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

        // split input into commands by `&&` and `>`
        let tokens = tokenizer::tokenizer(input);
        println!("{}", parser::eval_stmt(tokens));
    }
}
