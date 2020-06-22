use std::cmp::Ordering;

use crate::track::Track;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cart {
    pub position: Position,
    pub direction: Direction,
    pub turn_count: u32,
}

impl Cart {
    pub fn new(x: usize, y: usize, direction: Direction) -> Cart {
        Cart {
            position: Position { x, y },
            direction,
            turn_count: 0,
        }
    }

    pub fn to_char(&self) -> char {
        match &self.direction {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::West => '<',
            Direction::East => '>',
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.y.partial_cmp(&other.y) {
            Some(Ordering::Equal) => self.x.partial_cmp(&other.x),
            ordering => ordering,
        }
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            ordering => ordering,
        }
    }
}

pub fn turn(cart: &Cart, track: Track) -> (Direction, u32) {
    let mut turn_count = cart.turn_count;
    let direction = match (cart.direction, track) {
        (Direction::North, Track::DiagonalRight) => turn_right(cart.direction),
        (Direction::North, Track::DiagonalLeft) => turn_left(cart.direction),
        (Direction::South, Track::DiagonalRight) => turn_right(cart.direction),
        (Direction::South, Track::DiagonalLeft) => turn_left(cart.direction),
        (Direction::West, Track::DiagonalLeft) => turn_right(cart.direction),
        (Direction::West, Track::DiagonalRight) => turn_left(cart.direction),
        (Direction::East, Track::DiagonalLeft) => turn_right(cart.direction),
        (Direction::East, Track::DiagonalRight) => turn_left(cart.direction),
        (direction, Track::Intersection) => {
            turn_count += 1;
            match cart.turn_count % 3 {
                0 => turn_left(direction),
                1 => direction,
                2 => turn_right(direction),
                _ => panic!("Unreachable"),
            }
        }
        _ => cart.direction,
    };

    (direction, turn_count)
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn turn_left(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
        Direction::East => Direction::North,
    }
}

#[cfg(test)]
mod test_position {
    use super::*;

    #[test]
    fn test_ordering() {
        let mut positions = vec![
            Position { y: 1, x: 2 },
            Position { y: 4, x: 1 },
            Position { y: 0, x: 3 },
            Position { y: 1, x: 5 },
        ];

        positions.sort();

        let expected = vec![
            Position { y: 0, x: 3 },
            Position { y: 1, x: 2 },
            Position { y: 1, x: 5 },
            Position { y: 4, x: 1 },
        ];

        assert_eq!(positions, expected);
    }

    #[test]
    fn test_sorting_carts() {
        let mut carts = vec![
            Cart::new(2, 1, Direction::North),
            Cart::new(1, 4, Direction::North),
            Cart::new(3, 0, Direction::North),
            Cart::new(5, 1, Direction::North),
        ];

        carts.sort_by_cached_key(|c| c.position.clone());

        let expected = vec![
            Cart::new(3, 0, Direction::North),
            Cart::new(2, 1, Direction::North),
            Cart::new(5, 1, Direction::North),
            Cart::new(1, 4, Direction::North),
        ];

        assert_eq!(carts, expected);
    }
}
