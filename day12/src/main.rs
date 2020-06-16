// Very inefficient solution to day 12
// https://adventofcode.com/2018/day/12#part2

use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let mut lines = contents.lines();

    let mut state = get_state(lines.next().ok_or("Missing initial state")?);
    let rules = get_rules(&mut lines);

    let result = get_plant_count(&mut state, &rules);

    println!("# plants: {}", result);

    // no part 2 yet :(

    Ok(())
}

fn get_plant_count(initial_state: &VecDeque<char>, rules: &HashSet<String>) -> i32 {
    let mut start_index = 0;
    let mut state = initial_state.clone();

    for generation in 1..=20 {
        start_index = pad(&mut state, start_index);
        state = update_generation(&state, rules);
        println!(
            "{:>02}: {} (start={})",
            generation,
            state.iter().collect::<String>(),
            start_index
        );
    }

    state
        .into_iter()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(index, _)| index as i32 + start_index)
        .sum()
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
    tokens.next(); // we dont care about "initial state: "
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

fn pad(state: &mut VecDeque<char>, start_index: i32) -> i32 {
    let first = state[0];
    let second = state[1];
    let third = state[2];
    let mut start_index = start_index;

    if first == '#' {
        state.push_front('.');
        state.push_front('.');
        state.push_front('.');
        start_index -= 3;
    } else if second == '#' {
        state.push_front('.');
        state.push_front('.');
        start_index -= 2;
    } else if third == '#' {
        state.push_front('.');
        start_index -= 1;
    }

    let size = state.len();

    let first = state[size - 1];
    let second = state[size - 2];
    let third = state[size - 3];

    if first == '#' {
        state.push_back('.');
        state.push_back('.');
        state.push_back('.');
    } else if second == '#' {
        state.push_back('.');
        state.push_back('.');
    } else if third == '#' {
        state.push_back('.');
    }

    start_index
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
mod test_get_plant_count {
    use super::*;

    #[test]
    fn test_provided_example() {
        let mut lines = vec![
            "...## => #",
            "..#.. => #",
            ".#... => #",
            ".#.#. => #",
            ".#.## => #",
            ".##.. => #",
            ".#### => #",
            "#.#.# => #",
            "#.### => #",
            "##.#. => #",
            "##.## => #",
            "###.. => #",
            "###.# => #",
            "####. => #",
        ]
        .into_iter();

        let rules = get_rules(&mut lines);
        let state = get_state("initial state: #..#.#..##......###...###");

        let result = get_plant_count(&state, &rules);

        assert_eq!(result, 325);
    }
}

#[cfg(test)]
mod test_pad {
    use super::*;
    use rstest::*;

    #[rstest(state, expected, expected_new_index,
        case("...#...", "...#...", 0),
        case("......", "......", 0),
        case("#.....", "...#.....", -3),
        case(".....#", ".....#...", 0),
        case("....#.", "....#...", 0),
        case("...#..", "...#...", 0),
    )]
    fn test_correct_output(state: &str, expected: &str, expected_new_index: i32) {
        let mut state = state.chars().collect::<VecDeque<char>>();
        let expected = expected.chars().collect::<VecDeque<char>>();

        let new_index = pad(&mut state, 0);

        assert_eq!(state, expected);
        assert_eq!(new_index, expected_new_index);
    }
}

#[cfg(test)]
mod test_window {
    use super::*;
    use rstest::*;

    #[rstest(index, expected, case(0, "....#"), case(3, ".##.."), case(12, ".##.."))]
    fn test_correct_output(index: usize, expected: &str) {
        let state: VecDeque<char> = "..##..#.#..##".chars().collect::<VecDeque<char>>();

        assert_eq!(window(&state, index), expected);
    }
}
