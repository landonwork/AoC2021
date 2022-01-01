use std::fs;
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[macro_use]
extern crate lazy_static;

fn main() {
    let raw: String = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = raw.trim().split('\n').collect();
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines {
        let t: Vec<&str> = line.split('-').collect();
        add_to_map((t[0], t[1]), &mut map);
    }

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}

fn part1(map: &HashMap<&str, Vec<&str>>) -> usize {
    let mut n_paths = 0usize;
    find_paths(vec!["start"], map, &mut n_paths);
    n_paths
}

fn part2(map: &HashMap<&str, Vec<&str>>) -> usize {
    let mut n_paths = 0usize;
    find_paths2(vec!["start"], map, &mut n_paths);
    n_paths
}

fn find_paths(path: Vec<&str>, map: &HashMap<&str, Vec<&str>>, count: &mut usize) {
    for dest in map.get(&path[path.len()-1]).unwrap() {
        if dest == &"end" {
            *count += 1;
            continue;
        }
        if is_small(dest) && path.contains(dest) {
            continue;
        }
        let mut new_path = path.clone();
        new_path.push(dest);
        find_paths(new_path, map, count);
    }
}

fn find_paths2(path: Vec<&str>, map: &HashMap<&str, Vec<&str>>, count: &mut usize) {
    for dest in map.get(&path[path.len()-1]).unwrap() {
        if dest == &"end" {
            *count += 1;
            continue;
        }
        if dest == &"start" {
            continue;
        }
        if has_two_small(&path) {
            if is_small(dest) && path.contains(dest) {
                continue;
            }
        }
        let mut new_path = path.clone();
        new_path.push(dest);
        find_paths2(new_path, map, count);
    }
}

fn add_to_map<'a, 'b>((a, b): (&'a str, &'a str), map: &'b mut HashMap<&'a str, Vec<&'a str>>) -> () {
    if map.contains_key(&a) {
        map.get_mut(&a).unwrap().push(b);
    } else {
        map.insert(a, vec![b]);
    }
    if map.contains_key(&b) {
        map.get_mut(&b).unwrap().push(a);
    } else {
        map.insert(b, vec![a]);
    }
}

fn is_small(cave: &str) -> bool {
    lazy_static! {
        static ref SMALL: Regex = Regex::new(r"^[a-z]+$").unwrap();
    }
    SMALL.is_match(cave)
}

fn has_two_small(path: &Vec<&str>) -> bool {
    let mut smalls: HashSet<&str> = HashSet::new();
    for &cave in path {
        if smalls.contains(&cave) {
            return true;
        } else if is_small(cave) {
            smalls.insert(cave);
        }
    }
    false
}

