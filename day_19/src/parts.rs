use std::collections::{HashMap, HashSet};
use crate::{
    Matrix, Orientations, coords_to_distances, to_hashset, to_unsigned_hashset, to_unordered_hashset,
    original_position,
};

pub fn part1(mut scanners: Vec<Matrix>) -> (Matrix, HashMap<usize, Matrix>) {

    // Data structure onto which we can impose rotations
    let mut distances: Vec<Matrix> = scanners
        .iter()
        .map(coords_to_distances)
        .collect();
    // For finding correct orientation
    let mut unsigned_distances: Vec<HashSet<_>> = distances
        .iter()
        .map(to_unsigned_hashset)
        .collect();
    // For finding matches
    let mut unordered_distances: Vec<HashSet<_>> = distances
        .iter()
        .map(to_unordered_hashset)
        .collect();

    // Part 2
    let mut tracker: HashMap<usize, HashMap<usize, Matrix>> = HashMap::new();
    let mut scanners_found: HashSet<usize> = HashSet::new();

    loop {
        if scanners.len() == 1 { break }

        // If we just started or we got through all the scanners, then start over
        let mut first = 0;

        // while first < scanners.len() - 1 {
            let mut second = first + 1;
            while second < scanners.len() {

                // Check how many beacons are shared between two scanners
                let shared_n = unordered_distances[first]
                    .intersection(&unordered_distances[second])
                    .count();
                // println!("{}, {}", first, second);

                // If we see that there are enough beacons to match two scanners together
                if shared_n >= 66 {
                    
                    let first_distances = to_unsigned_hashset(&distances[first]);
                    let scanner_orientations = Orientations::new(&scanners[second]);
                    let dist_orientations = Orientations::new(&distances[second]);

                    // Then we rotate the second scanner until we find the matching orientation
                    if let Some((rotation, _, second_oriented)) = dist_orientations
                        .zip(scanner_orientations)
                        .enumerate()
                        .map(|(n, (dist, scanner))| (n, to_unsigned_hashset(&dist), scanner))
                        .find(|(_, dist, _scanner)| dist.intersection(&first_distances).count() >= shared_n - 5) { // I had to give the matching here some leniency for some reason

                            // Once we have found the correct orientation for the second scanner,
                            // we must find the correct displacement between the two scanners
                            let displacement = scanners[first].displacement(&second_oriented);

                            // Part 2
                            // Record the displacement in the tracker
                            // 1. Find the original positions of the scanners we are matching
                            let original_first = original_position(first, &scanners_found);
                            let original_second = original_position(second, &scanners_found);

                            // 2. Add the displacement to 0 matrix or existing displacement
                            let mut_in_place = tracker
                                .entry(original_first) // Entry enum (Occupied or Vacant)
                                .or_insert(HashMap::new()) // &mut HashMap<i32, Matrix>
                                    .entry(original_second) // Entry enum
                                    .or_insert(Matrix::from([[0,0,0]].as_slice())); // &mut Matrix
                            *mut_in_place = displacement.clone() + mut_in_place.clone();

                            // 3. If the second scanner had absorbed any scanners, move them to the
                            //    new scanner and add the displacement (I have to make sure these
                            //    rotate the same amount as the second scanner)
                            if let Some(second_disps) = tracker.remove(&original_second) {
                                second_disps
                                    .into_iter()
                                    .for_each(|(second_, disp)| {
                                        tracker
                                            .entry(original_first)
                                            .or_insert(HashMap::new())
                                            .insert(second_, Orientations::new(&disp).nth(rotation).unwrap());
                                    })
                            }

                            // 4. Mark the second scanner as having been found
                            scanners_found.insert(original_second);

                            // Then we modify ("match") the coordinates of the second to the first using the displacement
                            let second_matched = second_oriented + displacement;

                            // Absorb the matched scanner into the first scanner
                            scanners[first] = to_hashset(&scanners[first])
                                .union(&to_hashset(&second_matched))
                                .flatten()
                                .copied()
                                .collect::<Vec<_>>()
                                .into();

                            // Remove the second scanner from existence
                            scanners.remove(second);
                            distances.remove(second);
                            unsigned_distances.remove(second);
                            unordered_distances.remove(second);

                            // Recompute distances for first scanner
                            distances[first] = coords_to_distances(&scanners[first]);
                            unsigned_distances[first] = to_unsigned_hashset(&distances[first]);
                            unordered_distances[first] = to_unordered_hashset(&distances[first]);

                            // Signal progress
                            println!("Combined scanners {} and {}", first, second);
                            println!("{} scanners remaining", scanners.len());

                        } else {
                            todo!();
                            // second += 1;
                        }
                    // Do not increment if you found a match
                    break;
                }
                // If there aren't enough shared beacons, increment second
                second += 1;
            }
    }

    (scanners.remove(0), tracker.remove(&0).unwrap())
}

pub fn part2(scanners: Matrix) -> i32 {
    let mut distances = to_unordered_hashset(&coords_to_distances(&scanners));
    distances.drain().map(|[x,y,z]| x+y+z).max().unwrap()
}
