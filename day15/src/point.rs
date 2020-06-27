use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            ordering => ordering,
        }
    }
}

#[cfg(test)]
mod test_point {
    use super::*;
    use rstest::*;

    #[rstest(point1, point2, expected,
        case(Point { x: 1, y: 1}, Point { x: 1, y: 1 }, Ordering::Equal),
        case(Point { x: 1, y: 2}, Point { x: 1, y: 1 }, Ordering::Greater),
        case(Point { x: 2, y: 0}, Point { x: 4, y: 1 }, Ordering::Less),
        case(Point { x: 2, y: 3}, Point { x: 4, y: 3 }, Ordering::Less),
        case(Point { x: 9, y: 3}, Point { x: 7, y: 3 }, Ordering::Greater),
    )]
    fn test_ordering(point1: Point, point2: Point, expected: Ordering) {
        assert_eq!(point1.cmp(&point2), expected);
    }
}
