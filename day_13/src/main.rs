use std::fs;
use nalgebra::{DMatrix, Matrix, Dim, RawStorage, dmatrix};
use regex::Regex;
use std::cmp::max;

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
    let mut paper: DMatrix<bool> = DMatrix::from_element(x_max+1,y_max+1,false);
    let _a = pts.into_iter().map(|c| {paper[c] = true}).collect::<()>();
    println!("{:?}", paper.shape());

    let folds: Vec<(char, usize)> = lines[(empty_line+1)..]
        .iter()
        .map(|line| (line.chars().nth(11).unwrap(), line[13..].parse().unwrap()))
        .collect();

    println!("Part 1: {}", part1(paper.clone(), &folds));
    // println!("Part 2: {}", part2(paper));
}

fn left_fits_in_right<T, R: Dim, C: Dim, S: RawStorage<T,R,C>>(left: Matrix<T, R, C, S>, right: Matrix<T, R, C, S>) -> bool {
    left.shape().0 <= right.shape().0 && left.shape().1 <= right.shape().1
}

fn fold_paper(mut paper: DMatrix<bool>, orient: char, pos: usize) -> DMatrix<bool> {
    let mut new: Option<DMatrix<bool>> = None;
    if orient == 'y' {
        let top = paper.rows(0, pos);
        let bottom = paper.rows(pos, paper.shape().0 - pos);
        if left_fits_in_right(top, bottom) {
            println!("Folding top over bottom!");
            // let top_iter = top.row_iter().reversed();
            // let new_mat = DMatrix::from_element(bottom.shape().0, bottom.shape().1, false);
            // let new_iter = new_mat.row_iter_mut();
            // let _a = bottom.row_iter().map(
            //     |row| {new_iter.next() = row & top_iter.next().unwrap()}
            //     );
        } else if left_fits_in_right(bottom, top) {
            println!("Folding bottom over top!");
        } else {
            panic!("Neither side fits!");
        }
        // Resize top and bottom to be the same
        // Combine top and bottom into new
    } else if orient == 'x' {
        let left = paper.columns(0, pos);
        let right = paper.columns(pos, paper.shape().1 - pos);
        if left_fits_in_right(left, right) {
            println!("Folding left over right!");
        } else if left_fits_in_right(right, left) {
            println!("Folding right over left!");
        } else {
            panic!("Neither side fits!");
        }
        // let left = DMatrix::from_element(2,2,true);
        // left = left.resize_horizontally(1, false);
    } else {
        panic!("Cannot fold along {} axis", pos);
    }
    
    let new_mat = DMatrix::from_element(2,2,true);
    let other = dmatrix![false, true;
                          true, false];
    new_mat = new_mat.and(other);
                                  
    new_mat
}

fn part1(mut paper: DMatrix<bool>, folds: &Vec<(char, usize)>) -> usize {
    let mut iters = folds.iter();
    let fold = iters.next().unwrap();
    paper = fold_paper(paper, fold.0, fold.1);
    let ans: usize = paper.fold(0, |acc,x| acc + x as usize);
    ans
}
