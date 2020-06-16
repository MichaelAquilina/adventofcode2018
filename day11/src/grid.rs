pub struct Grid {
    pub data: Vec<Vec<i32>>,
    pub summed_area: Vec<Vec<i32>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

fn hundreth(value: i32) -> i32 {
    if value < 100 {
        0
    } else {
        (value / 100) % 10
    }
}

impl Grid {
    pub fn generate(width: usize, height: usize, serial_number: i32) -> Grid {
        let mut data = vec![vec![0; width]; height];
        let mut summed_area = vec![vec![0; width]; height];

        for x in 0..width {
            for y in 0..height {
                let rack_id = (x + 10) as i32;
                let mut power_level = rack_id * y as i32;
                power_level += serial_number;
                power_level *= rack_id;
                power_level = hundreth(power_level) - 5;

                data[y][x] = power_level;

                summed_area[y][x] += data[y][x];

                if y > 0 {
                    summed_area[y][x] += summed_area[y - 1][x];
                }
                if x > 0 {
                    summed_area[y][x] += summed_area[y][x - 1];
                }
                if x > 0 && y > 0 {
                    summed_area[y][x] -= summed_area[y - 1][x - 1];
                }
            }
        }

        Grid {
            data,
            summed_area,
            width,
            height,
        }
    }

    fn calculate_power(&self, point: &Point, size: usize) -> i32 {
        // Calculates power using the summed area data structure
        // https://en.wikipedia.org/wiki/Summed-area_table
        let mut power = 0;

        power += self.summed_area[point.y + size - 1][point.x + size - 1];
        if point.x > 0 && point.y > 0 {
            power += self.summed_area[point.y - 1][point.x - 1];
        }
        if point.y > 0 {
            power -= self.summed_area[point.y - 1][point.x + size - 1];
        }
        if point.x > 0 {
            power -= self.summed_area[point.y + size - 1][point.x - 1];
        }
        power
    }

    pub fn find_max_power_point(&self) -> (Option<Point>, i32) {
        let mut max_point = None;
        let mut max_power = i32::MIN;
        let size = 3;

        for x in 0..self.width - size {
            for y in 0..self.height - size {
                let point = Point { x, y };
                let power = self.calculate_power(&point, size);

                if power > max_power {
                    max_power = power;
                    max_point = Some(point);
                }
            }
        }

        (max_point, max_power)
    }

    pub fn find_max_power_point_adjustable(&self) -> (Option<(Point, usize)>, i32) {
        let mut max_point = None;
        let mut max_power = i32::MIN;

        for x in 0..self.width {
            for y in 0..self.height {
                let max_size = std::cmp::min(self.width - x, self.height - y);
                for size in 1..max_size {
                    let point = Point { x, y };
                    let power = self.calculate_power(&point, size);

                    if power > max_power {
                        max_power = power;
                        max_point = Some((point, size));
                    }
                }
            }
        }

        (max_point, max_power)
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
mod test_grid {
    use super::*;
    use rstest::*;

    #[test]
    fn test_summed_area() {
        let grid = Grid::generate(4, 4, 2);

        let expected_data = vec![
            vec![-5, -5, -5, -5],
            vec![-4, -4, -4, -4],
            vec![-3, -3, -2, -2],
            vec![-2, -2, -1, 0],
        ];

        assert_eq!(grid.data, expected_data);

        let expected_summed_area = vec![
            vec![-5, -10, -15, -20],
            vec![-9, -18, -27, -36],
            vec![-12, -24, -35, -46],
            vec![-14, -28, -40, -51],
        ];

        assert_eq!(grid.summed_area, expected_summed_area);

        assert_eq!(grid.calculate_power(&Point { x: 0, y: 0 }, 1), -5);
        assert_eq!(grid.calculate_power(&Point { x: 1, y: 1 }, 1), -4);
        assert_eq!(grid.calculate_power(&Point { x: 1, y: 1 }, 2), -13);
        assert_eq!(grid.calculate_power(&Point { x: 1, y: 1 }, 3), -22);

        let (point, _) = grid.find_max_power_point();
        assert_eq!(point, Some(Point { x: 0, y: 0 }));
    }

    #[rstest(x, y, serial_number, power_level,
        case(3, 5, 8, 4),
        case(122, 79, 57, -5),
        case(217, 196, 39, 0),
        case(101, 153, 71, 4),
    )]
    fn test_generate(x: usize, y: usize, serial_number: i32, power_level: i32) {
        let grid = Grid::generate(300, 300, serial_number);

        assert_eq!(grid.data[y][x], power_level);
    }

    #[rstest(serial_number, expected_point, expected_power,
        case(18, Point { x: 33, y: 45}, 29),
        case(42, Point { x: 21, y: 61}, 30),
    )]
    fn test_find_max_power_point(serial_number: i32, expected_point: Point, expected_power: i32) {
        let grid = Grid::generate(300, 300, serial_number);
        let (point, power) = grid.find_max_power_point();

        assert_eq!(power, expected_power);
        assert_eq!(point, Some(expected_point));
    }

    #[rstest(serial_number, expected_point, expected_size, expected_power,
        case(18, Point { x: 90, y: 269}, 16, 113),
        case(42, Point { x: 232, y: 251}, 12, 119),
    )]
    fn test_find_max_power_point_adjustable(
        serial_number: i32,
        expected_point: Point,
        expected_size: usize,
        expected_power: i32,
    ) {
        let grid = Grid::generate(300, 300, serial_number);
        let (point, power) = grid.find_max_power_point_adjustable();

        assert_eq!(power, expected_power);
        assert_eq!(point, Some((expected_point, expected_size)));
    }
}
