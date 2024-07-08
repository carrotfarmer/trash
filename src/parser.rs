use std::path::Path;

use anyhow::Result;

use crate::{
    builtins,
    path::{find_exec, get_path_var},
    run_exec::run,
    tokenizer::{Operator, Token},
    utils::write_to_file,
};

pub fn has_redir(tokens: &Vec<Token>) -> bool {
    for token in tokens {
        match token {
            Token {
                command: None,
                operator: Some(Operator::OutputRedir(_)),
            } => return true,
            _ => {}
        }
    }

    false
}

pub fn eval_stmt(tokens: Vec<Token>) -> (String, bool) {
    let mut stdout = String::new();
    let path_var = get_path_var();

    let mut printed_stdout = false;

    let mut prev_res: Option<Result<String>> = None;

    for token in &tokens {
        match token {
            Token {
                command: Some(cmd),
                operator: None,
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

                        let print_stdout = !has_redir(&tokens);
                        if print_stdout && !printed_stdout {
                            printed_stdout = true;
                        }

                        match exec_path {
                            Some(ep) => {
                                let res = run(ep, cmd.args.clone(), print_stdout);

                                match res {
                                    Ok(s) => {
                                        prev_res = Some(Ok(s.clone()));
                                        stdout.push_str(s.as_str())
                                    }
                                    Err(e) => {
                                        prev_res = Some(Err(e));
                                    }
                                }
                            }
                            None => eprintln!("{}: not found", exec),
                        }
                    }
                }
            }
            Token {
                command: None,
                operator: Some(operator),
            } => match operator {
                Operator::OutputRedir(or) => match write_to_file(&or.file, &stdout) {
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", e),
                },
                Operator::LogicalAnd => {
                    if let Some(Err(_)) = prev_res {
                        break;
                    } else {
                        prev_res = None;
                        stdout.push_str("\n");
                        continue;
                    }
                }
            },
            _ => {}
        }
    }

    (stdout, printed_stdout)
}
