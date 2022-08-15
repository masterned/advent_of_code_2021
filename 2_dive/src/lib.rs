use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug)]
pub struct CommandParseError(String);

impl Display for CommandParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command could not be parsed...")
    }
}

impl Error for CommandParseError {}

#[derive(Debug)]
pub enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(' ').collect();

        if parts.len() != 2 {
            return Err(CommandParseError("number arguments".into()));
        }

        let amount: Result<i32, _> = parts[1].parse();

        match parts[0] {
            "forward" => {
                let amount = amount.map_err(|_| CommandParseError("forward number".into()))?;
                Ok(Command::Forward(amount))
            }
            "up" => {
                let amount = amount.map_err(|_| CommandParseError("up number".into()))?;
                Ok(Command::Up(amount))
            }
            "down" => {
                let amount = amount.map_err(|_| CommandParseError("down number".into()))?;
                Ok(Command::Down(amount))
            }
            _ => Err(CommandParseError(format!("direction: {}", parts[0]))),
        }
    }
}

pub fn process_commands_basic(commands: &[Command]) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for command in commands {
        match command {
            Command::Forward(amount) => {
                x += amount;
            }
            Command::Up(amount) => {
                y -= amount;
            }
            Command::Down(amount) => {
                y += amount;
            }
        }
    }

    x * y
}
