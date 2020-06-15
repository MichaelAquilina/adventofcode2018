use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub enum PointErr {
    Missing(String),
    Parse(ParseIntError),
}

#[derive(Debug, PartialEq)]
pub struct Light {
    pub position: Point,
    pub velocity: Point,
}

#[derive(Debug)]
pub enum LightErr {
    Missing(String),
    InvalidName(String),
    Parse(PointErr),
}

impl From<ParseIntError> for PointErr {
    fn from(err: ParseIntError) -> Self {
        PointErr::Parse(err)
    }
}

impl std::fmt::Display for PointErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PointErr::Missing(msg) => write!(f, "missing: {}", msg),
            PointErr::Parse(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl From<PointErr> for LightErr {
    fn from(err: PointErr) -> Self {
        LightErr::Parse(err)
    }
}

impl std::fmt::Display for LightErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LightErr::Missing(msg) => write!(f, "Missing: {}", msg),
            LightErr::InvalidName(msg) => write!(f, "Invalid name: {}", msg),
            LightErr::Parse(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl Error for LightErr {}

impl std::str::FromStr for Point {
    type Err = PointErr;
    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        let mut tokens = contents.split(",");

        let x: i32 = tokens
            .next()
            .ok_or(PointErr::Missing(String::from("x")))?
            .replace("<", "")
            .trim()
            .parse()?;

        let y: i32 = tokens
            .next()
            .ok_or(PointErr::Missing(String::from("x")))?
            .replace(">", "")
            .trim()
            .parse()?;

        Ok(Point { x, y })
    }
}

impl std::str::FromStr for Light {
    type Err = LightErr;
    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        let mut tokens = contents.split("> ");

        let mut position_tokens = tokens
            .next()
            .ok_or(LightErr::Missing(String::from("position")))?
            .split("=");

        let mut velocity_tokens = tokens
            .next()
            .ok_or(LightErr::Missing(String::from("velocity")))?
            .split("=");

        let name = position_tokens
            .next()
            .ok_or(LightErr::Missing(String::from("position name")))?;

        if name != "position" {
            return Err(LightErr::InvalidName(String::from("position")));
        }

        let position = position_tokens
            .next()
            .ok_or(LightErr::Missing(String::from("position value")))?
            .parse()?;

        let name = velocity_tokens
            .next()
            .ok_or(LightErr::Missing(String::from("velocity name")))?;

        if name != "velocity" {
            return Err(LightErr::InvalidName(String::from("velocity")));
        }

        let velocity = velocity_tokens
            .next()
            .ok_or(LightErr::Missing(String::from("velocity value")))?
            .parse()?;

        Ok(Light { position, velocity })
    }
}

#[cfg(test)]
mod test_light {
    use super::*;

    #[test]
    fn test_parse_1() -> Result<(), LightErr> {
        let result: Light = "position=< 9,  1> velocity=< 0,  2>".parse()?;

        let expected = Light {
            position: Point { x: 9, y: 1 },
            velocity: Point { x: 0, y: 2 },
        };

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_parse_2() -> Result<(), LightErr> {
        let result: Light = "position=<10, -3> velocity=<-1,  1>".parse()?;

        let expected = Light {
            position: Point { x: 10, y: -3 },
            velocity: Point { x: -1, y: 1 },
        };

        assert_eq!(result, expected);

        Ok(())
    }
}
