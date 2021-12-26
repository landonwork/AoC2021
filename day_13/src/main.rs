use std::fs;
use nalgebra::DMatrix;
use regex::Regex;

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    let line = raw.trim().split('\n').collect();
    let empty_line = lines.position(|x| x.eq("")).unwrap();
    
    let pts: Vec<usize> = Vec::new();
    let pat = Regex::new(r"(\d+), (\d+)");
    for line in &lines[..empty_line] {
        
    }
}
