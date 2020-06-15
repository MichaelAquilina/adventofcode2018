// https://adventofcode.com/2018/day/9

mod config;

use config::Config;

use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::io::Read;

#[derive(Debug)]
struct Game {
    config: Config,
    board: VecDeque<u32>,
    scores: HashMap<u32, u32>,
    current_index: usize,
    current_player: u32,
    current_marble: u32,
}

fn cycle_clockwise<T>(board: &mut VecDeque<T>) {
    let old_marble = board.pop_front().unwrap();
    board.push_back(old_marble);
}

fn cycle_anticlockwise<T>(board: &mut VecDeque<T>) {
    let old_marble = board.pop_back().unwrap();
    board.push_front(old_marble);
}

impl Game {
    fn new(config: Config) -> Game {
        let mut board: VecDeque<u32> = VecDeque::with_capacity(config.max_points as usize);
        board.push_back(0);
        Game {
            config,
            board,
            scores: HashMap::new(),
            current_index: 0,
            current_player: 0,
            current_marble: 0,
        }
    }

    fn highest_score(&self) -> Option<u32> {
        self.scores.values().copied().max()
    }

    /// play a round of the game.
    /// The Current marble of the board is always in the front
    /// rotating clockwise is just a matter of traversing from the current marble
    fn play(&mut self) {
        self.current_marble += 1;
        let marble = self.current_marble;

        if marble % 23 == 0 {
            // move the current index of the board backwards
            for _ in 0..7 {
                cycle_anticlockwise(&mut self.board);
            }
            let value = self.board.pop_front().unwrap();

            let score = self.scores.entry(self.current_player).or_insert(0);
            *score += marble + value;
        } else {
            // move the current index of the board two spaces
            cycle_clockwise(&mut self.board);
            cycle_clockwise(&mut self.board);
            self.board.push_front(marble);
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

    let mut config: Config = contents.parse()?;
    config.max_points *= 100;

    let mut game = Game::new(config);
    game.play_all();
    println!("{:?}", game.highest_score());

    Ok(())
}

#[cfg(test)]
mod test_game {
    use super::*;
    use rstest::*;

    #[test]
    fn test_play() {
        let config = Config {
            players: 9,
            max_points: 25,
        };

        let mut game = Game::new(config);

        game.play();
        assert_eq!(game.board, VecDeque::from(vec![1, 0]));

        game.play();
        assert_eq!(game.board, VecDeque::from(vec![2, 1, 0]));

        game.play();
        assert_eq!(game.board, VecDeque::from(vec![3, 0, 2, 1]));

        game.play();
        assert_eq!(game.board, VecDeque::from(vec![4, 2, 1, 3, 0]));
    }

    #[test]
    fn test_cycle() {
        let mut board = VecDeque::from(vec![5, 6, 7, 8]);

        cycle_clockwise(&mut board);

        let expected = VecDeque::from(vec![6, 7, 8, 5]);

        assert_eq!(board, expected);

        cycle_anticlockwise(&mut board);

        let expected = VecDeque::from(vec![5, 6, 7, 8]);

        assert_eq!(board, expected);
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
        };

        let mut game = Game::new(config);
        game.play_all();

        println!("{:?}", game);
        assert_eq!(game.highest_score(), Some(points));
    }
}
