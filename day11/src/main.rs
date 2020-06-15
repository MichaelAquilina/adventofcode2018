mod grid;

use grid::Grid;

fn main() {
    let serial_number = 7857;

    let grid = Grid::generate(300, 300, serial_number);
    let point = grid.find_max_power_point();

    println!("{:?}", point);
}
