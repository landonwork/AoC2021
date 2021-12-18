use std::fs;
use std::cmp::Ordering;

fn main() {
    let raw: String = fs::read_to_string("input.txt")
        .expect("Something went wrong");
    let codes: Vec<&str> = raw
        .trim()
        .split('\n')
        .collect();

    let mut gamma = 0usize;
    let mut epsilon = 0usize;
    for i in 0..12 {
        if let Some(x) = most_common(&codes, i) {
            gamma = gamma * 2 + x;
            epsilon = epsilon * 2 + (1 - x);
        } else { panic!("Something went wrong"); }
    }
    println!("Part 1: {}", gamma * epsilon);
    
    let mut o2 = codes.clone();
    let mut co2 = codes.clone();
    for i in 0..12 {
        let most = most_common(&o2, i);
        if let Some(x) = most {
            o2.retain(|y| &y[i..i+1] == x.to_string());
        } else {
            o2.retain(|y| &y[i..i+1] == "1");
        }
        if o2.len() == 1 {
            break;
        }
    }
    for i in 0..12 {
        let most = most_common(&co2, i);
        if let Some(x) = most {
            co2.retain(|y| &y[i..i+1] == (1-x).to_string());
        } else {
            co2.retain(|y| &y[i..i+1] == "0");
        }
        if co2.len() == 1 {
            break;
        }
    }
    
    let o2_score: &str = o2.pop().unwrap();
    let co2_score: &str = co2.pop().unwrap();

    let o2_score: usize = o2_score
        .chars()
        .fold(0usize, |acc, x| acc*2 + x.to_digit(10).unwrap() as usize);
    let co2_score: usize = co2_score
        .chars()
        .fold(0usize, |acc, x| acc*2 + x.to_digit(10).unwrap() as usize);
    
    println!("Part 2: {}", o2_score * co2_score);
}

fn most_common(v: &Vec<&str>, digit: usize) -> Option<usize> {
    let mut count: usize = 0;
    let total = v.len();
    for num in v {
        count += (&num[digit..digit+1] == "1") as usize;
    }
    let ratio = (count as f32) / (total as f32);
    if ratio == 0.5 {
        return None;
    } else if ratio > 0.5 {
        return Some(1);
    } else {
        return Some(0);
    }
}
