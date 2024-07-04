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

    let mut curr_cmd_type = None;
    let mut curr_args: Vec<String> = vec![];

    let mut split = input.split_whitespace();

    while let Some(word) = split.next() {
        if word == ">" {
            if !curr_cmd_type.is_none() {
                tokens.push(Token::new(
                    None,
                    Some(Command::new(curr_cmd_type.clone(), curr_args.clone())),
                ));
            }

            curr_cmd_type = None;
            curr_args = vec![];

            let file = split.next().unwrap().to_string();
            tokens.push(Token::new(
                Some(Pipe::OutputRedir(OutputRedir::new(file))),
                None,
            ));
        } else {
            if curr_cmd_type.is_none() {
                curr_cmd_type = Some(word.to_string());
            } else {
                curr_args.push(word.to_string());
            }
        }
    }

    if !curr_cmd_type.is_none() {
        tokens.push(Token::new(
            None,
            Some(Command::new(curr_cmd_type.clone(), curr_args.clone())),
        ));
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_redir() {
        let input = "ls > test.txt";
        let tokens = tokenizer(input.to_string());
        println!("\n\n{:?}\n\n", tokens);
        let expected = vec![
            Token::new(None, Some(Command::new(Some("ls".to_string()), vec![]))),
            Token::new(
                Some(Pipe::OutputRedir(OutputRedir::new("test.txt".to_string()))),
                None,
            ),
        ];
        assert_eq!(tokens, expected);

        let input = "ls -l > test.txt";
        let tokens = tokenizer(input.to_string());
        println!("\n\n{:?}\n\n", tokens);
        let expected = vec![
            Token::new(
                None,
                Some(Command::new(Some("ls".to_string()), vec!["-l".to_string()])),
            ),
            Token::new(
                Some(Pipe::OutputRedir(OutputRedir::new("test.txt".to_string()))),
                None,
            ),
        ];
        assert_eq!(tokens, expected);

        let input = "cmd -s -l --help > test.txt";
        let tokens = tokenizer(input.to_string());
        println!("\n\n{:?}\n\n", tokens);
        let expected = vec![
            Token::new(
                None,
                Some(Command::new(
                    Some("cmd".to_string()),
                    vec!["-s".to_string(), "-l".to_string(), "--help".to_string()],
                )),
            ),
            Token::new(
                Some(Pipe::OutputRedir(OutputRedir::new("test.txt".to_string()))),
                None,
            ),
        ];
        assert_eq!(tokens, expected);
    }
}
