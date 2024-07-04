use std::path::Path;

use crate::{
    builtins,
    path::{find_exec, get_path_var},
    run_exec::run,
    tokenizer::{Pipe, Token},
    utils::write_to_file,
};

pub fn has_pipe(tokens: &Vec<Token>) -> bool {
    tokens.iter().any(|t| t.pipe.is_some())
}

pub fn eval_stmt(tokens: Vec<Token>) -> (String, bool) {
    let mut stdout = String::new();
    let path_var = get_path_var();

    let mut printed_stdout = false;

    for token in &tokens {
        match token {
            Token {
                command: Some(cmd),
                pipe: None,
            } => {
                let cmd_type = cmd.cmd_type.as_ref().unwrap();

                match cmd_type.to_string().as_str() {
                    "echo" => {
                        stdout.push_str(builtins::echo(cmd.args.clone()).as_str());
                    }
                    "exit" => {
                        stdout.push_str(builtins::exit_fn(cmd.args.clone()).as_str());
                    }
                    "type" => {
                        stdout.push_str(builtins::type_fn(cmd.args.clone(), &path_var).as_str());
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

                        let print_stdout = !has_pipe(&tokens);
                        if print_stdout && !printed_stdout {
                            printed_stdout = true;
                        }

                        match exec_path {
                            Some(ep) => {
                                stdout.push_str(run(ep, cmd.args.clone(), print_stdout).as_str());
                            }
                            None => eprintln!("{}: not found", exec),
                        }
                    }
                }
            }
            Token {
                command: None,
                pipe: Some(pipe),
            } => match pipe {
                Pipe::OutputRedir(or) => match write_to_file(&or.file, &stdout) {
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", e),
                },
            },
            _ => {}
        }
    }

    (stdout, printed_stdout)
}
