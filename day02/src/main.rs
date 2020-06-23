// https://adventofcode.com/2018/day/2

use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let boxes = contents.split_whitespace().collect::<Vec<&str>>();

    let result = get_solution_1(&boxes);
    println!("{}", result);

    let result = find_similar_box_id(&boxes);
    println!("{:?}", result);

    Ok(())
}

fn get_solution_1(boxes: &[&str]) -> i32 {
    let mut twice = 0;
    let mut thrice = 0;
    for value in boxes {
        let result = check_box_id(value);
        twice += result.0 as i32;
        thrice += result.1 as i32;
    }

    twice * thrice
}

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

fn compare_box_ids(box1: &str, box2: &str) -> String {
    let mut result = vec![];

    for (c1, c2) in box1.chars().zip(box2.chars()) {
        if c1 == c2 {
            result.push(c1);
        }
    }
    return result.iter().collect::<String>();
}

fn find_similar_box_id(boxes: &[&str]) -> Option<String> {
    for box1 in boxes {
        for box2 in boxes {
            if box1 == box2 {
                continue;
            }
            let result = compare_box_ids(box1, box2);
            if result.chars().count() == box1.chars().count() - 1 {
                return Some(result);
            }
        }
    }
    None
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

#[cfg(test)]
mod test_find_similar_box_id {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(find_similar_box_id(&[]), None);
    }

    #[test]
    fn test_no_similar() {
        assert_eq!(find_similar_box_id(&["abdef", "podid", "12345"]), None);
    }

    #[test]
    fn test_finds_similar() {
        let input = vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];
        assert_eq!(find_similar_box_id(&input), Some(String::from("fgij")));
    }
}

#[cfg(test)]
mod test_compare_box_ids {
    use super::*;
    use rstest::rstest;

    #[rstest(
        box1,
        box2,
        expected,
        case("abcde", "abcde", "abcde"),
        case("abcde", "axcye", "ace"),
        case("fghij", "fguij", "fgij")
    )]
    fn test_cases(box1: &str, box2: &str, expected: &str) {
        assert_eq!(compare_box_ids(box1, box2), expected);
    }
}
