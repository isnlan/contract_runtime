use argparse::{ArgumentParser, Store};
use std::io::{stderr, stdout};
use error::*;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    Help,
    Build,
    Setup,
    Command(String),
    Unknown(String),
}

impl CommandType {
    pub fn new(command: &str) -> Result<CommandType> {
        let v = command
            .split(" ")
            .map(|s| String::from(s))
            .collect::<Vec<String>>();

        let c = match v[0].as_ref() {
            "help" => CommandType::Help,
            "build" => CommandType::Build,
            "setup" => CommandType::Setup,
            "command" => {
                let mut input = "".to_string();
                {
                    let mut ap = ArgumentParser::new();
                    ap.refer(&mut input)
                        .add_option(&["--input"], Store, r#"Output source"#);
                    ap.parse(v, &mut stdout(), &mut stderr()).map_err( |_|anyhow!("parse command error"))?;
                }

                CommandType::Command(input)
            }
            _ => CommandType::Unknown(command.to_string()),
        };
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_command_tyep() {
        assert_eq!(CommandType::Help, CommandType::new("help"));
        assert_eq!(CommandType::Build, CommandType::new("build fs"));
        assert_eq!(
            CommandType::Command(String::from("s")),
            CommandType::new("command --input s")
        );
    }
}
