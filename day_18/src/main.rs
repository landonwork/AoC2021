use std::io::Error;
use day_18::lib::SnailFish;
use std::time::Instant;

/// This is a docstring :)
fn main() -> Result<(), Error> {
    let start = Instant::now();
    let raw = std::fs::read_to_string("input.txt")?;
    let mut pairs = Vec::new();
    for line in raw.lines() {
        pairs.push(SnailFish::new(line));
    }

    println!("Part 1: {}", part1(pairs.clone()));
    println!("Part 2: {}", part2(pairs));
    println!("Time elapsed: {:?}", start.elapsed());

    Ok(())
}

fn part1(pairs: Vec<SnailFish>) -> i32 {
    pairs.into_iter().reduce(|left, right| left + right ).unwrap().magnitude()
}

fn part2(pairs: Vec<SnailFish>) -> i32 {

    let mut ans = 0;
    for i in 0..pairs.len() {
        for j in (i+1)..pairs.len() {
            let res = (pairs[i].clone() + pairs[j].clone()).magnitude();
            if res > ans {
                ans = res;
            }
        }
    }
    ans
}
