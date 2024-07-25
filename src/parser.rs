use std::path::Path;

use anyhow::Result;

use crate::{
    builtins,
    path::{find_exec, get_path_var},
    run_exec::run,
    tokenizer::{Command, Operator, Token},
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
    let mut is_prev_pipe = false;

    for token in &tokens {
        match token {
            Token {
                command: Some(cmd),
                operator: None,
            } => {
                let cmd_type = cmd.cmd_type.as_ref().unwrap();
                let mut cmd_args = cmd.args.clone();

                if is_prev_pipe {
                    let formatted_stdout =
                        format!("{}", prev_res.as_ref().unwrap().as_ref().unwrap());
                    let formatted_stdout =
                        formatted_stdout.replace("\\n", "\n").replace("\\t", "\t");

                    cmd_args.insert(0, formatted_stdout);
                    println!("{}", cmd_args.join(" "));
                    is_prev_pipe = false;
                }

                match cmd_type.to_string().as_str() {
                    "echo" => {
                        stdout.push_str(builtins::echo(cmd_args.clone()).as_str());
                    }
                    "exit" => {
                        stdout.push_str(builtins::exit_fn(cmd_args.clone()).as_str());
                    }
                    "type" => {
                        stdout.push_str(builtins::type_fn(cmd_args.clone(), &path_var).as_str());
                    }
                    "cd" => {
                        let path = Path::new(cmd_args.get(0).unwrap());
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
                                let res = run(ep, cmd_args.clone(), print_stdout);

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
                Operator::Pipe => {
                    if let Some(Err(_)) = prev_res {
                        break;
                    } else {
                        is_prev_pipe = true;
                        continue;
                    }
                }
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
