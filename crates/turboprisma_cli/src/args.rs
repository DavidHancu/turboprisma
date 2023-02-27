use std::collections::HashMap;

impl Args {
    pub fn empty_commands(&mut self) -> bool {
        return self.commands.first.is_none() && self.commands.second.is_none();
    }
    pub fn try_parse() -> Result<Self, CLIError> {
        let mut args = std::env::args().skip(1).peekable();

        if !args.peek().is_some() {
            return Ok(Args {
                help: true,
                commands: Commands {
                    first: None,
                    second: None
                },
                flags: vec![],
                options: HashMap::new()
            })
        }

        let mut help = false;
        let mut flags: Vec<String> = vec![];
        let mut save: Option<String> = None;
        let mut options: HashMap<String, String> = HashMap::new();
        let mut commands = Commands {
            first: None,
            second: None
        };

        for arg in args {
            if arg.eq_ignore_ascii_case("-h") || arg.eq_ignore_ascii_case("--help")
            {
                help = true;
                continue;
            }

            if arg.eq_ignore_ascii_case("-v") 
            {
                if commands.first.is_none() {
                    commands.first = Some("-v".to_string());
                    continue;
                }
            }

            let (first, rest) = split_first_rest(&arg);
            if first == "-"
            {
                if save.is_some() {
                    flags.push(save.unwrap());
                }

                let (second, second_rest) = split_first_rest(rest);
                if second == "-" 
                {
                    let (third, _flag_name) = split_first_rest(second_rest);
                    if third == "-"
                    {
                        return Err(CLIError { 
                            context: arg, 
                            kind: ErrorType::InvalidFlag
                        })
                    }
                }

                save = Some(arg);
                continue;
            }

            if save.is_some() {
                options.insert(save.unwrap(), arg);
                save = None;
            } else {
                if commands.first.is_none() {
                    commands.first = Some(arg);
                } else if commands.second.is_none() {
                    commands.second = Some(arg);
                }
            }
        }

        if save.is_some() {
            flags.push(save.unwrap());
        }

        return Ok(Args {
            help,
            commands,
            flags,
            options
        })
    }
}

#[derive(Debug)]
pub struct Args {
    pub help: bool,
    pub commands: Commands,
    pub flags: Vec<String>,
    pub options: HashMap<String, String>
}

#[derive(Debug)]
pub struct Commands {
    pub first: Option<String>,
    pub second: Option<String>
}

#[derive(Debug)]
pub enum ErrorType {
    InvalidFlag
}

#[derive(Debug)]
pub struct CLIError {
    pub context: String,
    pub kind: ErrorType
}

fn split_first_rest(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}