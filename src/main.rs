#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

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

        let cmd = input.trim();
        let cmd_vec: Vec<char> = cmd.chars().collect();
        let cmd_type = cmd_vec.get(0);
        let args = cmd_vec.get(1..).unwrap();

        match cmd_type.unwrap().to_string().as_str() {
            "echo" => {
                let str_form: String = args.iter().collect();
                println!("{}", str_form);
            }
            "exit" => {
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
            _ => println!("{}: command not found", cmd),
        }
    }
}
