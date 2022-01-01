use std::fs;
use regex::Regex;
use std::cmp::{max, min};

type Paper<T> = Vec<Vec<T>>;

fn shape<T>(p: &Paper<T>) -> (usize, usize) {
    (p.len(), p[0].len())
}


fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = raw.trim().split('\n').collect();
    let empty_line: usize = lines.iter().position(|&x| x.eq("")).unwrap();
    
    let mut pts: Vec<(usize, usize)> = Vec::new();
    let pat = Regex::new(r"(\d+),(\d+)").unwrap();
    for line in &lines[..empty_line] {
        let cap = pat.captures(line).unwrap();
        pts.push((cap[1].parse().unwrap(), cap[2].parse().unwrap()));
    }

    let (x_max, y_max) = pts.iter()
        .fold((0,0), |acc, (x,y)| (max(acc.0,*x), max(acc.1,*y)));

    let mut paper = vec![vec![false; x_max+1]; y_max+1];
    for (x, y) in pts.into_iter() {
        paper[y][x] = true;
    }

    let mut folds: Vec<_> = Vec::new();
    for &line in &lines[empty_line..] {
        if line == "" { continue; }
        let b: bool = line.chars().nth(11) == Some('x');
        let pos: usize = line[13..].parse().unwrap();
        folds.push((b, pos));
    }

    println!("Part 1: {}", part1(paper.clone(), &folds));
    println!("Part 2:\n{:?}", part2(paper, &folds));
}

fn fold_paper(paper: Paper<bool>, (vert, pos): (bool, usize)) -> Paper<bool> {
    let mut new_paper: Paper<bool>;
    let mut side1: Paper<bool> = Vec::new();
    let mut side2: Paper<bool> = Vec::new();
    if vert /*vertical crease; fold paper x=pos*/ {        
        paper.iter().map(|x| (&x[..pos]).to_vec()).for_each(|x| side1.push(x));
        paper.iter().map(|x| (&x[pos+1..]).to_vec()).for_each(|x| side2.push(x));
        new_paper = vec![vec![false; max(side1[0].len(), side2[0].len())]; side1.len()];
        for i in 0..new_paper.len() {
            for j in 0..min(side1[0].len(), side2[0].len()) {
                new_paper[i][j] = side1[i][side1[0].len()-j-1] || side2[i][j];
            }
        }
    } else /*horizontal crease; fold paper y=pos*/ {
        side1 = (&paper[..pos]).to_vec();
        side2 = (&paper[pos+1..]).to_vec();
        new_paper = vec![vec![false; side1[0].len()]; max(side1.len(), side2.len())];
        for i in 0..min(side1.len(), side2.len()) {
            for j in 0..new_paper[0].len() {
                new_paper[i][j] = side1[side1.len()-i-1][j] || side2[i][j];
            }
        }
    }
    new_paper
}

fn count_dots(p: &Paper<bool>) -> usize {
    p.iter().fold(
        0, |acc, x| acc + x.iter().fold(
            0, |acc2, &x2| acc2 + (x2 as usize)
        )
    )
}

fn part1(mut paper: Paper<bool>, folds: &Vec<(bool, usize)>) -> usize {
    paper = fold_paper(paper, folds[0]);
    count_dots(&paper)
}

fn part2(mut paper: Paper<bool>, folds: &Vec<(bool, usize)>) -> Paper<usize> {
    for &fold in folds {
        paper = fold_paper(paper, fold);
    }
    let paper: Paper<usize> = paper.into_iter().map(
        |x| x.into_iter().map(|y| y as usize).rev().collect()
        ).rev().collect();
    paper
}
