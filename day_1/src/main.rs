use std::fs;

fn main() {
    let input = read_input("input.txt");
    let split = input.trim().split('\n');
    let depths = split
        .into_iter()
        .map(|s| -> u32 { s.parse().unwrap() })
        .collect();
    let solution1 = solve1(&depths);
    println!("Part 1: {}", solution1);
    let solution2 = solve2(depths);
    println!("Part 2: {}", solution2);
}

fn read_input(filename: &str) -> String {
    let result = fs::read_to_string(filename).expect("Something went wrong reading the file");
    result
}

fn solve1(depths: &Vec<u32>) -> String {
    let mut v = depths.iter();
    let mut d = v.next().unwrap();
    let mut count = 0;
    loop {
        let n = v.next();
        match n {
            None => break,
            Some(_) => {
                let m = n.unwrap();
                if m > d {
                    count += 1;
                }
                d = m
            }
        }
    }
    count.to_string()
}

fn solve2(depths: Vec<u32>) -> String {
    let mut d = sum(&depths[0..3]);
    let mut count = 0;
    for i in 1..(depths.len() - 2) {
        let n = sum(&depths[i..(i + 3)]);
        if n > d {
            count += 1;
        }
        d = n
    }
    count.to_string()
}

fn sum(arr: &[u32]) -> u32 {
    let mut s = 0;
    for n in arr {
        s += n;
    }
    s
}
