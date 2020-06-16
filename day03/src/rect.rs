#[derive(PartialEq, Debug)]
pub struct Rect {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(PartialEq, Debug)]
pub enum RectError {
    Parse(std::num::ParseIntError),
    Missing(String),
}

impl std::fmt::Display for RectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RectError::Parse(e) => write!(f, "Parse Error: {}", e),
            RectError::Missing(e) => write!(f, "Missing Error: {}", e),
        }
    }
}

impl std::error::Error for RectError {}

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

        let x: i32 = coordinates
            .next()
            .ok_or(RectError::is_missing("X position"))?
            .parse()?;
        let y: i32 = coordinates
            .next()
            .ok_or(RectError::is_missing("Y position"))?
            .parse()?;

        let mut dimensions = tokens
            .next()
            .ok_or(RectError::is_missing("dimensions"))?
            .trim()
            .split("x");

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

#[cfg(test)]
mod test_rect {
    use super::*;
    use rstest::rstest;

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
    fn test_parse_error() {
        let result = "#300 @ d,10 : 200,300".parse::<Rect>();
        assert!(match result {
            Err(RectError::Parse(_)) => true,
            Err(_) => false,
            Ok(_) => false,
        });
    }

    #[rstest(
        input,
        message,
        case("", "definition"),
        case("#123", "definition"),
        case("#123 @ 11", "Y position"),
        case("#123 @ 11,12 : 100", "height")
    )]
    fn test_missing(input: &str, message: &str) {
        let result = input.parse::<Rect>();
        assert_eq!(result, Err(RectError::is_missing(message)));
    }
}
