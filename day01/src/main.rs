use std::collections::HashSet;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let result = get_resulting_frequency(&contents)?;
    println!("{}", result);

    let result = find_repeat_frequency(&contents)?;
    println!("{}", result);

    Ok(())
}

fn get_resulting_frequency(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut accumulator = 0;

    for line in contents.split_whitespace() {
        let value: i32 = line.parse()?;

        accumulator += value;
    }

    Ok(accumulator)
}

fn find_repeat_frequency(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut visited = HashSet::new();
    let mut accumulator: i32 = 0;

    loop {
        for line in contents.split_whitespace() {
            visited.insert(accumulator);

            let value: i32 = line.parse()?;

            accumulator += value;

            if visited.contains(&accumulator) {
                return Ok(accumulator);
            }
        }
    }
}

#[cfg(test)]
mod test_get_resulting_frequency {
    use super::*;
    use rstest::*;

    #[rstest(contents, expected,
        case("+1 +1 +1", 3),
        case("+1 +1 -2", 0),
        case("-1 -2 -3", -6),
    )]
    fn test_provided_examples(contents: &str, expected: i32) -> Result<(), Box<dyn Error>> {
        let result = get_resulting_frequency(&contents)?;
        assert_eq!(result, expected);

        Ok(())
    }
}

#[cfg(test)]
mod test_find_repeat_frequency {
    use super::*;

    #[test]
    fn test_case_1() -> Result<(), Box<dyn Error>> {
        let result = find_repeat_frequency("+1 -1")?;
        assert_eq!(result, 0);
        Ok(())
    }

    #[test]
    fn test_case_2() -> Result<(), Box<dyn Error>> {
        let result = find_repeat_frequency("+3 +3 +4 -2 -4")?;
        assert_eq!(result, 10);
        Ok(())
    }

    #[test]
    fn test_case_3() -> Result<(), Box<dyn Error>> {
        let result = find_repeat_frequency("-6 +3 +8 +5 -6")?;
        assert_eq!(result, 5);
        Ok(())
    }

    #[test]
    fn test_case_4() -> Result<(), Box<dyn Error>> {
        let result = find_repeat_frequency("+7 +7 -2 -7 -4")?;
        assert_eq!(result, 14);
        Ok(())
    }
}
