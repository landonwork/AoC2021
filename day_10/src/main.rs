use std::fs;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref BRACKETS: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);
    
    static ref POINTS: HashMap<char, usize> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    static ref POINTS2: HashMap<char, usize> = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);

}

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    let lines = raw.trim().split('\n').collect();
    
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &Vec<&str>) -> usize {
    let mut pts = 0;
    for line in lines {
        let mut stack: Vec<char> = Vec::with_capacity(100);
        for c in line.chars() {
            if BRACKETS.contains_key(&c) {
                stack.push(c);
            } else if c == *BRACKETS.get(&stack[stack.len()-1]).unwrap() {
                stack.pop();
            } else {
                pts += POINTS.get(&c).unwrap();
                break;
            }
        }
    }
    pts
}

fn is_corrupted(line: &str) -> bool {
   let mut stack: Vec<char> = Vec::with_capacity(100);
   for c in line.chars() {
       if BRACKETS.contains_key(&c) {
           stack.push(c);
       } else if c == *BRACKETS.get(&stack[stack.len()-1]).unwrap() {
           stack.pop();
       } else {
           return true;
       }
   }
   false
}

fn complete_line(line: &str) -> usize {
   let mut stack: Vec<char> = Vec::with_capacity(100);
   let mut pts: usize = 0;
   for c in line.chars() {
       if BRACKETS.contains_key(&c) {
           stack.push(c);
       } else {
           stack.pop();
       }
   }
   loop {
        if stack.len() == 0 { break; }
        pts = pts * 5 + POINTS2.get(&BRACKETS.get(&stack.pop().unwrap()).unwrap()).unwrap();
   }
   pts
}

fn part2(lines: &Vec<&str>) -> usize {
    let mut scores: Vec<usize> = Vec::new();
    for line in lines {
        if !is_corrupted(line) {
            scores.push(complete_line(line));
        }
    }
    scores.sort();
    scores[scores.len()/2]
}

