use std::fs;

fn main() {
    let raw: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|x| x.to_string())
        .collect();

    let mut outputs: Vec<String> = Vec::new();
    for line in raw.iter() {
        let mut v: Vec<String> = line
            .split(" | ")
            .map(|x| x.to_string())
            .collect();
        outputs.push(v.remove(1));
    }
    
    let mut count: usize = 0;
    let is_1478 = |s: &str| {((s.len() == 2) | (s.len() == 3) | (s.len() == 4) | (s.len() == 7)) as usize};
    for out in outputs {
        count += out.split(' ').map(is_1478).sum::<usize>();
    }

    println!("Part 1: {}", count);
}
