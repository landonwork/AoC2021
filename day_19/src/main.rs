use std::collections::HashSet;
use day_19::{Orientations, Matrix, to_hashset, to_unsigned_hashset};
use ndarray::{s, Axis, concatenate};

fn main() {

    // Parse the input
    let scanners = read_input();

    let full_map = part_1(scanners);

    // Profit
    println!("Part 1: {}", full_map.dim().0);
}

fn read_input() -> Vec<Matrix> {

    let raw = std::fs::read_to_string("input.txt").unwrap();
    let mut scanners = Vec::with_capacity(27);

    let mut scanner = Vec::new();
    for line in raw.lines() {

        if line.is_empty() {
            scanners.push(std::mem::take(&mut scanner).into());
            continue;
        }

        if let Some(s) = line.get(..3) {
            if s == "---" {
                continue;
            }
        } 
        
        line.split(',')
            .try_for_each(|x| {

            scanner.push(x.parse()?);
            
            Ok::<(),std::num::ParseIntError>(())

        }).unwrap();
    }

    scanners
}

fn coordinates_to_distances(scanner: &Matrix) -> Matrix {

    let mut distances: Matrix = (&[[0; 3]; 0]).into();
    let n_rows = scanner.dim().0;

    for i in 0..(n_rows - 1) {
        let d = scanner.slice(s![i..(i+1), ..]) - scanner.slice(s![(i+1).., ..]);
        distances = Matrix(concatenate(Axis(0), &[distances.0.view(), d.0.view()]).unwrap());
    }

    distances
}

fn part_1(mut scanners: Vec<Matrix>) -> Matrix {

    let mut distances: Vec<Matrix> = scanners
        .iter()
        .map(coordinates_to_distances)
        .collect();
    // let mut hashed_distances: Vec<HashSet> = distances
    //     .iter()
    //     .map(to_hashset)
    //     .collect();
    // Used to find matching scanners
    let mut unsigned_distances: Vec<HashSet<_>> = distances
        .iter()
        .map(to_unsigned_hashset)
        .collect();

    loop {
        if scanners.len() == 1 { break }
        if scanners.len() == 3 {
            println!("{:?}", &scanners);
            break;
        }
        let mut first = 0;

        while first < scanners.len() - 1 {
            let mut second = first + 1;
            while second < scanners.len() {

                // Check how many beacons are shared between two scanners
                let shared_n = unsigned_distances[first]
                    .intersection(&unsigned_distances[second])
                    .count();

                // If we see that there are enough beacons to match two scanners together
                if shared_n >= 66 {
                    // let first_orientation = to_hashset(&scanners[first]);
                    let first_distances = to_hashset(&coordinates_to_distances(&scanners[first]));
                    let scanner_orientations = Orientations::new(&scanners[second]);
                    let dist_orientations = Orientations::new(&distances[second]);

                    // Then we rotate the second scanner until we find the matching orientation
                    // This uses our signed distance hashsets
                    let second_oriented = dist_orientations
                        .zip(scanner_orientations)
                        .map(|(dist, scanner)| (to_hashset(&dist), scanner))
                        .find(
                            |(dist, _scanner)| dist.intersection(&first_distances).count() > 1
                        )
                        .unwrap()
                        .1;

                    // Once we have found the correct orientation for the second scanner,
                    // we must find the correct displacement between the two scanners
                    let displacement = scanners[first].displacement(&second_oriented);

                    // Then we modify the coordinates of the second using the displacement
                    // let second_matched = second_oriented.apply_sub(displacement);
                    let second_matched = second_oriented - displacement;

                    // TODO
                    // Absorb the matched scanner into the first scanner
                    scanners[first] = to_hashset(&scanners[first])
                        .union(&to_hashset(&second_matched))
                        .flatten()
                        .copied()
                        .collect::<Vec<_>>()
                        .into();

                    // Remove the second scanner from existence
                    scanners.swap_remove(second);
                    distances.swap_remove(second);
                    unsigned_distances.swap_remove(second);

                    // Recompute distance and unsigned distance for first scanner
                    distances[first] = coordinates_to_distances(&scanners[first]);
                    unsigned_distances[first] = to_unsigned_hashset(&distances[first]);

                    // Print
                    println!("Combined scanners {} and {}", first, second);
                    println!("{} scanners remaining", scanners.len());

                    break;
                }

                // If there aren't enough shared beacons, increment second
                second += 1;
            }
            // If the first scanner does not currently have any matching scanners, increment first
            first += 1;
        }

        // break;
    }

    scanners.remove(0)
}

