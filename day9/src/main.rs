// https://adventofcode.com/2018/day/9

mod config;
mod game;

use config::Config;
use game::Game;

use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let config: Config = contents.parse()?;

    let mut game = Game::new(config);
    game.play_all();
    println!("{:?}", game.highest_score());

    let mut config: Config = contents.parse()?;
    config.max_points *= 100;

    let mut game = Game::new(config);
    game.play_all();
    println!("{:?}", game.highest_score());

    Ok(())
}
