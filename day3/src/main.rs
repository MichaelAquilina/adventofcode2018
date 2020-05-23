// https://adventofcode.com/2018/day/3
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};

mod rect;

use rect::Rect;

type Coord = (i32, i32);

fn does_not_collide(rect: &Rect, map: &HashMap<Coord, i32>) -> bool {
    for x in 0..rect.width {
        for y in 0..rect.height {
            let key = (rect.x + x, rect.y + y);
            if let Some(n) = map.get(&key) {
                if *n > 1 {
                    return false;
                }
            }
        }
    }
    true
}

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
    let mut rects = vec![];

    for line in input.lines() {
        let rect: Rect = line.parse()?;

        add_rect_to_map(&rect, &mut map);
        rects.push(rect);
    }

    let collisions = map.values().filter(|&v| v > &1).count();

    // Result for part 1
    println!("{}", collisions);

    // Result for part 2
    for rect in rects {
        if does_not_collide(&rect, &map) {
            println!("{} does not collide with any other rect", rect.id);
            break;
        }
    }

    Ok(())
}
