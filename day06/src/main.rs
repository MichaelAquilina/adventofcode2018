mod coord;

use coord::Coord;
use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

// get the bottom right corners of the map
fn get_bottom_right(coordinates: &[Coord]) -> Coord {
    let mut result = Coord { x: 0, y: 0 };

    for coord in coordinates {
        result.x = max(coord.x, result.x);
        result.y = max(coord.y, result.y);
    }
    result
}

type CoordMap = HashMap<Coord, Option<i32>>;

fn get_map(coordinates: &[Coord], bottom_right: &Coord) -> CoordMap {
    // coordinate -> closest point (by index)
    let mut coord_map = HashMap::new();

    for x in 0..=bottom_right.x {
        for y in 0..=bottom_right.y {
            let current = Coord { x, y };
            let mut min = i32::MAX;
            let mut closest = None;

            for (index, coord) in coordinates.iter().enumerate() {
                let distance = coord.distance_from(&current);
                if distance < min {
                    min = distance;
                    closest = Some(index as i32);
                } else if distance == min {
                    closest = None;
                }
            }
            coord_map.insert(current, closest);
        }
    }

    coord_map
}

fn touches_edge(coord: &Coord, bottom_right: &Coord) -> bool {
    coord.x == 0 || coord.y == 0 || coord.x == bottom_right.x || coord.y == bottom_right.y
}

// get the largest area on the map that is *not* infinite
// infinite => area touches a border on the map
fn get_largest_area(coord_map: &CoordMap, bottom_right: &Coord) -> Option<i32> {
    let mut area: HashMap<i32, i32> = HashMap::new();
    let mut edges = vec![];

    for (coord, index) in coord_map.iter() {
        if let Some(index) = index {
            let count = area.entry(*index).or_insert(0);
            *count += 1;

            if touches_edge(&coord, &bottom_right) {
                edges.push(index);
            }
        }
    }

    area.into_iter()
        .filter(|(k, _)| !edges.contains(&k))
        .map(|(_, v)| v)
        .max()
}

fn solution1(coordinates: &[Coord], bottom_right: &Coord) -> Option<i32> {
    let coord_map = get_map(&coordinates, &bottom_right);
    get_largest_area(&coord_map, &bottom_right)
}

fn solution2(coordinates: &[Coord], bottom_right: &Coord, max_distance: i32) -> i32 {
    let mut area = 0;
    for x in 0..=bottom_right.x {
        for y in 0..=bottom_right.y {
            let point = Coord { x, y };
            let mut total_distance = 0;
            for coord in coordinates {
                total_distance += coord.distance_from(&point);
            }

            if total_distance < max_distance {
                area += 1;
            }
        }
    }

    area
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;

    let mut coordinates: Vec<Coord> = vec![];
    for line in contents.lines() {
        coordinates.push(line.parse()?);
    }

    let bottom_right = get_bottom_right(&coordinates);

    let result = solution1(&coordinates, &bottom_right);
    println!("{:?}", result);

    let result = solution2(&coordinates, &bottom_right, 10000);
    println!("{:?}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use coord::CoordError;

    fn get_example() -> Result<Vec<Coord>, CoordError> {
        Ok(vec![
            "1, 1".parse()?,
            "1, 6".parse()?,
            "8, 3".parse()?,
            "3, 4".parse()?,
            "5, 5".parse()?,
            "8, 9".parse()?,
        ])
    }

    #[test]
    fn test_solution1() -> Result<(), CoordError> {
        let coordinates = get_example()?;

        // Hard coded from the provided example
        let bottom_right = Coord { x: 9, y: 9 };
        let result = solution1(&coordinates, &bottom_right);

        assert_eq!(result, Some(17));

        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), CoordError> {
        let coordinates = get_example()?;

        let bottom_right = Coord { x: 9, y: 9 };
        let result = solution2(&coordinates, &bottom_right, 32);

        assert_eq!(result, 16);
        Ok(())
    }

    #[test]
    fn test_get_map() -> Result<(), CoordError> {
        let coordinates = get_example()?;

        // Hard coded from the provided example
        let bottom_right = Coord { x: 9, y: 9 };

        let map = get_map(&coordinates, &bottom_right);

        // its cumbersome and hard to read comparing a map
        // result with 81 entries.
        // Instead we assert by sampling some points

        assert_eq!(map[&"0,0".parse()?], Some(0));
        assert_eq!(map[&"6,0".parse()?], Some(2));
        assert_eq!(map[&"0,1".parse()?], Some(0));
        assert_eq!(map[&"2,4".parse()?], Some(3));
        assert_eq!(map[&"5,4".parse()?], Some(4));
        assert_eq!(map[&"0,9".parse()?], Some(1));
        assert_eq!(map[&"9,9".parse()?], Some(5));

        assert_eq!(map[&"5,0".parse()?], None);
        assert_eq!(map[&"5,1".parse()?], None);
        assert_eq!(map[&"0,4".parse()?], None);
        assert_eq!(map[&"1,4".parse()?], None);
        assert_eq!(map[&"9,6".parse()?], None);
        assert_eq!(map[&"8,6".parse()?], None);

        Ok(())
    }
}
