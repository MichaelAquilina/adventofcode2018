use std::error::Error;
use std::io::{self, Read};

mod entry;

use entry::Entry;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    io::stdin().read_to_string(&mut contents)?;

    let contents = contents;
    let mut entries = vec![];

    for line in contents.lines() {
        let entry: Entry = line.parse()?;
        entries.push(entry);
    }

    entries.sort_unstable();

    for entry in entries {
        println!("{:?}", entry);
    }

    Ok(())
}
