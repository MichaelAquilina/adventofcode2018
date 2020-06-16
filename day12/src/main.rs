use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let mut lines = contents.lines();

    let mut state = get_state(lines.next().ok_or("Missing initial state")?);
    let rules = get_rules(&mut lines);

    for index in 1..=20 {
        state = update_generation(&mut state, &rules);
        println!("{:>02}: {}", index, state.iter().collect::<String>());
    }

    Ok(())
}

fn get_rules<'a, T: Iterator<Item = &'a str>>(lines: &'a mut T) -> HashSet<String> {
    let mut result = HashSet::new();

    for line in lines {
        if line == "" {
            continue;
        }

        let mut tokens = line.split(" => ");

        let key = tokens.next().unwrap();
        let value = tokens.next().unwrap();

        if value == "#" {
            result.insert(String::from(key));
        }
    }

    result
}

fn get_state(content: &str) -> VecDeque<char> {
    let mut tokens = content.split(": ");
    tokens.next();  // we dont care about "initial state: "
    let state = tokens.next().unwrap();

    state.chars().collect()
}

fn window(state: &VecDeque<char>, index: usize) -> String {
    let mut result = vec![];

    for i in -2..=2 {
        let target: i32 = index as i32 + i;
        if target < 0 || target as usize >= state.len() {
            result.push('.');
        } else {
            result.push(state[target as usize]);
        }
    }

    result.iter().collect()
}

fn update_generation(state: &VecDeque<char>, rules: &HashSet<String>) -> VecDeque<char> {
    let mut result = VecDeque::new();
    for index in 0..state.len() {
        let key = window(state, index);

        if rules.contains(&key) {
            result.push_back('#');
        } else {
            result.push_back('.');
        }
    }

    result
}

#[cfg(test)]
mod test_window {
    use super::*;
    use rstest::*;

    #[rstest(index, expected,
        case(0, "....#"),
        case(3, ".##.."),
        case(12, ".##.."),
    )]
    fn test_correct_output(index: usize, expected: &str) {
        let state: VecDeque<char> = VecDeque::from(
            "..##..#.#..##".chars().collect::<Vec<char>>()
        );

        assert_eq!(window(&state, index), expected);
    }
}
