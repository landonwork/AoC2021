use std::{collections::HashSet, ops::Sub};
use ndarray::{s, arr2, Axis, Array2, ArrayBase, concatenate};

// X rotation
// [   1      0       0    ]
// [   0    cos(t) -sin(t) ]
// [   0    sin(t)  cos(t) ]

// Y rotation (flip down 90 degrees)
// [ cos(t)   0     sin(t) ]
// [   0      1       0    ]
// [-sin(t)   0     cos(t) ]

// Z rotation
// [ cos(t) -sin(t)    0    ]
// [ sin(t)  cos(t)    0    ]
// [   0       0       1    ]

// Total # of orientations = 24
// 1. R0
//  + Rz
// 2. Rz
//  + Rz
// 3. 2Rz
//  + Rz
// 4. 3Rz
//  + Rz + Ry
// 5. Ry
//  + Rz
// 6. Ry + Rz
//  + Rz
// 7. Ry + 2Rz
//  + Rz
// 8. Ry + 3Rz
//  + Rz + Ry
// 9. 2Ry
//  + Rz
// 10. 2Ry + Rz
//  + Rz
// 11. 2Ry + 2Rz
//  + Rz
// 12. 2Ry + 3Rz
//  + Rz + Ry
// 13. 3Ry
//  + Rz
// 14. 3Ry + Rz
//  + Rz
// 15. 3Ry + 2Rz
//  + Rz
// 16. 3Ry + 3Rz
//  + Rz + Ry + Rz + Ry
// 17. Rz + Ry
//  + Rz
// 18. Rz + Ry + Rz
//  + Rz
// 19. Rz + Ry + 2Rz
//  + Rz
// 20. Rz + Ry + 3Rz
//  + Rz + 3Ry + 2Rz
// 21. 3Rz + Ry
//  + Rz
// 22. 3Rz + Ry + Rz
//  + Rz
// 23. 3Rz + Ry + 2Rz
//  + Rz
// 24. 3Rz + Ry + 3Rz

type Rotation = [[i32; 3]; 3];

const Y_ROTATION: Rotation = [
    [ 0,  0, 1],
    [ 0,  1, 0],
    [-1,  0, 0],
];

const Z_ROTATION: Rotation = [
    [0, -1, 0],
    [1,  0, 0],
    [0,  0, 1],
];

fn main() {
    let v = arr2(&[[1,2,3]]);
    let v2 = arr2(&[[2,3,4]]);
    // Like numpy.stack. This stacks rows.
    let arr = ndarray::concatenate(Axis(0), &[v.view(), v2.view()]).unwrap();
    // This stacks columns.
    let arr2 = ndarray::concatenate(Axis(1), &[v.view(), v2.view()]).unwrap();

    println!("{:?}", arr2);
    // Parse the input
    let scanners = read_input();

    // Convert to sets of Cartesian distances
    let distances: Vec<_> = scanners.into_iter().map(coordinates_to_distances).collect();

    // ???

    // Profit
    // println!("Part 1: {}", total_beacons);
}

type Scanner = Array2<i32>;

fn read_input() -> Vec<Scanner> {

    let raw = std::fs::read_to_string("input.txt").unwrap();
    let mut scanners = Vec::with_capacity(27);

    let mut scanner = Vec::new();
    for line in raw.lines() {

        if line.is_empty() {
            scanners.push(std::mem::take(&mut scanner));
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
        .into_iter()
        .map(|scanner| ArrayBase::from_shape_vec((scanner.len() / 3, 3), scanner).unwrap())
        .collect()
}

fn coordinates_to_distances(scanner: Scanner) -> Scanner {

    let mut distances: Scanner = arr2(&[[0; 3]; 0]);
    let n_rows = scanner.dim().0;

    for i in 0..(n_rows - 1) {
        let d = scanner.slice(s![..1_usize, ..]).to_owned() - scanner.slice(s![1_usize.., ..]).to_owned();
        distances = concatenate(Axis(0), &[distances.view(), d.view()]).unwrap();
    }

    distances
}

fn piece_together(mut distances: Vec<Scanner>) -> Scanner { // Can change to (Scanner, Scanner) later
    
    while distances.len() > 1 {
        let mut flag = false;
        for scanner1 in 0..(distances.len() - 1) {
            let orientations = Orientations::new(&distances[scanner1]);
            for orientation in orientations {
                let scanner1_set = to_hashset(&orientation);
                for scanner2 in distances.iter().skip(scanner1) {
                    let scanner2_set = to_hashset(scanner2);
                    let shared_nodes = scanner1_set.intersection(&scanner2_set).count();
                    if shared_nodes >= 66 {
                        flag = true;

                        break;
                    }
                }
                if flag { break; }
            }
        }
    }

    distances.swap_remove(0)
}

struct Orientations {
    scanner: Scanner,
    face: u8,
    r_z: u8,
}

impl Orientations {
    fn new(scanner: &Scanner) -> Self {
        Orientations {
            scanner: scanner.clone(),
            face: 0,
            r_z: 0,
        }
    }
}

impl Iterator for Orientations {
    type Item = Scanner;

    fn next(&mut self) -> Option<Self::Item> {
        match self.face {
            0 | 1 | 2 | 3 => match self.r_z {
                0 | 1 | 2 => {
                    let val = Some(self.scanner.clone());
                    self.scanner = rotate_z(&self.scanner);
                    self.r_z += 1;
                    val
                },
                3 => {
                    let val = Some(self.scanner.clone());
                    self.face += 1;
                    self.r_z = 0;
                    self.scanner = rotate_y(&rotate_z(&rotate_y(&rotate_z(&self.scanner))));
                    val
                },
                _ => unreachable!(),
            },
            4 => match self.r_z {
                0 | 1 | 2 => {
                    let val = Some(self.scanner.clone());
                    self.r_z += 1;
                    self.scanner = rotate_z(&self.scanner);
                    val
                },
                3 => {
                    let val = Some(self.scanner.clone());
                    self.r_z = 0;
                    self.face += 1;
                    self.scanner = rotate_z(&self.scanner);
                    self.scanner = repeat(3, &self.scanner, rotate_y);
                    self.scanner = repeat(2, &self.scanner, rotate_z);
                    self.scanner = rotate_y(&self.scanner);
                    val
                },
                _ => unreachable!(),
            },
            5 => {
                let val = Some(self.scanner.clone());
                self.r_z += 1;
                self.face += self.r_z / 4;
                self.scanner = rotate_z(&self.scanner);
                val
            },
            6 => None,
            _ => {
                println!("{}", self.face);
                unreachable!()
            },
        }
    }
}

fn repeat<T: Clone>(n: u8, val: &T, f: fn(&T) -> T) -> T {
    if n == 0 {
        val.clone()
    } else {
        repeat(n-1, &f(val), f)
    }
}

fn to_hashset(a: &Scanner) -> HashSet<[i32; 3]> {
    let mut set = HashSet::with_capacity(a.dim().0);
    let mut buf: std::slice::Iter<i32> = a.as_slice().unwrap().iter();
    while let (Some(&n), Some(&m), Some(&p)) = (buf.next(), buf.next(), buf.next()) {
        set.insert([n, m, p]);
    }
    set
}

// fn inv_tri_number(n: usize) -> usize {
//     if n == 0 { 0 }
//     else {
//         let mut ans = 1;
//         loop {
//             if ans * (ans - 1) / 2 == n {
//                 break ans
//             }
//             ans += 1;
//         }
//     }
// }

fn rotate_y(scanner: &Scanner) -> Scanner {
    scanner.dot(&arr2(&Y_ROTATION))
}

fn rotate_z(scanner: &Scanner) -> Scanner {
    scanner.dot(&arr2(&Z_ROTATION))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_y() {
        let arr1 = arr2(&[[1,2,3]]);
        let expected = arr2(&[[-3,2,1]]);
        let actual = rotate_y(&arr1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_rotate_z() {
        let arr1 = arr2(&[[1,2,3]]);
        let expected = arr2(&[[2,-1,3]]);
        let actual = rotate_z(&arr1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn add_matrices() {
        let a1 = arr2(&[[1,2,3]]);
        let a2 = arr2(&[
            [1,2,3],
            [4,5,6],
        ]);
        assert_eq!(a1 + a2, arr2(&[
            [2,4,6],
            [5,7,9],
        ]));
    }

    #[test]
    fn test_orientations() {
        let scanner = arr2(&[[1,2,3]]);
        let mut orientations = Orientations::new(&scanner);
        assert_eq!(orientations.count(), 24);
    }
}
