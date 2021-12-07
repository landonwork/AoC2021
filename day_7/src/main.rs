use std::fs;

fn main() {
    let crabs: Vec<usize> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    println!("{}",crabs.len());

    let min: usize = *crabs.iter().min().unwrap();
    let max: usize = *crabs.iter().max().unwrap();
    let solution1: usize = (min..=max)
        .map(|n| total_distance(&crabs,n))
        .min()
        .unwrap();
    println!("Part 1: {}", solution1);

    let solution2: usize = (min..=max)
        .map(|n| triangular_distance(&crabs,n))
        .min()
        .unwrap();
    println!("Part 2: {}", solution2);
}

fn total_distance(v: &Vec<usize>, n: usize) -> usize {
    let mut dist: usize = 0;
    for pos in v.iter() {
        let d: usize = (*pos as i32 - n as i32).abs() as usize;
        dist += d;
    }
    dist
}

fn triangular_distance(v: &Vec<usize>, n: usize) -> usize {
    let mut dist: usize = 0;
    for pos in v.iter() {
        let tri: usize = (*pos as i32 - n as i32).abs() as usize;
        let d: usize = (tri * (tri + 1)) / 2;
        dist += d;
    }
    dist
}
