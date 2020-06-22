use std::collections::HashMap;

use crate::cart::{turn, Cart, Direction, Position};
use crate::track::Track;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MapError {
    #[error("Unknown track type: {0}")]
    UnknownTrack(char),
    #[error("Simulation ran past limit")]
    RanPastLimit,
}

#[derive(Debug, PartialEq)]
pub struct Map {
    pub carts: Vec<Cart>,
    pub tracks: Vec<Vec<Track>>,
}

impl Map {
    pub fn print(&self) -> String {
        let mut result = vec![];

        for row in &self.tracks {
            result.push(row.iter().map(|t| t.to_char()).collect::<Vec<char>>());
        }

        for cart in &self.carts {
            let x = cart.position.x;
            let y = cart.position.y;

            result[y][x] = cart.to_char();
        }

        result
            .into_iter()
            .map(|r| r.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn get_track(&self, x: usize, y: usize) -> Track {
        self.tracks[y][x]
    }

    pub fn check_collisions(&self, dead: &[usize]) -> Option<(usize, usize)> {
        let mut positions: HashMap<&Position, usize> = HashMap::new();
        for (index, cart) in self.carts.iter().enumerate() {
            if dead.contains(&index) {
                continue;
            }

            if let Some(other) = positions.get(&cart.position) {
                return Some((index, *other));
            }
            positions.insert(&cart.position, index);
        }
        None
    }

    /// Runs 1 event loop of the cart simulation. Returns a Position
    /// of the first crash if one does occur, and None otherwise
    pub fn run(&mut self) -> Result<Vec<Position>, MapError> {
        let mut crashes = vec![];
        let mut dead = vec![];

        // we do not use standard iteration here because we need fine grained
        // control over mutating the objects inside self.carts
        for index in 0..self.carts.len() {
            if dead.contains(&index) {
                continue;
            }

            let cart = &self.carts[index];
            let x = cart.position.x;
            let y = cart.position.y;

            let position = match cart.direction {
                Direction::North => Position { x, y: y - 1 },
                Direction::South => Position { x, y: y + 1 },
                Direction::West => Position { x: x - 1, y },
                Direction::East => Position { x: x + 1, y },
            };
            let track = self.get_track(position.x, position.y);
            let (direction, turn_count) = turn(&cart, track);

            self.carts[index] = Cart {
                position,
                direction,
                turn_count,
            };

            let collision = self.check_collisions(&dead);
            if let Some((c1_index, c2_index)) = collision {
                dead.push(c1_index);
                dead.push(c2_index);
                crashes.push(self.carts[index].position.clone());
            }
        }

        self.carts = self
            .carts
            .iter()
            .enumerate()
            .filter(|(i, _)| !dead.contains(i))
            .map(|(_, c)| c.clone())
            .collect::<Vec<Cart>>();

        // Carts all move at the same speed; they take turns moving a single step at a time.
        // They do this based on their current location: carts on the top row move first
        // (acting from left to right), then carts on the second row move (again from left
        // to right), then carts on the third row, and so on.
        // Once each cart has moved one step, the process repeats;
        // each of these loops is called a tick.
        // TODO: There must be a better way to do this without cloning position
        self.carts.sort_by_cached_key(|c| c.position.clone());
        Ok(crashes)
    }

    pub fn run_until_collission(
        &mut self,
        print: bool,
        limit: usize,
    ) -> Result<Position, MapError> {
        for _ in 0..limit {
            if print {
                println!("{}\n", self.print());
            }

            let crashes = self.run()?;

            if !crashes.is_empty() {
                return Ok(crashes[0].clone());
            }
        }
        Err(MapError::RanPastLimit)
    }

    pub fn run_until_last_cart(&mut self, print: bool, limit: usize) -> Result<Position, MapError> {
        for _ in 0..limit {
            if print {
                println!("{}\n", self.print());
            }

            self.run()?;

            if self.carts.len() == 1 {
                return Ok(self.carts[0].position.clone());
            }
        }
        Err(MapError::RanPastLimit)
    }
}

impl std::str::FromStr for Map {
    type Err = MapError;
    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        // TODO: capacity could be easily estimated here
        let mut tracks: Vec<Vec<Track>> = vec![];
        let mut carts: Vec<Cart> = vec![];

        for (y, line) in contents.lines().enumerate() {
            let mut row = vec![];
            for (x, value) in line.chars().enumerate() {
                let track = match value {
                    '-' => Track::Horizontal,
                    '|' => Track::Vertical,
                    '/' => Track::DiagonalRight,
                    '\\' => Track::DiagonalLeft,
                    '+' => Track::Intersection,
                    ' ' => Track::Empty,
                    // On your initial map, the track under each cart is a straight path
                    // matching the direction the cart is facing.)
                    '^' => {
                        let cart = Cart::new(x, y, Direction::North);
                        carts.push(cart);
                        Track::Vertical
                    }
                    '>' => {
                        let cart = Cart::new(x, y, Direction::East);
                        carts.push(cart);
                        Track::Horizontal
                    }
                    '<' => {
                        let cart = Cart::new(x, y, Direction::West);
                        carts.push(cart);
                        Track::Horizontal
                    }
                    'v' => {
                        let cart = Cart::new(x, y, Direction::South);
                        carts.push(cart);
                        Track::Vertical
                    }
                    unknown => return Err(MapError::UnknownTrack(unknown)),
                };
                row.push(track);
            }
            tracks.push(row);
        }

        Ok(Map { tracks, carts })
    }
}

#[cfg(test)]
mod test_map {
    use super::*;

    mod test_run {
        use super::*;

        /// Check for cart position equality
        fn assert_cart_positions_eq(m1: &Map, m2: &Map) {
            assert_eq!(m1.carts.len(), m2.carts.len());
            for (c1, c2) in m1.carts.iter().zip(m2.carts.iter()) {
                assert_eq!(c1.position, c2.position);
                assert_eq!(c1.direction, c2.direction);
            }
        }

        // TODO: Fix this infinite loop
        // extra example found on Reddit
        // https://www.reddit.com/r/adventofcode/comments/a8f32j/2018_day_13_help_needed/#t1_ecdqxrx
        #[test]
        fn test_extra_example() -> Result<(), MapError> {
            #[rustfmt::skip]
            let mut map: Map = [
                r"/-\  ",
                r"\>+-\",
                r"  \</",
            ]
            .join("\n")
            .parse()?;

            let position = map.run_until_collission(true, 10)?;

            assert_eq!(position, Position { x: 0, y: 1 });

            Ok(())
        }

        #[test]
        fn test_provided_example_1() -> Result<(), MapError> {
            let mut map: Map = [
                r"/->-\        ",
                r"|   |  /----\",
                r"| /-+--+-\  |",
                r"| | |  | v  |",
                r"\-+-/  \-+--/",
                r"  \------/   ",
            ]
            .join("\n")
            .parse()?;

            map.run()?;
            let expected: Map = [
                r"/-->\        ",
                r"|   |  /----\",
                r"| /-+--+-\  |",
                r"| | |  | |  |",
                r"\-+-/  \->--/",
                r"  \------/   ",
            ]
            .join("\n")
            .parse()?;
            assert_cart_positions_eq(&map, &expected);

            map.run()?;
            let expected: Map = [
                r"/---v        ",
                r"|   |  /----\",
                r"| /-+--+-\  |",
                r"| | |  | |  |",
                r"\-+-/  \-+>-/",
                r"  \------/   ",
            ]
            .join("\n")
            .parse()?;
            assert_cart_positions_eq(&map, &expected);

            let position = map.run_until_collission(false, 15)?;

            assert_eq!(position, Position { x: 7, y: 3 });

            Ok(())
        }

        #[test]
        fn test_provided_example_2() -> Result<(), MapError> {
            let mut map: Map = [
                r"/>-<\  ", r"|   |  ", r"| /<+-\", r"| | | v", r"\>+</ |", r"  |   ^", r"  \<->/",
            ]
            .join("\n")
            .parse()?;

            let position = map.run_until_last_cart(true, 10)?;

            assert_eq!(position, Position { x: 6, y: 4 });

            Ok(())
        }
    }

    mod test_check_collissions {
        use super::*;

        #[test]
        fn test_no_collissions() {
            let map = Map {
                tracks: vec![],
                carts: vec![
                    Cart::new(0, 0, Direction::North),
                    Cart::new(1, 0, Direction::North),
                    Cart::new(1, 1, Direction::South),
                ],
            };

            assert_eq!(map.check_collisions(&[]), None);
        }

        #[test]
        fn test_crash() {
            let map = Map {
                tracks: vec![],
                carts: vec![
                    Cart::new(0, 0, Direction::North),
                    Cart::new(1, 1, Direction::North),
                    Cart::new(1, 1, Direction::South),
                ],
            };

            let expected = (2, 1);
            assert_eq!(map.check_collisions(&[]), Some(expected));
        }
    }

    mod test_print {
        use super::*;

        #[test]
        fn test_correct_output() -> Result<(), MapError> {
            let map_string = [
                r"/-----\   ",
                r"|     |   ",
                r"^  /--+--\",
                r"|  |  |  |",
                r"\<-+--/  v",
                r"   |     |",
                r"   \--->-/",
            ]
            .join("\n");

            let map: Map = map_string.parse()?;

            assert_eq!(map.print(), map_string);

            Ok(())
        }
    }

    mod test_parse {
        use super::*;

        #[test]
        fn test_empty() -> Result<(), MapError> {
            let map: Map = "".parse()?;

            let expected_tracks: Vec<Vec<Track>> = Vec::new();

            assert_eq!(map.tracks, expected_tracks);
            assert_eq!(map.carts, vec![]);

            Ok(())
        }

        #[test]
        fn test_correct_output() -> Result<(), MapError> {
            let input = [
                r"/-----\   ",
                r"|     |   ",
                r"^  /--+--\",
                r"|  |  |  |",
                r"\<-+--/  v",
                r"   |     |",
                r"   \--->-/",
            ]
            .join("\n");

            let map: Map = input.parse()?;

            assert_eq!(
                map.carts,
                vec![
                    Cart::new(0, 2, Direction::North),
                    Cart::new(1, 4, Direction::West),
                    Cart::new(9, 4, Direction::South),
                    Cart::new(7, 6, Direction::East),
                ]
            );

            // it would be too cumbersome testing the entire
            // map, so instead we can just sample it
            assert_eq!(map.get_track(0, 0), Track::DiagonalRight);
            assert_eq!(map.get_track(1, 0), Track::Horizontal);
            assert_eq!(map.get_track(0, 1), Track::Vertical);
            assert_eq!(map.get_track(6, 0), Track::DiagonalLeft);
            assert_eq!(map.get_track(7, 0), Track::Empty);
            assert_eq!(map.get_track(6, 1), Track::Vertical);
            assert_eq!(map.get_track(6, 2), Track::Intersection);

            // Test the track values at cart locations
            assert_eq!(map.get_track(0, 2), Track::Vertical);
            assert_eq!(map.get_track(1, 4), Track::Horizontal);
            assert_eq!(map.get_track(9, 4), Track::Vertical);
            assert_eq!(map.get_track(7, 6), Track::Horizontal);

            Ok(())
        }
    }
}
