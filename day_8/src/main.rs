use std::fs;
use std::collections::{HashMap, HashSet};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DIGITS: HashMap<usize, HashSet<char>> = HashMap::from([
        (0, HashSet::from(['a','b','c','e','f','g'])),
        (1, HashSet::from(['c','f'])),
        (2, HashSet::from(['a','c','d','e','g'])),
        (3, HashSet::from(['a','c','d','f','g'])),
        (4, HashSet::from(['b','c','d','f'])),
        (5, HashSet::from(['a','b','d','f','g'])),
        (6, HashSet::from(['a','b','d','e','f','g'])),
        (7, HashSet::from(['a','c','f'])),
        (8, HashSet::from(['a','b','c','d','e','f','g'])),
        (9, HashSet::from(['a','b','c','d','f','g'])),
    ]);
}

mod tests {
    use super::*;

    #[test]
    fn is_like() {
        let cands: HashMap<char, char> = HashMap::from([
            ('c','a'),
            ('f','b'),
        ]);
        let scram = "ab";

        assert_eq!(like_digit(scram, &cands, 1), true);
    }

    #[test]
    fn is_not_like() {
        let cands: HashMap<char, char> = HashMap::from([
            ('c','a'),
            ('f','b'),
        ]);
        let scram = "ae";

        assert_eq!(like_digit(scram, &cands, 1), false);
    }

    #[test]
    fn is_mostly_like() {
        let cands: HashMap<char, char> = HashMap::from([
            ('c','a'),
            ('f','b'),
        ]);
        let scram = "abc";
        
        assert_eq!(like_digit(scram, &cands, 7), true);
    }

    #[test]
    fn are_like() {
        let s = "egfdac gceb bcf cb gacefb gbcfa gacfe ecbafd efgbacd agbfd";
        assert_eq!(like_digits(s, &HashMap::new()), true);
    }

    #[test]
    fn can_unscramble() {
        let s = "egfdac gceb bcf cb gacefb gbcfa gacfe ecbafd efgbacd agbfd";
        let res = unscramble(s, HashMap::new(), 'a');
        match res {
            Ok(_set) => assert!(true),
            Err(_c) => assert!(false),
        }
    }

    #[test]
    fn can_decode() {
        let s = "fceabd decba debgf acgefb cfedb ecf fdgaceb acfd fc gdbaec | cedbag cdeabf fdca bgadec";
        let key: Vec<&str> = s.split(" | ").collect();
        let cands = unscramble(key[0], HashMap::new(), 'a').unwrap();
        let score = decode(key[1], &cands);
        println!("{}", score);
    }
}

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

    // Let's do a recursive search.
    println!("Part 2: {}", part2(&raw));
}

fn part2(lines: &Vec<String>) -> usize {
    let mut total = 0usize;
    for line in lines {
        let parts: Vec<&str> = line.split(" | ").collect();
        let key = unscramble(parts[0], HashMap::new(), 'a').unwrap();
        total +=  decode(parts[1], &key);
    }
    total
}

fn like_digit(scrambled: &str, candidates: &HashMap<char, char>, n: usize) -> bool {
    let d = DIGITS.get(&n).unwrap();
    if !(scrambled.len() == d.len()) {
        return false;
    }
    for c in 'a'..='g' {
        if let Some(cand) = candidates.get(&c) {
            if (scrambled.chars().find(|ch| ch == cand) != None) ^ d.contains(&c) {
                return false;
            }
        }
    }
    true
}

fn like_digits(scrambled: &str, candidates: &HashMap<char, char>) -> bool {
    (0..10).fold(
        true, |acc, x| acc & scrambled.split(' ').fold(
            false, |acc2, y| acc2 | like_digit(y, candidates, x)
        )
    )
}

fn unscramble(scrambled: &str, mut candidates: HashMap<char, char>,
              letter: char) -> Result<HashMap<char, char>, HashMap<char, char>> {
    for c in 'a'..='g' {
        if candidates.values().find(|val| *val == &c) != None { continue; }
        candidates.insert(letter, c);
        if like_digits(scrambled, &candidates) {
            if letter == 'g' {
                return Ok(candidates)
            }
            let next_letter = ((letter as u8) + 1) as char;
            let res = unscramble(scrambled, candidates, next_letter);
            match res {
                Ok(set) => {
                    return Ok(set);
                },
                Err(cands) => {
                    candidates = cands;
                    continue;
                }
            }
        }
    }
    candidates.remove(&letter);
    Err(candidates)
}

fn decode(scrambled: &str, candidates: &HashMap<char, char>) -> usize {
    scrambled.trim().split(' ').fold(
        0, |acc, s| 10 * acc + (0..10).find(
            |x| like_digit(s, candidates, *x)
        ).unwrap()
    )
}

