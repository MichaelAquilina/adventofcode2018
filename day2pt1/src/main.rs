// https://adventofcode.com/2018/day/2

#[macro_use]
extern crate clap;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn check_box_id(box_id: &str) -> (bool, bool) {
    let mut letters = HashMap::new();

    for value in box_id.chars() {
        let counter = letters.entry(value).or_insert(0);
        *counter += 1;
    }

    let mut twice = false;
    let mut thrice = false;
    for value in letters.values() {
        match value {
            2 => twice = true,
            3 => thrice = true,
            _ => {}
        }
    }

    (twice, thrice)
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(day2 =>
        (@arg input: +required)
    )
    .get_matches();

    let input = matches.value_of("input").ok_or("input missing")?;

    let mut file = File::open(input)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let mut twice = 0;
    let mut thrice = 0;
    for line in contents.split_whitespace() {
        let result = check_box_id(line);
        twice += result.0 as i32;
        thrice += result.1 as i32;
    }

    let result = twice * thrice;

    println!("{}", result);

    Ok(())
}

#[cfg(test)]
mod test_check_box_id {
    use super::*;
    use rstest::rstest;

    #[rstest(input, expected,
        case("abcdef", (false, false)),
        case("bababc", (true, true)),
        case("abbcde", (true, false)),
        case("abcccd", (false, true)),
        case("aabcdd", (true, false)),
        case("abcdee", (true, false)),
        case("ababab", (false, true)),
    )]
    fn test_cases(input: &str, expected: (bool, bool)) {
        let result = check_box_id(input);
        assert_eq!(result, expected);
    }
}
