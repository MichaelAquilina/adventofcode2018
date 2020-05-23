// https://adventofcode.com/2018/day/3
use std::error::Error;
use std::collections::HashMap;
use std::io::{self, Read};

mod rect;

use rect::Rect;

type Coord = (i32, i32);

fn add_rect_to_map(rect: &Rect, map: &mut HashMap<Coord, i32>) {
    for x in 0..rect.width {
        for y in 0..rect.height {
            let key = (rect.x + x, rect.y + y);
            let value = map.entry(key).or_insert(0);
            *value += 1;
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut map: HashMap<Coord, i32> = HashMap::new();

    for line in input.lines() {
        let rect: Rect = line.parse()?;

        add_rect_to_map(&rect, &mut map);
    }

    let collisions = map.values().filter(|&v| v > &1).count();

    println!("{}", collisions);

    Ok(())
}
