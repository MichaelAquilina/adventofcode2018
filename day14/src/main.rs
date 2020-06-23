// https://adventofcode.com/2018/day/14

fn main() {
    let n = 286051;

    let result = improve_recipes(n, false);
    println!("{}", result);

    let result = find_recipes(&n.to_string(), false);
    println!("{}", result);
}

fn split_decimal(value: usize) -> (usize, usize) {
    (value / 10, value % 10)
}

fn print_recipes(recipes: &[usize], index_1: usize, index_2: usize) {
    let mut buffer = vec![];

    for (index, value) in recipes.iter().enumerate() {
        buffer.push(if index == index_1 {
            format!("({})", value)
        } else if index == index_2 {
            format!("[{}]", value)
        } else {
            format!(" {} ", value)
        });
    }

    println!("{}", buffer.join(""));
}

fn improve(recipes: &mut Vec<usize>, index_1: &mut usize, index_2: &mut usize) {
    let value = recipes[*index_1] + recipes[*index_2];
    let (recipe_1, recipe_2) = split_decimal(value);

    if recipe_1 != 0 {
        recipes.push(recipe_1);
    }
    recipes.push(recipe_2);

    // the Elf steps forward through the scoreboard a number of recipes equal
    // to 1 plus the score of their current recipe.
    // If they run out of recipes, they loop back around to the beginning.
    *index_1 = (*index_1 + recipes[*index_1] + 1) % recipes.len();
    *index_2 = (*index_2 + recipes[*index_2] + 1) % recipes.len();
}

fn improve_recipes(n: usize, print: bool) -> String {
    let mut recipes = vec![3, 7];

    let mut index_1 = 0;
    let mut index_2 = 1;

    for _ in 0..n + 10 {
        if print {
            print_recipes(&recipes, index_1, index_2);
        }

        improve(&mut recipes, &mut index_1, &mut index_2);
    }

    recipes[n..n + 10]
        .iter()
        .map(|v| v.to_string())
        .collect::<String>()
}

// FIXME: This part is quite slow!
fn find(recipes: &[usize], n: &str) -> Option<usize> {
    if n.len() <= recipes.len() {
        let index = recipes.len() - n.len();
        let current = recipes[index..]
            .iter()
            .map(|c| c.to_string())
            .collect::<String>();

        if current == n {
            return Some(index);
        }
    }
    if recipes.len() > 0 && n.len() <= recipes.len() - 1 {
        let index = recipes.len() - 1 - n.len();
        let current = recipes[index..recipes.len() - 1]
            .iter()
            .map(|c| c.to_string())
            .collect::<String>();

        if current == n {
            return Some(index);
        }
    }
    None
}

fn find_recipes(n: &str, print: bool) -> usize {
    let mut recipes = vec![3, 7];

    let mut index_1 = 0;
    let mut index_2 = 1;

    loop {
        if print {
            print_recipes(&recipes, index_1, index_2);
        }

        improve(&mut recipes, &mut index_1, &mut index_2);

        if let Some(index) = find(&recipes, n) {
            return index;
        }
    }
}

#[cfg(test)]
mod test_find_recipes {
    use super::*;
    use rstest::*;

    #[rstest(recipes, n, expected,
        case(&[1, 0, 2, 3, 4], "234", Some(2)),
        case(&[1, 0, 2, 3, 4, 5], "234", Some(2)),
        case(&[], "1000", None),
        case(&[1, 0, 0, 0], "1000", Some(0)),
        case(&[1, 0, 2, 2], "1000", None),
    )]
    fn test_find(recipes: &[usize], n: &str, expected: Option<usize>) {
        assert_eq!(find(recipes, n), expected);
    }

    #[rstest(
        value,
        expected,
        case("51589", 9),
        case("01245", 5),
        case("92510", 18),
        case("59414", 2018)
    )]
    fn test_provided_examples_case_2(value: &str, expected: usize) {
        assert_eq!(find_recipes(value, false), expected);
    }
}

#[cfg(test)]
mod test_improve_recipes {
    use super::*;
    use rstest::*;

    #[rstest(value, expected,
        case(10, (1, 0)),
        case(56, (5, 6)),
        case(7, (0, 7)),
    )]
    fn test_split_decimal(value: usize, expected: (usize, usize)) {
        assert_eq!(split_decimal(value), expected);
    }

    #[rstest(
        n,
        expected,
        case(9, "5158916779"),
        case(5, "0124515891"),
        case(18, "9251071085"),
        case(2018, "5941429882")
    )]
    fn test_provided_examples_case_1(n: usize, expected: &str) {
        assert_eq!(improve_recipes(n, false), expected);
    }
}
