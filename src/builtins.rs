use std::{
    env::{self, set_current_dir},
    path::{Path, PathBuf},
    process::exit,
};

use crate::path::find_exec;

const BUILTINS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

pub fn echo(args: &[&str]) {
    let str_form: String = args.join(" ");
    println!("{}", str_form);
}

pub fn exit_fn(args: &[&str]) {
    if let Some(&single_char) = args.get(0) {
        // Convert the character to a String
        let char_string = single_char.to_string();

        // Parse the String to an integer
        match char_string.parse::<i32>() {
            Ok(exit_code) => exit(exit_code),
            Err(_) => println!("exit: could not parse exit code"),
        }
    } else {
        println!("exit: too many arguments");
    }
}

pub fn type_fn(args: &[&str], path: &str) {
    if let Some(&cmd_ref) = args.get(0) {
        for builtin in BUILTINS {
            if cmd_ref.eq(builtin) {
                println!("{} is a shell builtin", cmd_ref);
                return;
            }
        }

        match find_exec(path, cmd_ref) {
            Some(path_buf) => {
                let path_str = path_buf.to_string_lossy().to_string();
                println!("{} is {}", cmd_ref, path_str);
            }
            None => println!("{}: not found", cmd_ref),
        }
    } else {
        println!("exit: too many arguments");
    }
}

#[allow(deprecated, deprecated_in_future)]
pub fn cd(path: PathBuf) {
    let path_str = path.to_string_lossy().to_string();
    if path_str.contains("~") {
        let new_path = path_str.replace(
            "~",
            env::home_dir()
                .unwrap()
                .to_string_lossy()
                .to_string()
                .as_ref(),
        );

        let _ = set_current_dir(Path::new(&new_path));
        return;
    }

    if !path.exists() {
        println!(
            "{}: No such file or directory",
            path.to_string_lossy().to_string()
        );
        return;
    }

    let _ = set_current_dir(path);
}
