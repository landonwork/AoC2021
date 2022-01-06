use std::fs;
use std::collections::HashMap;

fn main() {
    let mut raw: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|x| x.to_string())
        .collect();

    let mut polymer: String = raw.remove(0);
    raw.remove(0);
    let mut rules = HashMap::new();
    for line in raw {
        rules.insert(
            line[0..2].to_string(),
            line[line.len()-1..].to_string()
        );
    }

    println!("Part 1: {}", part1(polymer.clone(), &rules));
    println!("Part 2: {}", part2(&polymer, &rules));
}

fn part1(mut polymer: String, rules: &HashMap<String, String>) -> usize {
    for step in 0..10 {
        let mut insertions: String = String::with_capacity(polymer.len()-1);
        let iter = (0..polymer.len()-1).map(|i| polymer[i..i+2].to_string());
        let _a = iter.map(|x| insertions.push_str(rules[&x].as_str())).collect::<()>();
        let old = polymer.clone();

        polymer = String::with_capacity(polymer.len()*2-1);
        for i in 0..old.len() {
            polymer.push_str(&old[i..i+1]);
            if i != old.len()-1 {
                polymer.push_str(&insertions[i..i+1]);
            }
        }
    }
    
    let mut count: HashMap<char, usize> = HashMap::new();
    for c in polymer.chars() {
        if let Some(x) = count.get_mut(&c) {
            *x += 1;
        } else { count.insert(c, 1); }
    }
    
    count.values().max().unwrap() - count.values().min().unwrap()
}

fn part2(polymer: &String, rules: &HashMap<String, String>) -> usize {
    let mut pair_count: HashMap<String, usize> = HashMap::new();

    let iter = (0..polymer.len()-1).map(|i| polymer[i..i+2].to_string());
    for s in iter {
        let x = pair_count.entry(s).or_insert(0);
        *x += 1;
    }

    for _step in 0..40 {
        let mut new_count: HashMap<String, usize> = HashMap::new();
        for (k, v) in pair_count.drain() {
            let mut k1 = k[..1].to_string();
            let mut k2 = rules.get(&k).unwrap().clone();
            k1.push_str(rules.get(&k).unwrap());
            k2.push_str(&k[1..]);

            let x = new_count.entry(k1).or_insert(0);
            *x += v;
            let y = new_count.entry(k2).or_insert(0);
            *y += v;
        }
        pair_count = new_count
    }

    let mut count: HashMap<char, usize> = HashMap::with_capacity(26);
    for (k, v) in pair_count.drain() {
        let chars: Vec<char> = k.chars().collect();
        let k1 = chars[0];
        let k2 = chars[1];

        let a = count.entry(k1).or_insert(0);
        *a += v; // I usually prefer to assign both then change both BUT
        let b = count.entry(k2).or_insert(0);
        *b += v; // that would require two simultaneous mutable borrows
    }

    // I got lucky here and I didn't have to account for the possibility
    // of one of the letters being on the end of beginning of the polymer
    (count.values().max().unwrap() - count.values().min().unwrap()) / 2
}
