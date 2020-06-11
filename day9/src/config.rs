use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub players: u32,
    pub max_points: u32,
    pub bonus_point: u32,    // marble value that causes the player to take it
    pub bonus_rotation: i32, // rotation amount that happens at bonus
}

#[derive(Debug)]
pub enum ConfigErr {
    Missing(String),
    Parse(ParseIntError),
}

impl Error for ConfigErr {}

impl std::fmt::Display for ConfigErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigErr::Missing(m) => write!(f, "Missing: {}", m),
            ConfigErr::Parse(e) => write!(f, "{}", e),
        }
    }
}

impl From<ParseIntError> for ConfigErr {
    fn from(err: ParseIntError) -> Self {
        ConfigErr::Parse(err)
    }
}

impl std::str::FromStr for Config {
    type Err = ConfigErr;
    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        let mut tokens = contents.split(";");

        let players = tokens
            .next()
            .ok_or(ConfigErr::Missing(String::from("players")))?;

        let players: u32 = players.replace(" players", "").parse()?;

        let max_points = tokens
            .next()
            .ok_or(ConfigErr::Missing(String::from("max points")))?;

        let max_points: u32 = max_points
            .replace(" last marble is worth ", "")
            .replace(" points", "")
            .replace("\n", "")
            .parse()?;

        Ok(Config {
            players,
            max_points,
            bonus_point: 23,
            bonus_rotation: -7,
        })
    }
}

#[cfg(test)]
mod test_config {
    use super::*;

    #[test]
    fn test_correct() -> Result<(), ConfigErr> {
        let config: Config = "429 players; last marble is worth 70901 points\n".parse()?;

        let expected = Config {
            players: 429,
            max_points: 70901,
            bonus_point: 23,
            bonus_rotation: -7,
        };

        assert_eq!(config, expected);

        Ok(())
    }
}
