// https://adventofcode.com/2018/day/2

#[macro_use]
extern crate clap;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

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

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(day2 =>
        (@arg input: +required)
    )
    .get_matches();

    let input = matches.value_of("input").ok_or("input missing")?;

    let mut file = File::open(input)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let boxes = contents.split_whitespace().collect::<Vec<&str>>();

    let result = find_similar_box_id(&boxes);

    println!("{:?}", result);

    Ok(())
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
