use std::fs;
use std::collections::HashMap;

fn main() {
    let mut raw: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|x| x.to_string())
        .collect();
    // println!("{:?}", raw);

    let mut polymer: String = raw.remove(0);
    raw.remove(0);
    let mut rules = HashMap::new();
    for line in raw {
        rules.insert(
            line[0..2].to_string(),
            line[line.len()-1..].to_string()
            );
    }
    // println!("{:?}", rules["PS"]);

    for step in 0..10 {
        println!("{}", step);
        let mut insertions: String = String::with_capacity(polymer.len()-1);
        let iter = (0..polymer.len()-1).map(|i| polymer[i..i+2].to_string());
        let _a = iter.map(|x| insertions.push_str(rules[&x].as_str())).collect::<()>();
        // println!("{}", &insertions);
        let old = polymer.clone();
        polymer = String::with_capacity(polymer.len()*2-1);
        for i in 0..old.len() {
            // println!("{:?} {:?}", polymer, i);
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
    
    println!("Part 1: {}", count.values().max().unwrap() - count.values().min().unwrap());
    // println!("{}", polymer);
}
