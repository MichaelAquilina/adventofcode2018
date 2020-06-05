/// Solution for adventofcode day8
/// https://adventofcode.com/2018/day/8
mod node;

use node::Node;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let result: Node = contents.parse()?;

    println!("{}", result.metadata_sum());

    Ok(())
}
