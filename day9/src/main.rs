// https://adventofcode.com/2018/day/9

mod config;

use config::Config;

use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

// the % operator in rust is the reminder function NOT
// the modulo operator like it is in other languages like Python
fn modulo(v1: i32, v2: i32) -> i32 {
    (v1 % v2 + v2) % v2
}

#[derive(Debug)]
struct Game {
    config: Config,
    board: Vec<u32>,
    scores: HashMap<u32, u32>,
    current_index: usize,
    current_player: u32,
    current_marble: u32,
}

impl Game {
    fn new(config: Config) -> Game {
        Game {
            config,
            board: vec![0],
            scores: HashMap::new(),
            current_index: 0,
            current_player: 0,
            current_marble: 0,
        }
    }

    fn highest_score(&self) -> Option<u32> {
        self.scores.values().copied().max()
    }

    /// Play the next turn in the game
    fn play(&mut self) {
        self.current_marble += 1;
        let marble = self.current_marble;

        if marble % self.config.bonus_point == 0 {
            self.current_index = modulo(
                self.current_index as i32 + self.config.bonus_rotation,
                self.board.len() as i32,
            ) as usize;

            let value = self.board.remove(self.current_index as usize);

            let score = self.scores.entry(self.current_player).or_insert(0);
            *score += marble + value;
        } else {
            self.current_index =
                1 + modulo(self.current_index as i32 + 1, self.board.len() as i32) as usize;
            self.board.insert(self.current_index as usize, marble);
        }
        self.current_player = (self.current_player + 1) % self.config.players;
    }

    /// Play all remaining turns till the end
    fn play_all(&mut self) {
        while !self.completed() {
            self.play();
        }
    }

    fn completed(&self) -> bool {
        self.current_marble >= self.config.max_points
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let config: Config = contents.parse()?;

    let mut game = Game::new(config);
    game.play_all();
    println!("{:?}", game.highest_score());

    Ok(())
}

#[cfg(test)]
mod test_modulo {
    use super::*;
    use rstest::*;

    #[rstest(v1, v2, result,
        case(1, 7, 1),
        case(-1, 4, 3),
        case(9, 3, 0),
        case(10, 8, 2),
    )]
    fn test_correct_output(v1: i32, v2: i32, result: i32) {
        assert_eq!(modulo(v1, v2), result);
    }
}

#[cfg(test)]
mod test_game {
    use super::*;
    use rstest::*;

    #[test]
    fn test_play() {
        let config = Config {
            players: 2,
            max_points: 5,
            bonus_point: 3,
            bonus_rotation: -2,
        };

        let mut game = Game::new(config);

        game.play();
        let expected_board = vec![0, 1];
        assert_eq!(game.board, expected_board);

        game.play();
        let expected_board = vec![0, 2, 1];
        assert_eq!(game.board, expected_board);

        game.play();
        let expected_board = vec![0, 2];
        assert_eq!(game.board, expected_board);
        assert_eq!(game.highest_score(), Some(4));

        game.play();
        let expected_board = vec![0, 2, 4];
        assert_eq!(game.board, expected_board);

        game.play();
        let expected_board = vec![0, 5, 2, 4];
        assert_eq!(game.board, expected_board);

        assert!(game.completed())
    }

    #[test]
    fn test_board_state() {
        let config = Config {
            players: 9,
            max_points: 25,
            bonus_point: 23,
            bonus_rotation: -7,
        };

        let mut game = Game::new(config);
        game.play_all();

        let expected_board = vec![
            0, 16, 8, 17, 4, 18, 19, 2, 24, 20, 25, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15,
        ];

        assert_eq!(game.board, expected_board);
    }

    #[rstest(
        players,
        max_points,
        points,
        case(9, 25, 32),
        // below 2 are extra examples provided by this reddit post for
        // additional cases to debug
        // https://www.reddit.com/r/adventofcode/comments/a4ipsk/day_9_part_one_my_code_works_for_the_worked/#t1_ebetbcr
        case(9, 48, 63),
        case(1, 48, 95),
        case(10, 1618, 8317),
        case(13, 7999, 146373),
        case(17, 1104, 2764),
        case(21, 6111, 54718),
        case(30, 5807, 37305)
    )]
    fn test_provided_examples(players: u32, max_points: u32, points: u32) {
        let config = Config {
            players,
            max_points,
            bonus_point: 23,
            bonus_rotation: -7,
        };

        let mut game = Game::new(config);
        game.play_all();

        println!("{:?}", game);
        assert_eq!(game.highest_score(), Some(points));
    }
}
