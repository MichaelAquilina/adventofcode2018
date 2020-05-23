// https://adventofcode.com/2018/day/3
use std::error::Error;
use std::io::{self, Read};

mod rect;

use rect::Rect;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    for line in input.lines() {
        let rect: Rect = line.parse()?;
    }

    Ok(())
}
