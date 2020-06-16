// https://adventofcode.com/2018/day/11

mod grid;

use grid::Grid;

fn main() {
    let serial_number = 7857;

    let grid = Grid::generate(300, 300, serial_number);
    let (point, power) = grid.find_max_power_point();

    println!("{:?} (power: {})", point, power);

    let (point, power) = grid.find_max_power_point_adjustable();

    println!("{:?} (power: {})", point, power);
}
