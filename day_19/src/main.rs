
#![allow(clippy::or_fun_call)]

use std::collections::{HashSet, HashMap};

use day_19::{
    Orientations, Matrix, coords_to_distances, original_position, read_input,
    to_hashset, to_unsigned_hashset, to_unordered_hashset, part1, part2
};
use ndarray::{Axis, concatenate};


fn main() {

    // let scanners = read_input("input.txt");
    let scanners = read_input("input.txt"); // A better name for this would be beacons probably but it's too late now
    println!("Number of scanners: {}", scanners.len());

    let (beacons, mut scanners) = part1(scanners);
    // println!("{:?}", &scanners);

    println!("Part 1: {}", beacons.dim().0);

    let scanners: Vec<_> = scanners.drain().map(|(_, m)| m).collect();
    let slice = scanners.iter().map(|m| m.0.view()).collect::<Vec<_>>();
    let map = Matrix(concatenate(Axis(0), slice.as_slice()).unwrap());
    let largest_distance = part2(map);

    println!("Part 2: {}", largest_distance);
}

