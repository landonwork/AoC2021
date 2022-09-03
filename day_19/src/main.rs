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
    let mut distances: Vec<_> = scanners
        .iter()
        .map(coordinates_to_distances)
        .map(|dist| to_unsigned_hashset(&dist))
        .collect();

    loop {
        if scanners.len() == 1 { break }
        let mut first = 0;

        while first < scanners.len() - 1 {
            let mut second = first + 1;
            while second < scanners.len() {

                // Check how many beacons are shared between two scanners
                let shared_n = distances[first].intersection(&distances[second]).count();

                // If we see that there are enough beacons to match two scanner together
                if shared_n >= 66 {
                    // Oof, everything was good until right here. Forgot I still
                    // need the displacement from the first scanner to the second
                    // scanner. I'm going to need two vecs for distance, one
                    // that's signed and one that's unsigned. I'll fix that
                    // next time.
                    let orientations = Orientations::new(&scanners[second]);
                    let first_orientation = to_hashset(&scanners[first]);

                    // Then we rotate the second scanner until we find the matching orientation
                    let matched_orientation = orientations
                        .map(|or| to_hashset(&or))
                        .filter(|or| or.intersection(&first_orientation).count() >= 12)
                        .collect::<Vec<_>>()
                        .remove(0);

                    // Absorb the matched scanner into the first scanner
                    scanners[first] = first_orientation
                        .union(&matched_orientation)
                        .flatten()
                        .copied()
                        .collect::<Vec<_>>()
                        .into();

                    // Make sure to remove the second scanner form existence
                    scanners.swap_remove(second);
                    distances.swap_remove(second);

                    break;
                }

                // If there aren't enough shared beacons, increment second
                second += 1;
            }
            // If the first scanner does not currently have any matching scanners, increment first
            first += 1;
        }
    }

    scanners.remove(0)
}

