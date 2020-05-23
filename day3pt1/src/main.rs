// https://adventofcode.com/2018/day/3

#[derive(PartialEq, Debug)]
struct Rect {
    id: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(PartialEq, Debug)]
enum RectError {
    Parse(std::num::ParseIntError),
    Missing(String),
}

impl From<std::num::ParseIntError> for RectError {
    fn from(err: std::num::ParseIntError) -> Self {
        RectError::Parse(err)
    }
}

impl RectError {
    fn is_missing(message: &str) -> Self {
        RectError::Missing(String::from(message))
    }
}

impl std::str::FromStr for Rect {
    type Err = RectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split("@");

        let id = String::from(tokens.next().ok_or(RectError::is_missing("id"))?.trim());

        let definition = tokens.next().ok_or(RectError::is_missing("definition"))?;

        let mut tokens = definition.split(":");

        let mut coordinates = tokens
            .next()
            .ok_or(RectError::is_missing("coordinates"))?
            .trim()
            .split(",");

        let mut dimensions = tokens
            .next()
            .ok_or(RectError::is_missing("dimensions"))?
            .trim()
            .split("x");

        let x: i32 = coordinates
            .next()
            .ok_or(RectError::is_missing("X position"))?
            .parse()?;
        let y: i32 = coordinates
            .next()
            .ok_or(RectError::is_missing("Y position"))?
            .parse()?;

        let width = dimensions
            .next()
            .ok_or(RectError::is_missing("width"))?
            .parse()?;
        let height = dimensions
            .next()
            .ok_or(RectError::is_missing("height"))?
            .parse()?;

        Ok(Rect {
            id,
            x,
            y,
            width,
            height,
        })
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test_rect {
    use super::*;

    #[test]
    fn test_correct_value() -> Result<(), RectError> {
        let result: Rect = "#123 @ 3,2: 5x4".parse()?;

        let expected = Rect {
            id: String::from("#123"),
            x: 3,
            y: 2,
            width: 5,
            height: 4,
        };

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_missing_definition() {
        let result = "".parse::<Rect>();
        assert_eq!(result, Err(RectError::is_missing("definition")));
    }
}
