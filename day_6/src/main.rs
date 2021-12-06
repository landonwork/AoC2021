use std::fs;

fn main() {
    let mut fishes: Vec<usize> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.to_string().parse::<usize>().unwrap())
        .collect();

    for _ in 0..80 {
        fishes = step(fishes);
    }
    println!("Part 1: {}", fishes.len());

///////////////////////////////////////////////

    fishes = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.to_string().parse::<usize>().unwrap())
        .collect();

    let mut count: [u128; 9] = [0; 9];
    for f in fishes {
        count[f] += 1;
    }

    for _ in 0..256 {
        count = step2(count);
    }
    println!("Part 2: {}", count.iter().sum::<u128>());
}

fn step(v: Vec<usize>) -> Vec<usize> {
    let mut n: usize = 0;
    let mut new: Vec<usize> = v
        .iter()
        .map(|x| if *x == 0 {
            n += 1;
            return 6;
        } else {
            return x - 1;
        }).collect();
    for _ in 0..n {new.push(8);}
    new
}

fn step2(v: [u128; 9]) -> [u128; 9] {
    let mut new: [u128; 9] = [0; 9];
    new[8] = v[0];
    new[6] = v[0];
    for i in 0..8 {
        new[i] += v[i + 1];
    }
    new
}
