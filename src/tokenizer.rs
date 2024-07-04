use std::fmt::Debug;

#[derive(Debug, PartialEq)]
struct Command {
    cmd_type: Option<String>,
    args: Vec<String>,
}

impl Command {
    fn new(cmd_type: Option<String>, args: Vec<String>) -> Self {
        Command { cmd_type, args }
    }
}

#[derive(Debug, PartialEq)]
struct OutputRedir {
    file: String,
}

impl OutputRedir {
    fn new(file: String) -> Self {
        OutputRedir { file }
    }
}

#[derive(Debug, PartialEq)]
enum Pipe {
    OutputRedir(OutputRedir),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    command: Option<Command>,
    pipe: Option<Pipe>,
}

impl Token {
    fn new(pipe: Option<Pipe>, command: Option<Command>) -> Self {
        Token { pipe, command }
    }
}

pub fn tokenizer(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut cmd_type: Option<String> = None;
    let mut args: Vec<String> = vec![];
    let mut pipe: Option<Pipe> = None;

    let input = &input.trim().to_string();
    let input = input.replace(">", " > ");

    let split = input.split_whitespace();

    for word in split {
        if word == ">" {
            let file = split.next().unwrap().to_string();
            pipe = Some(Pipe::OutputRedir(OutputRedir::new(file)));
        } else {
            if cmd_type.is_none() {
                cmd_type = Some(word.to_string());
            } else {
                args.push(word.to_string());
            }
        }
    }

    tokens.push(Token::new(pipe, Some(Command::new(cmd_type, args))));

    tokens
}
