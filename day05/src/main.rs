use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;

    let contents = contents.trim_end();

    let result = parse_polymer(&contents, None);
    println!("no improvement: {}", result.chars().count());

    let result = improve_polymer(&contents);
    println!("with improvements: {:?}", result.chars().count());

    Ok(())
}

fn reacts(unit1: char, unit2: char) -> bool {
    unit1 != unit2 && unit1.to_ascii_uppercase() == unit2.to_ascii_uppercase()
}

fn parse_polymer(polymer: &str, ignore: Option<char>) -> String {
    let mut result: Vec<char> = vec![];

    for unit in polymer.chars() {
        if let Some(ignore) = ignore {
            if ignore.to_ascii_uppercase() == unit.to_ascii_uppercase() {
                continue;
            }
        }

        if let Some(prev_unit) = result.last() {
            if reacts(*prev_unit, unit) {
                result.pop();
                continue;
            }
        }

        // if nothing else happens, push the unit
        result.push(unit);
    }

    result.iter().collect()
}

fn improve_polymer(polymer: &str) -> String {
    let mut polymers = vec![];
    // There does not seem to a built-in for rust :(
    for unit in "abcdefghijklmnopqrstuvwxyz".chars() {
        let result = parse_polymer(&polymer, Some(unit));
        polymers.push(result);
    }

    polymers
        .into_iter()
        .min_by_key(|p| p.chars().count())
        .unwrap()
}

#[cfg(test)]
mod test_reacts {
    use super::*;
    use rstest::rstest;

    #[rstest(
        unit1,
        unit2,
        expected,
        case('c', 'C', true),
        case('U', 'u', true),
        case('A', 'A', false),
        case('d', 'd', false),
        case('a', 'b', false),
        case('F', 'X', false),
        case('G', 'b', false)
    )]
    fn test_reactions(unit1: char, unit2: char, expected: bool) {
        let result = reacts(unit1, unit2);
        println!("{} + {} => {}", unit1, unit2, result);
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod test_parse_polymer {
    use super::*;
    use rstest::rstest;

    #[rstest(
        polymer,
        output,
        case("aA", ""),
        case("abBA", ""),
        case("abAB", "abAB"),
        case("aabAAB", "aabAAB"),
        case("dabAcCaCBAcCcaDA", "dabCBAcaDA")
    )]
    fn test_examples_no_ignore(polymer: &str, output: &str) {
        let result = parse_polymer(polymer, None);

        assert_eq!(result, output);
    }

    #[rstest(
        polymer,
        ignore,
        output,
        case("dabAcCaCBAcCcaDA", 'a', "dbCBcD"),
        case("dabAcCaCBAcCcaDA", 'b', "daCAcaDA"),
        case("dabAcCaCBAcCcaDA", 'c', "daDA"),
        case("dabAcCaCBAcCcaDA", 'd', "abCBAc")
    )]
    fn test_examples_ignore(polymer: &str, ignore: char, output: &str) {
        let result = parse_polymer(polymer, Some(ignore));

        assert_eq!(result, output);
    }
}

#[cfg(test)]
mod test_improve_polymer {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(improve_polymer(""), "");
    }

    #[test]
    fn test_example() {
        let result = improve_polymer("dabAcCaCBAcCcaDA");
        assert_eq!(result, "daDA");
    }
}
