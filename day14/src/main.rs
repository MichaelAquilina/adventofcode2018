// https://adventofcode.com/2018/day/14

fn main() {
    let n = 286051;

    let result = improve_recipes(n, false);

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

fn improve_recipes(n: usize, print: bool) -> String {
    let mut recipes = vec![3, 7];

    let mut index_1 = 0;
    let mut index_2 = 1;

    for _ in 0..n + 10 {
        if print {
            print_recipes(&recipes, index_1, index_2);
        }

        let value = recipes[index_1] + recipes[index_2];
        let (recipe_1, recipe_2) = split_decimal(value);

        if recipe_1 != 0 {
            recipes.push(recipe_1);
        }
        recipes.push(recipe_2);

        // the Elf steps forward through the scoreboard a number of recipes equal
        // to 1 plus the score of their current recipe.
        // If they run out of recipes, they loop back around to the beginning.
        index_1 = (index_1 + recipes[index_1] + 1) % recipes.len();
        index_2 = (index_2 + recipes[index_2] + 1) % recipes.len();
    }

    recipes[n..n + 10]
        .iter()
        .map(|v| format!("{}", v))
        .collect::<String>()
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
    fn test_provided_examples(n: usize, expected: &str) {
        assert_eq!(improve_recipes(n, false), expected);
    }
}
