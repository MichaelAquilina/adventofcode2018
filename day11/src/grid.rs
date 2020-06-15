pub struct Grid {
    pub data: Vec<Vec<i32>>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

fn hundreth(value: i32) -> i32 {
    if value < 100 {
        0
    } else {
        (value / 100) % 10
    }
}

impl Grid {
    pub fn generate(width: u32, height: u32, serial_number: i32) -> Grid {
        let mut data = vec![vec![0; width as usize]; height as usize];

        for x in 0..width {
            for y in 0..height {
                let rack_id: i32 = (x + 10) as i32;
                let mut power_level: i32 = rack_id * y as i32;
                power_level += serial_number;
                power_level *= rack_id;
                power_level = hundreth(power_level) - 5;

                data[y as usize][x as usize] = power_level;
            }
        }

        Grid {
            data,
            width,
            height,
        }
    }

    fn calculate_power(&self, point: &Point, size: u32) -> i32 {
        let mut power = 0;
        for ix in 0..size {
            for iy in 0..size {
                power += self.data[(point.y + iy) as usize][(point.x + ix) as usize];
            }
        }
        power
    }

    pub fn find_max_power_point(&self) -> Option<Point> {
        let mut max_point = None;
        let mut max_power = 0;
        let size = 3;

        for x in 0..self.width - size {
            for y in 0..self.height - size {
                let point = Point { x, y };
                let power = self.calculate_power(&point, size);

                if power > max_power {
                    max_power = power;
                    max_point = Some(Point { x, y });
                }
            }
        }

        max_point
    }
}

#[cfg(test)]
mod test_hundredth {
    use super::*;
    use rstest::*;

    #[rstest(value, output, case(12345, 3), case(10, 0), case(99899, 8))]
    fn test_examples(value: i32, output: i32) {
        let result = hundreth(value);
        assert_eq!(result, output);
    }
}

#[cfg(test)]
mod test_generate {
    use super::*;
    use rstest::*;

    #[rstest(x, y, serial_number, power_level,
        case(3, 5, 8, 4),
        case(122, 79, 57, -5),
        case(217, 196, 39, 0),
        case(101, 153, 71, 4),
    )]
    fn test_provided_examples(x: u32, y: u32, serial_number: i32, power_level: i32) {
        let grid = Grid::generate(300, 300, serial_number);

        assert_eq!(grid.data[y as usize][x as usize], power_level);
    }
}

#[cfg(test)]
mod test_find_max_power_point {
    use super::*;

    #[test]
    fn test_provided_example() {
        let grid = Grid::generate(300, 300, 18);
        let point = grid.find_max_power_point();

        let expected = Some(Point { x: 33, y: 45 });

        assert_eq!(point, expected);
    }
}
