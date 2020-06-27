// https://adventofcode.com/2018/day/15

mod game;
mod point;
mod race;
mod unit;

use game::Game;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let mut game: Game = contents.parse()?;
    let rounds = game.play(u64::MAX);
    let total_hp = game.total_hp_remaining();

    // "You need to determine the outcome of the battle:
    // the number of full rounds that were completed (not counting
    // the round in which combat ends) multiplied by the sum of
    // the hit points of all remaining units at the moment combat ends"
    println!("Rounds: {}, Total HP: {}", rounds, total_hp);
    println!("Result: {}", rounds * total_hp as u64);

    Ok(())
}
