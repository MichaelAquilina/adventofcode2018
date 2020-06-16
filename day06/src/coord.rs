use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub enum CoordError {
    Parse(std::num::ParseIntError),
    Missing(String),
}

impl Display for CoordError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            CoordError::Missing(m) => write!(f, "Missing: {}", m),
            CoordError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl Error for CoordError {}

impl From<std::num::ParseIntError> for CoordError {
    fn from(error: std::num::ParseIntError) -> Self {
        CoordError::Parse(error)
    }
}

impl std::str::FromStr for Coord {
    type Err = CoordError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(",");
        let x = tokens
            .next()
            .ok_or(CoordError::Missing(String::from("x")))?
            .trim()
            .parse()?;
        let y = tokens
            .next()
            .ok_or(CoordError::Missing(String::from("y")))?
            .trim()
            .parse()?;
        Ok(Coord { x, y })
    }
}

impl Coord {
    pub fn distance_from(&self, other: &Coord) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[cfg(test)]
mod test_coord {
    use super::*;
    use rstest::rstest;

    #[rstest(input, expected,
        case("339, 345", Coord { x: 339, y: 345 }),
        case("0, 0", Coord { x: 0, y: 0 }),
        case("-1, 20", Coord { x: -1, y: 20 }),
        case("200 ,400", Coord { x: 200, y: 400 }),
    )]
    fn test_parse(input: &str, expected: Coord) -> Result<(), CoordError> {
        let result: Coord = input.parse()?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[rstest(p1, p2, expected, case("1, 2", "1, 2", 0), case("1, 2", "0, 0", 3))]
    fn test_distance_from(p1: &str, p2: &str, expected: i32) -> Result<(), CoordError> {
        let p1: Coord = p1.parse()?;
        let p2: Coord = p2.parse()?;

        let result = p1.distance_from(&p2);
        assert_eq!(result, expected);

        Ok(())
    }
}
