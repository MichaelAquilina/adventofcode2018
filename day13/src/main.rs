// https://adventofcode.com/2018/day/13

mod map;

use map::Map;

use anyhow::Result;
use std::io::Read;

fn main() -> Result<()> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let mut map: Map = contents.parse()?;

    println!("Track Size: {}x{}", map.tracks[0].len(), map.tracks.len());
    println!("Carts: {}", map.carts.len());

    println!("Running cart simulation");
    let position = map.run_until_collission(false, 400)?;
    println!("{:?}", position);

    println!("Continuing cart simulation");
    let position = map.run_until_last_cart(false, 100000)?;
    println!("{:?}", position);

    Ok(())
}
