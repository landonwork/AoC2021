use std::io::Error;
use day_18::lib::SnailFish;
use day_18::lib::SnailFish::{Number, Pair};

/// This is a docstring :)
fn main() -> Result<(), Error> {
    let raw = std::fs::read_to_string("input.txt")?;
    let mut pairs = Vec::new();
    for line in raw.lines() {
        pairs.push(SnailFish::new(line));
    }

    println!("{:?}", pairs[0]);

    Ok(())
}
