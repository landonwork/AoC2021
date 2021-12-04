use std::fs;

fn main() {
    let input = read_input("input.txt");
    let instructions = input.trim().split('\n');
    let solution1 = solve1(instructions.clone());
    println!("Part 1: {}", solution1);
    let solution2 = solve2(instructions);
    println!("Part 2: {}", solution2);
}

fn read_input(filename: &str) -> String {
    let result = fs::read_to_string(filename).expect("Something went wrong reading the file");
    result
}

fn solve1(instructions: std::str::Split<char>) -> String {
    let mut v: i32 = 0;
    let mut h: i32 = 0;
    for s in instructions {
        let len: usize = s.len().try_into().unwrap();
        let dir: &str = &s[..len - 1];
        // print!("{}",dir);
        let digit: &str = &s[len - 1..];
        match dir {
            "forward " => h += digit.parse::<i32>().unwrap(),
            "down " => v += digit.parse::<i32>().unwrap(),
            "up " => v -= digit.parse::<i32>().unwrap(),
            _ => print!("Error"),
        }
    }
    (v * h).to_string()
}

fn solve2(instructions: std::str::Split<char>) -> String {
    let mut v: i32 = 0;
    let mut h: i32 = 0;
    let mut aim: i32 = 0;
    for s in instructions {
        let len: usize = s.len().try_into().unwrap();
        let dir: &str = &s[..len - 1];
        let digit: &str = &s[len - 1..];
        drop(len);

        match dir {
            "forward " => {
                let d = digit.parse::<i32>().unwrap();
                h += d;
                v += aim * d;
            }
            "down " => aim += digit.parse::<i32>().unwrap(),
            "up " => aim -= digit.parse::<i32>().unwrap(),
            _ => print!("Error"),
        }
    }
    (v * h).to_string()
}

// fn sum(arr: &[i32]) -> i32 {
//     let mut s = 0;
//     for n in arr { s += n; }
//     s
// }
