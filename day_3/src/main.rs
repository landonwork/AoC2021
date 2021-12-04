use std::fs;
use std::cmp::Ordering;

// I've made a big mess
fn main() {
    let input = read_input("input.txt");
    let codes: Vec<&str> = input.trim().split('\n').into_iter().collect();
    let solution1 = solve1(codes.clone());
    let mut a: Vec<i32> = vec![1,2,3];
    a.retain(|x| *x == 1);
    println!("{:?}",a);
    println!("Part 1: {}", solution1);
    let solution2 = solve2(codes);
    println!("Part 2: {}", solution2);
}

fn read_input(filename: &str) -> String {
    let result = fs::read_to_string(filename).expect("Something went wrong reading the file");
    result
}

fn most_common(codes: &Vec<&str>) -> Vec<u64> {
    let mut total: i32 = 0;
    let mut codes = codes.into_iter();
    let mut code: Option<&&str> = codes.next();
    let mut count: Vec<i32> = vec![0; code.unwrap().len()];

    loop {
        match code {
            None => break,
            Some(s) => {
                let mut i = 0;
                for c in s.chars() {
                    count[i] += c.to_string().parse::<i32>().unwrap();
                    i += 1;
                }
                total += 1;
                code = codes.next();
            }
        }
    }
    
    let most_common  = |x: &i32| (((*x as f32) / (total as f32)) > 0.5) as u64;
    count.iter().map(most_common).collect()
}

fn solve1(codes: Vec<&str>) -> String {
    let gamma2: Vec<u64>   = most_common(&codes);
    let epsilon2: Vec<u64> = gamma2.iter().map(|x| 1 - x).collect();

    let gamma: u64 = gamma2.iter().fold(0, |acc: u64, x: &u64| acc*2 + x);
    let epsilon: u64 = epsilon2.iter().fold(0, |acc: u64, x: &u64| acc*2 + x);
    
    return (gamma * epsilon).to_string();
}

fn solve2(codes: Vec<&str>) -> String {

    // Magic function for O2
    let mut v = codes.clone();
    let mut i = 0;
    while (v.len() != 1) & (i < codes[0].len()) {
        v = magic(v, i, true);
        i += 1;
    }
    println!("{:?}", v);
    // Magic function for CO2

    "hello".to_string()
}

fn magic(mut v: Vec<&str>, ind: usize, greater: bool) -> Vec<&str> {
    let most_common: Vec<u64> = most_common(&v);
    let mut keep: Vec<bool> = vec![true; v.len()];
    for i in 0..v.len() {
        let n = v[i][ind..ind + 1].to_string().parse::<u64>().unwrap();
        match n.cmp(&most_common[ind]) {
            Ordering::Greater => {
                if !greater {keep[i] = false;}
            },
            Ordering::Equal => (),
            Ordering::Less => {
                if greater {keep[i] = false;}
            },
        }
    }
    v.retain(|_x| keep.pop().unwrap());
    v
}
