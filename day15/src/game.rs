use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

use crate::point::Point;
use crate::race::Race;
use crate::unit::Unit;
use thiserror::Error;

type Path = Vec<Point>;

#[derive(Debug, PartialEq)]
pub enum Terrain {
    Wall,
    Open,
}

impl Terrain {
    fn to_char(&self) -> char {
        match self {
            Terrain::Wall => '#',
            Terrain::Open => '.',
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub units: HashMap<Point, Unit>,
    pub terrain: Vec<Vec<Terrain>>,
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Unknown value: {0}")]
    Unknown(char),
}

impl FromStr for Game {
    type Err = GameError;
    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        let mut terrain = vec![];
        let mut units = HashMap::new();

        for (y, line) in contents.lines().enumerate() {
            let mut row = vec![];
            for (x, value) in line.chars().enumerate() {
                if value == 'G' {
                    units.insert(Point { x, y }, Unit::new(Race::Goblin));
                    row.push(Terrain::Open);
                } else if value == 'E' {
                    units.insert(Point { x, y }, Unit::new(Race::Elf));
                    row.push(Terrain::Open);
                } else if value == '#' {
                    row.push(Terrain::Wall);
                } else if value == '.' {
                    row.push(Terrain::Open);
                } else {
                    return Err(GameError::Unknown(value));
                }
            }
            terrain.push(row);
        }

        Ok(Game { terrain, units })
    }
}

impl Game {
    fn width(&self) -> usize {
        self.terrain[0].len()
    }

    fn height(&self) -> usize {
        self.terrain.len()
    }

    /// Get points adjacent to the current point.
    /// Only cardinal directions are considered to be
    /// adjacent
    /// TODO: Can we remove filter_units? Feels hacky
    fn get_adjacent(&self, point: &Point, filter_units: bool) -> Vec<Point> {
        let mut result = Vec::with_capacity(4);

        // The order of checks below is purposefully done
        // in reading order

        if point.y > 0 {
            result.push(Point {
                x: point.x,
                y: point.y - 1,
            });
        }

        if point.x > 0 {
            result.push(Point {
                x: point.x - 1,
                y: point.y,
            });
        }

        if point.x + 1 < self.width() {
            result.push(Point {
                x: point.x + 1,
                y: point.y,
            });
        }

        if point.y + 1 < self.height() {
            result.push(Point {
                x: point.x,
                y: point.y + 1,
            });
        }

        if filter_units {
            result
                .into_iter()
                .filter(|p| self.is_position_free(&p))
                .collect()
        } else {
            result
        }
    }

    /// Perform a breadth first search on the map given two points
    fn breadth_first_search(&self, point: &Point) -> Vec<Vec<u32>> {
        // initialise all the necessary variables
        let mut weights = vec![vec![u32::MAX; self.width()]; self.height()];

        let mut visited = HashSet::new();
        visited.insert(*point);
        weights[point.y][point.x] = 0;

        let mut queue = VecDeque::new();
        for next in self.get_adjacent(point, true) {
            weights[next.y][next.x] = 1;
            queue.push_back(next);
        }

        // start populating the weights
        while let Some(next) = queue.pop_front() {
            visited.insert(next);

            for point in self.get_adjacent(&next, true) {
                if !visited.contains(&point) && !queue.contains(&point) {
                    weights[point.y][point.x] = weights[next.y][next.x] + 1;
                    queue.push_back(point);
                }
            }
        }

        weights
    }

    fn get_path(&self, point_a: &Point, point_b: &Point) -> Option<Path> {
        let weights = self.breadth_first_search(&point_b);
        let mut result = vec![];
        let mut current = *point_a;

        while &current != point_b {
            let next = self
                .get_adjacent(&current, true)
                .into_iter()
                .filter(|p| !result.contains(p))
                // From the rust docs:
                // If several elements are equally minimum, the first element is returned
                .min_by_key(|p| weights[p.y][p.x]);

            if let Some(next) = next {
                current = next;
                result.push(current);
            } else {
                // Unable to reach destination
                // Treat the turn as a skip
                return None;
            }
        }

        Some(result)
    }

    fn is_position_free(&self, position: &Point) -> bool {
        self.terrain[position.y][position.x] == Terrain::Open && !self.units.contains_key(&position)
    }

    fn get_adjacent_target(&self, point: &Point, race: Race) -> Option<Point> {
        // To attack, the unit first determines all of the targets that are in range of it
        // by being immediately adjacent to it. If there are no such targets, the unit ends its turn.
        // Otherwise, the adjacent target with the fewest hit points is selected; in a tie,
        // the adjacent target with the fewest hit points which is first in reading order is selected.
        let mut min_hit_points = u32::MAX;
        let mut result = None;
        for position in self.get_adjacent(point, false) {
            if let Some(unit) = self.units.get(&position) {
                if unit.race == race {
                    if result.is_none()
                        || unit.hit_points < min_hit_points
                        || (unit.hit_points == min_hit_points && position < result.unwrap())
                    {
                        result = Some(position);
                        min_hit_points = unit.hit_points;
                    }
                }
            }
        }

        result
    }

    /// Retrieves all units of the specified race who still are
    /// still in the game and have an available space adjacent
    /// to them.
    fn get_free_positions(&self, race: Race) -> Vec<Point> {
        let mut results = vec![];

        for (position, unit) in &self.units {
            if unit.race == race {
                let adjacent = self.get_adjacent(&position, true);
                results.extend(adjacent);
            }
        }

        results.sort_unstable();
        results
    }

    pub fn render_map(&self) -> String {
        let mut result = vec![];

        for (y, row) in self.terrain.iter().enumerate() {
            for (x, terrain) in row.iter().enumerate() {
                if let Some(unit) = self.units.get(&Point { x, y }) {
                    result.push(format!("{}", unit.to_char()));
                } else {
                    result.push(format!("{}", terrain.to_char()));
                }
            }
            result.push(String::from("\n"));
        }

        result.into_iter().collect()
    }

    fn get_next_step(&self, point: &Point, race: Race) -> Option<Point> {
        // First calculate distance to all points
        let weights = self.breadth_first_search(&point);

        let free_positions = self.get_free_positions(race.enemy());
        let target = free_positions.iter().min_by_key(|p| weights[p.y][p.x]);

        if let Some(point_b) = target {
            if let Some(mut path) = self.get_path(&point, &point_b) {
                // is there a nicer way of doing this?
                // I just want to remove the first element in the list
                // without having to need to use a VecDequeue
                path.reverse();
                return path.pop();
            }
        }

        None
    }

    /// Checks if the end condition for the game has been satisfied
    /// The game ends when there is only one remaining race left on the board
    fn game_completed(&self) -> bool {
        let unique_units = self.units.values().map(|u| u.race).collect::<HashSet<_>>();
        unique_units.len() <= 1
    }

    pub fn total_hp_remaining(&self) -> u32 {
        self.units.values().map(|u| u.hit_points).sum()
    }

    pub fn play(&mut self, max_rounds: u64) -> u64 {
        let mut index = 0;
        println!();
        println!("Initial");
        println!("{}", self.render_map());

        loop {
            if index >= max_rounds {
                return index;
            }

            let completed = self.next();

            // TODO: I think there is a bug in how we increment index here
            // I think it should only increment if possibly the entire round
            // from self.next completed and do not increment otherwise?
            // Confirmed: You need to determine the outcome of the battle: the
            // number of full rounds that were completed (not counting the round in which
            // combat ends)
            if completed {
                index += 1;
            }
            println!("Round: {}", index);
            println!("{}", self.render_map());

            if self.game_completed() {
                return index;
            }
        }
    }

    fn next(&mut self) -> bool {
        // we copy here to prevent an immutable borrow from allowing us to
        // mutate the contents in the loop below
        let mut keys = self.units.keys().copied().collect::<Vec<Point>>();

        keys.sort_unstable();

        for point in keys {
            if self.game_completed() {
                return false;
            }

            // TODO: below is quite messy, could do with a cleanup
            if let Some(unit) = self.units.get(&point) {
                let unit = unit.clone();
                if let Some(enemy_position) = self.get_adjacent_target(&point, unit.race.enemy()) {
                    let mut enemy = self.units.get_mut(&enemy_position).unwrap();
                    unit.attack(&mut enemy);

                    if !enemy.is_alive() {
                        self.units.remove(&enemy_position);
                    }
                } else if let Some(step) = self.get_next_step(&point, unit.race) {
                    // move the unit if there is a step
                    self.units.remove(&point);
                    self.units.insert(step, unit.clone());

                    // TODO: This repeated code, find a nicer way of doing this!
                    if let Some(enemy_position) = self.get_adjacent_target(&step, unit.race.enemy())
                    {
                        let mut enemy = self.units.get_mut(&enemy_position).unwrap();
                        unit.attack(&mut enemy);

                        if !enemy.is_alive() {
                            self.units.remove(&enemy_position);
                        }
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test_game {
    use super::*;

    #[test]
    fn test_path_finding() -> Result<(), GameError> {
        #[rustfmt::skip]
        let game: Game = [
            "#######",
            "#.E#.G#",
            "#..#..#",
            "#.....#",
            "#######",
        ]
        .join("\n")
        .parse()?;

        let weights = game.breadth_first_search(&Point { x: 1, y: 1 });
        let expected = vec![
            vec![u32::MAX; 7],
            vec![u32::MAX, 0, u32::MAX, u32::MAX, 7, u32::MAX, u32::MAX],
            vec![u32::MAX, 1, 2, u32::MAX, 6, 7, u32::MAX],
            vec![u32::MAX, 2, 3, 4, 5, 6, u32::MAX],
            vec![u32::MAX; 7],
        ];
        assert_eq!(weights, expected);

        let path = game.get_path(&Point { x: 5, y: 1 }, &Point { x: 1, y: 1 });
        let expected = vec![
            Point { x: 4, y: 1 },
            Point { x: 4, y: 2 },
            Point { x: 4, y: 3 },
            Point { x: 3, y: 3 },
            Point { x: 2, y: 3 },
            Point { x: 2, y: 2 },
            Point { x: 1, y: 2 },
            Point { x: 1, y: 1 },
        ];
        assert_eq!(path, Some(expected));

        let step = game.get_next_step(&Point { x: 5, y: 1 }, Race::Goblin);
        assert_eq!(step, Some(Point { x: 4, y: 1 }));

        Ok(())
    }

    #[test]
    fn test_provided_simulation_1() -> Result<(), GameError> {
        #[rustfmt::skip]
        let mut game: Game = [
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
        ]
        .join("\n")
        .parse()?;

        let rounds = game.play(50);
        let total_hp = game.total_hp_remaining();

        assert_eq!((rounds, total_hp), (47, 590));

        Ok(())
    }

    #[test]
    fn test_provided_simulation_2() -> Result<(), GameError> {
        #[rustfmt::skip]
        let mut game: Game = [
            "#######",
            "#G..#E#",
            "#E#E.E#",
            "#G.##.#",
            "#...#E#",
            "#...E.#",
            "#######",
        ]
        .join("\n")
        .parse()?;

        let rounds = game.play(50);
        let total_hp = game.total_hp_remaining();

        assert_eq!((rounds, total_hp), (37, 982));

        Ok(())
    }

    #[test]
    fn test_provided_simulation_3() -> Result<(), GameError> {
        #[rustfmt::skip]
        let mut game: Game = [
            "#######",
            "#E..EG#",
            "#.#G.E#",
            "#E.##E#",
            "#G..#.#",
            "#..E#.#",
            "#######",
        ]
        .join("\n")
        .parse()?;

        let rounds = game.play(50);
        let total_hp = game.total_hp_remaining();

        assert_eq!((rounds, total_hp), (46, 859));

        Ok(())
    }

    #[test]
    fn test_provided_simulation_4() -> Result<(), GameError> {
        #[rustfmt::skip]
        let mut game: Game = [
            "#######",
            "#E.G#.#",
            "#.#G..#",
            "#G.#.G#",
            "#G..#.#",
            "#...E.#",
            "#######",
        ]
        .join("\n")
        .parse()?;

        let rounds = game.play(50);
        let total_hp = game.total_hp_remaining();

        assert_eq!((rounds, total_hp), (35, 793));

        Ok(())
    }

    #[test]
    fn test_provided_simulation_5() -> Result<(), GameError> {
        #[rustfmt::skip]
        let mut game: Game = [
            "#######",
            "#.E...#",
            "#.#..G#",
            "#.###.#",
            "#E#G#G#",
            "#...#G#",
            "#######",
        ]
        .join("\n")
        .parse()?;

        let rounds = game.play(60);
        let total_hp = game.total_hp_remaining();

        assert_eq!((rounds, total_hp), (54, 536));

        Ok(())
    }

    #[test]
    fn test_provided_simulation_6() -> Result<(), GameError> {
        #[rustfmt::skip]
        let mut game: Game = [
            "#########",
            "#G......#",
            "#.E.#...#",
            "#..##..G#",
            "#...##..#",
            "#...#...#",
            "#.G...G.#",
            "#.....G.#",
            "#########",
        ]
        .join("\n")
        .parse()?;

        let rounds = game.play(50);
        let total_hp = game.total_hp_remaining();

        assert_eq!((rounds, total_hp), (20, 937));

        Ok(())
    }

    #[test]
    fn test_provided_example() -> Result<(), GameError> {
        #[rustfmt::skip]
        let map_string = [
            "#######",
            "#.G.E.#",
            "#E.G.E#",
            "#.G.E.#",
            "#######",
        ]
        .join("\n");

        let game: Game = map_string.parse()?;

        let expected_units: HashMap<Point, Unit> = vec![
            (Point { x: 2, y: 1 }, Unit::new(Race::Goblin)),
            (Point { x: 4, y: 1 }, Unit::new(Race::Elf)),
            (Point { x: 1, y: 2 }, Unit::new(Race::Elf)),
            (Point { x: 3, y: 2 }, Unit::new(Race::Goblin)),
            (Point { x: 5, y: 2 }, Unit::new(Race::Elf)),
            (Point { x: 2, y: 3 }, Unit::new(Race::Goblin)),
            (Point { x: 4, y: 3 }, Unit::new(Race::Elf)),
        ]
        .into_iter()
        .collect();

        assert_eq!(game.units, expected_units);

        assert_eq!(game.width(), 7);
        assert_eq!(game.height(), 5);

        assert!(game.is_position_free(&Point { x: 1, y: 1 }));
        assert!(game.is_position_free(&Point { x: 3, y: 1 }));
        assert!(!game.is_position_free(&Point { x: 0, y: 0 }));
        assert!(!game.is_position_free(&Point { x: 2, y: 1 }));

        let render = game.render_map();

        // render_map produces a trailing newline
        assert_eq!(render.trim_end_matches("\n"), map_string);

        Ok(())
    }
}
