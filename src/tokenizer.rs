use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    pub cmd_type: Option<String>,
    pub args: Vec<String>,
}

impl Command {
    fn new(cmd_type: Option<String>, args: Vec<String>) -> Self {
        Command { cmd_type, args }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OutputRedir {
    pub file: String,
}

impl OutputRedir {
    fn new(file: String) -> Self {
        OutputRedir { file }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    OutputRedir(OutputRedir),
    LogicalAnd,
    Pipe,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub command: Option<Command>,
    pub operator: Option<Operator>,
}

impl Token {
    fn new(operator: Option<Operator>, command: Option<Command>) -> Self {
        Token { operator, command }
    }
}

pub fn tokenizer(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut curr_cmd_type = None;
    let mut curr_args: Vec<String> = vec![];

    let input = input.trim();
    let input = input.replace("&&", " && ");
    let input = input.replace(">", " > ");
    let input = input.replace("|", " | ");
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
                Some(Operator::OutputRedir(OutputRedir::new(file))),
                None,
            ));
        } else if word == "&&" {
            if !curr_cmd_type.is_none() {
                tokens.push(Token::new(
                    None,
                    Some(Command::new(curr_cmd_type.clone(), curr_args.clone())),
                ));
            }

            tokens.push(Token::new(Some(Operator::LogicalAnd), None));

            curr_cmd_type = None;
            curr_args = vec![];
        } else if word == "|" {
            if !curr_cmd_type.is_none() {
                tokens.push(Token::new(
                    None,
                    Some(Command::new(curr_cmd_type.clone(), curr_args.clone())),
                ));
            }

            tokens.push(Token::new(Some(Operator::Pipe), None));

            curr_cmd_type = None;
            curr_args = vec![];
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
        let expected = vec![
            Token::new(None, Some(Command::new(Some("ls".to_string()), vec![]))),
            Token::new(
                Some(Operator::OutputRedir(OutputRedir::new(
                    "test.txt".to_string(),
                ))),
                None,
            ),
        ];
        assert_eq!(tokens, expected);

        let input = "ls -l > test.txt";
        let tokens = tokenizer(input.to_string());
        let expected = vec![
            Token::new(
                None,
                Some(Command::new(Some("ls".to_string()), vec!["-l".to_string()])),
            ),
            Token::new(
                Some(Operator::OutputRedir(OutputRedir::new(
                    "test.txt".to_string(),
                ))),
                None,
            ),
        ];
        assert_eq!(tokens, expected);

        let input = "cmd -s -l --help > test.txt";
        let tokens = tokenizer(input.to_string());
        let expected = vec![
            Token::new(
                None,
                Some(Command::new(
                    Some("cmd".to_string()),
                    vec!["-s".to_string(), "-l".to_string(), "--help".to_string()],
                )),
            ),
            Token::new(
                Some(Operator::OutputRedir(OutputRedir::new(
                    "test.txt".to_string(),
                ))),
                None,
            ),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn logical_and() {
        let input = "ls && echo hello";
        let tokens = tokenizer(input.to_string());

        let expected = vec![
            Token::new(None, Some(Command::new(Some("ls".to_string()), vec![]))),
            Token::new(Some(Operator::LogicalAnd), None),
            Token::new(
                None,
                Some(Command::new(
                    Some("echo".to_string()),
                    vec!["hello".to_string()],
                )),
            ),
        ];
        assert_eq!(tokens, expected);

        let input = "ls -l && echo hello";
        let tokens = tokenizer(input.to_string());
        let expected = vec![
            Token::new(
                None,
                Some(Command::new(Some("ls".to_string()), vec!["-l".to_string()])),
            ),
            Token::new(Some(Operator::LogicalAnd), None),
            Token::new(
                None,
                Some(Command::new(
                    Some("echo".to_string()),
                    vec!["hello".to_string()],
                )),
            ),
        ];
        assert_eq!(tokens, expected);

        let input = "cmd -s -l --help && echo hello";
        let tokens = tokenizer(input.to_string());
        let expected = vec![
            Token::new(
                None,
                Some(Command::new(
                    Some("cmd".to_string()),
                    vec!["-s".to_string(), "-l".to_string(), "--help".to_string()],
                )),
            ),
            Token::new(Some(Operator::LogicalAnd), None),
            Token::new(
                None,
                Some(Command::new(
                    Some("echo".to_string()),
                    vec!["hello".to_string()],
                )),
            ),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_pipe() {
        let input = r"ls -l | grep .txt";
        let tokens = tokenizer(input.to_string());

        let expected = vec![
            Token::new(
                None,
                Some(Command::new(Some("ls".to_string()), vec!["-l".to_string()])),
            ),
            Token::new(Some(Operator::Pipe), None),
            Token::new(
                None,
                Some(Command::new(
                    Some("grep".to_string()),
                    vec![r".txt".to_string()],
                )),
            ),
        ];
        assert_eq!(tokens, expected);

        let input = r"ls -l | grep .txt | wc -l";
        let tokens = tokenizer(input.to_string());

        let expected = vec![
            Token::new(
                None,
                Some(Command::new(Some("ls".to_string()), vec!["-l".to_string()])),
            ),
            Token::new(Some(Operator::Pipe), None),
            Token::new(
                None,
                Some(Command::new(
                    Some("grep".to_string()),
                    vec![r".txt".to_string()],
                )),
            ),
            Token::new(Some(Operator::Pipe), None),
            Token::new(
                None,
                Some(Command::new(Some("wc".to_string()), vec!["-l".to_string()])),
            ),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_combined() {
        let input = r"ls -l > test.txt && echo hello";
        let tokens = tokenizer(input.to_string());

        let expected = vec![
            Token::new(
                None,
                Some(Command::new(Some("ls".to_string()), vec!["-l".to_string()])),
            ),
            Token::new(
                Some(Operator::OutputRedir(OutputRedir::new(
                    "test.txt".to_string(),
                ))),
                None,
            ),
            Token::new(Some(Operator::LogicalAnd), None),
            Token::new(
                None,
                Some(Command::new(
                    Some("echo".to_string()),
                    vec!["hello".to_string()],
                )),
            ),
        ];
        assert_eq!(tokens, expected);

        let input = r"ls -l > test.txt && echo hello && cat test.txt";
        let tokens = tokenizer(input.to_string());

        let expected = vec![
            Token::new(
                None,
                Some(Command::new(Some("ls".to_string()), vec!["-l".to_string()])),
            ),
            Token::new(
                Some(Operator::OutputRedir(OutputRedir::new(
                    "test.txt".to_string(),
                ))),
                None,
            ),
            Token::new(Some(Operator::LogicalAnd), None),
            Token::new(
                None,
                Some(Command::new(
                    Some("echo".to_string()),
                    vec!["hello".to_string()],
                )),
            ),
            Token::new(Some(Operator::LogicalAnd), None),
            Token::new(
                None,
                Some(Command::new(
                    Some("cat".to_string()),
                    vec!["test.txt".to_string()],
                )),
            ),
        ];
        assert_eq!(tokens, expected);

        let input = r"ls -l > test.txt && echo hello && cat test.txt | grep .txt";
        let tokens = tokenizer(input.to_string());

        let expected = vec![
            Token::new(
                None,
                Some(Command::new(Some("ls".to_string()), vec!["-l".to_string()])),
            ),
            Token::new(
                Some(Operator::OutputRedir(OutputRedir::new(
                    "test.txt".to_string(),
                ))),
                None,
            ),
            Token::new(Some(Operator::LogicalAnd), None),
            Token::new(
                None,
                Some(Command::new(
                    Some("echo".to_string()),
                    vec!["hello".to_string()],
                )),
            ),
            Token::new(Some(Operator::LogicalAnd), None),
            Token::new(
                None,
                Some(Command::new(
                    Some("cat".to_string()),
                    vec!["test.txt".to_string()],
                )),
            ),
            Token::new(Some(Operator::Pipe), None),
            Token::new(
                None,
                Some(Command::new(
                    Some("grep".to_string()),
                    vec![r".txt".to_string()],
                )),
            ),
        ];

        assert_eq!(tokens, expected);
    }
}
