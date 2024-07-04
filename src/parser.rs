use std::path::Path;

use crate::{
    builtins,
    path::{find_exec, get_path_var},
    run_exec::run,
    tokenizer::Token,
};

pub fn eval_stmt(tokens: Vec<Token>) -> String {
    let mut stdout = String::new();
    let path_var = get_path_var();

    let has_pipe = tokens.iter().any(|t| t.pipe.is_some());

    for token in tokens {
        match token {
            Token {
                command: Some(cmd),
                pipe: None,
            } => {
                let cmd_type = cmd.cmd_type.as_ref().unwrap();

                match cmd_type.to_string().as_str() {
                    "echo" => {
                        stdout.push_str(builtins::echo(cmd.args).as_str());
                    }
                    "exit" => {
                        stdout.push_str(builtins::exit_fn(cmd.args).as_str());
                    }
                    "type" => {
                        stdout.push_str(builtins::type_fn(cmd.args, &path_var).as_str());
                    }
                    "cd" => {
                        let path = Path::new(cmd.args.get(0).unwrap());
                        builtins::cd(path.to_path_buf());
                    }
                    "pwd" => {
                        stdout.push_str(builtins::pwd().as_str());
                    }
                    _ => {
                        let exec = cmd_type.to_string();
                        let exec_path = find_exec(path_var.as_ref(), exec.as_ref());

                        let print_stdout: bool = has_pipe;

                        match exec_path {
                            Some(ep) => {
                                stdout.push_str(run(ep, cmd.args, print_stdout).as_str());
                            }
                            None => eprintln!("{}: not found", exec),
                        }
                    }
                }
            }
            Token {
                command: None,
                pipe: Some(pipe),
            } => {}
            _ => {}
        }
    }

    stdout
}
