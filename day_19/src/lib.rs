
mod parts;
pub use parts::{part1, part2};

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

// Like numpy.stack. This stacks rows.
// let arr = ndarray::concatenate(Axis(0), [v.view(), v2.view()].as_slice()).unwrap();
// This stacks columns.
// let arr2 = ndarray::concatenate(Axis(1), &[v.view(), v2.view()]).unwrap();
use ndarray::{s, arr2, Array2, ArrayBase, Axis, concatenate, Dim, SliceArg};
use std::{collections::{HashSet, HashMap}, ops::{Add, Sub}};

type Rotation = [[i32; 3]; 3];

const Y_ROTATION: &Rotation = &[
    [ 0,  0, 1],
    [ 0,  1, 0],
    [-1,  0, 0],
];

const Z_ROTATION: &Rotation = &[
    [0, -1, 0],
    [1,  0, 0],
    [0,  0, 1],
];

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Matrix(pub Array2<i32>);

impl From<&[[i32; 3]]> for Matrix {
    fn from(array: &[[i32; 3]]) -> Self {
        Matrix(arr2(array))
    }
}

impl From<Vec<i32>> for Matrix {
    fn from(v: Vec<i32>) -> Self {
        Matrix(ArrayBase::from_shape_vec((v.len() / 3, 3), v).unwrap())
    }
}

impl Matrix {
    pub fn dot(&self, right: &Matrix) -> Matrix {
        Matrix(self.0.dot(&right.0))
    }

    pub fn slice<I: SliceArg<Dim<[usize; 2]>, OutDim = Dim<[usize; 2]>>>(&self, info: I) -> Self {
        Self(self.0.slice(info).to_owned())
    }

    pub fn dim(&self) -> (usize, usize) {
        self.0.dim()
    }

    // The smart thing would have been to keep track of the nodes for each edge
    // but doing arrays is hard enough in Rust. I can't even imagine trying to
    // make a proper graph.
    pub fn displacement(&self, other: &Matrix) -> Matrix {
        let mut count = HashMap::with_capacity(self.dim().0 * self.dim().0);

        for i in 0..self.dim().0 {
            for j in 0..other.dim().0 {

                let self_info = s![i..=i, ..];
                let other_info = s![j..=j, ..];

                let d_matrix = self.slice(self_info) - other.slice(other_info);

                let d: [i32; 3] = d_matrix.0
                    .as_slice()
                    .unwrap()
                    .try_into()
                    .unwrap();

                *count.entry(d).or_insert(0) += 1;
            }
        }

        let (displacement, _n) = count.drain()
            .max_by_key(|&(_disp, n)| n)
            .unwrap();

        Matrix::from([displacement].as_slice())
    }
}

impl Add<&mut Matrix> for &Matrix {
    type Output = Matrix;
    fn add(self, right: &mut Matrix) -> Self::Output {
        Matrix(&self.0 + &right.0)
    }
}

impl Add<Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, right: Matrix) -> Self::Output {
        Matrix(&self.0 + &right.0)
    }
}

impl Sub<&Matrix> for &mut Matrix {
    type Output = Matrix;
    fn sub(self, right: &Matrix) -> Self::Output {
        Matrix(&self.0 - &right.0)
    }
}

impl Sub<Matrix> for Matrix {
    type Output = Self;
    fn sub(self, right: Matrix) -> Self {
        Matrix(self.0 - right.0)
    }
}

pub fn rotate_y(scanner: &Matrix) -> Matrix {
    scanner.dot(&Y_ROTATION.as_slice().into())
}

pub fn rotate_z(scanner: &Matrix) -> Matrix {
    scanner.dot(&Z_ROTATION.as_slice().into())
}

pub fn read_input(path: &str) -> Vec<Matrix> {

    let raw = std::fs::read_to_string(path).unwrap();
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


pub fn coords_to_distances(scanner: &Matrix) -> Matrix {

    let mut distances: Matrix = [[0; 3]; 0].as_slice().into();
    let n_rows = scanner.dim().0;

    for i in 0..(n_rows - 1) {
        let d = scanner.slice(s![i..=i, ..]) - scanner.slice(s![(i+1).., ..]);
        distances = Matrix(concatenate(Axis(0), &[distances.0.view(), d.0.view()]).unwrap());
    }

    distances
}

pub fn original_position(mut current: usize, found: &HashSet<usize>) -> usize {
    let mut counter = 0;
    while counter <= current {
        current += found.contains(&counter) as usize;
        counter += 1;
    }
    current
}

pub fn to_hashset(a: &Matrix) -> HashSet<[i32; 3]> {
    let mut set = HashSet::with_capacity(a.dim().0);
    let mut buf: std::slice::Iter<i32> = a.0.as_slice().unwrap().iter();
    while let (Some(&n), Some(&m), Some(&p)) = (buf.next(), buf.next(), buf.next()) {
        set.insert([n, m, p]);
    }
    set
}

/// signs on coordinates of each row vector is flipped so that the x-coordinates are positive
pub fn to_unsigned_hashset(a: &Matrix) -> HashSet<[i32; 3]> {
    let mut set = HashSet::with_capacity(a.dim().0);
    let mut buf: std::slice::Iter<i32> = a.0.as_slice().unwrap().iter();
    while let (Some(&n), Some(&m), Some(&p)) = (buf.next(), buf.next(), buf.next()) {
        let arr = if n < 0 {
            [-n, -m, -p]
        } else {
            [ n,  m,  p]
        };
        set.insert(arr);
    }
    set
}

/// All elements are absoluted and then xyz coordinates of row vectors are sorted left to right
pub fn to_unordered_hashset(a: &Matrix) -> HashSet<[i32; 3]> {
    let mut set = HashSet::with_capacity(a.dim().0);
    let mut buf: std::slice::Iter<i32> = a.0.as_slice().unwrap().iter();
    while let (Some(&n), Some(&m), Some(&p)) = (buf.next(), buf.next(), buf.next()) {
        let mut arr = [n.abs(), m.abs(), p.abs()];
        arr.sort();
        set.insert(arr);
    }
    set
}

pub fn repeat<T: Clone>(n: u8, val: &T, f: fn(&T) -> T) -> T {
    if n == 0 {
        val.clone()
    } else {
        repeat(n-1, &f(val), f)
    }
}


pub struct Orientations {
    scanner: Matrix,
    face: u8,
    r_z: u8,
}

impl Orientations {
    pub fn new(scanner: &Matrix) -> Self {
        Orientations {
            scanner: scanner.clone(),
            face: 0,
            r_z: 0,
        }
    }
}

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


impl Iterator for Orientations {
    type Item = Matrix;

    fn next(&mut self) -> Option<Self::Item> {
        match self.face {
            0 | 1 | 2  => match self.r_z {
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
                    self.scanner = rotate_y(&rotate_z(&self.scanner));
                    val
                },
                _ => unreachable!(),
            },
            3 => match self.r_z {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_rotate_y() {
        let arr1 = Matrix::from([[1,2,3]].as_slice());
        let expected = Matrix::from([[-3,2,1]].as_slice());
        let actual = rotate_y(&arr1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_rotate_z() {
        let arr1 = Matrix::from([[1,2,3]].as_slice());
        let expected = Matrix::from([[2,-1,3]].as_slice());
        let actual = rotate_z(&arr1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sub_matrices() {
        let a1 = Matrix::from([[1,2,3]].as_slice());
        let a2 = Matrix::from([
            [1,2,3],
            [4,5,6],
        ].as_slice());
        assert_eq!(a1 - a2, Matrix::from([
            [0,0,0],
            [-3,-3,-3],
        ].as_slice()));
    }

    // This test doesn't actually test that the orientations are correct
    // Just that there are 24 of them
    #[test]
    fn test_orientations() {
        let scanner = Matrix::from([[1,2,3]].as_slice());
        let orientations = Orientations::new(&scanner).collect::<HashSet<_>>();
        assert_eq!(orientations.len(), 24);
    }

    #[test]
    fn test_original() {
        let mut nums: Vec<usize> = (0..100).collect();
        let mut rng = thread_rng();
        let mut found = HashSet::new();

        while nums.len() != 0 {
            let new = rng.gen_range(0..nums.len());
            nums.remove(new);
            found.insert(original_position(new, &found));
        }

        assert_eq!(found.len(), 100);
    }

    #[test]
    fn test_unsigned() {
        let distances = Matrix::from([
            [   1,   2,   3],
            [  -4,   5,   6],
            [   7,  -8,   9],
            [  10,  11, -12],
            [ -13, -14,  15],
            [ -16,  17, -18],
            [  19, -20, -21],
            [ -22, -23, -24],
        ].as_slice());
        let expected = Matrix::from([
            [   1,   2,   3],
            [   4,  -5,  -6],
            [   7,  -8,   9],
            [  10,  11, -12],
            [  13,  14, -15],
            [  16, -17,  18],
            [  19, -20, -21],
            [  22,  23,  24],
        ].as_slice());
        let mut unsorted = to_unsigned_hashset(&distances)
            .into_iter()
            .collect::<Vec<_>>();
        unsorted.sort();
        let answer: Matrix = unsorted
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        assert_eq!(answer, expected);

    }

    #[test]
    fn test_unordered() {
        let distances = Matrix::from([
            [   1,   2,   3],
            [  -4,   5,   6],
            [   7,  -8,   9],
            [ -12,  11,  10],
            [ -13, -14,  15],
            [ -16,  17, -18],
            [  19, -20, -21],
            [ -22, -23, -24],
        ].as_slice());
        let expected = Matrix::from([
            [   1,   2,   3],
            [   4,   5,   6],
            [   7,   8,   9],
            [  10,  11,  12],
            [  13,  14,  15],
            [  16,  17,  18],
            [  19,  20,  21],
            [  22,  23,  24],
        ].as_slice());
        let mut unsorted = to_unordered_hashset(&distances)
            .into_iter()
            .collect::<Vec<_>>();
        unsorted.sort();
        let answer: Matrix = unsorted
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        assert_eq!(answer, expected);
    }

    #[test]
    fn test_max_dist() {
        let scanners = Matrix::from([[68,-1246,-43], [1105,-1205,1229], [-92,-2380,-20], [-20,-1133,1061]].as_slice());

        let mut distances = to_unordered_hashset(&coords_to_distances(&scanners));
        assert_eq!(
            distances.drain().map(|[x,y,z]| x+y+z).max().unwrap(),
            3621
        );
    }

    #[test]
    fn test_distances() {
        let coords = Matrix::from([
            [1,2,3],
            [4,5,6],
            [7,8,9],
        ].as_slice());
        let expected = Matrix::from([
            [-3, -3, -3],
            [-6, -6, -6],
            [-3, -3, -3],
        ].as_slice());
        let answer = coords_to_distances(&coords);
        assert_eq!(answer, expected);
    }

    #[test]
    fn test_rotate_distance() {
        let coords = Matrix::from([
            [  1,  2,  3],
            [  4,  5,-16],
            [  7,-28,  9],
        ].as_slice());
        let distances = coords_to_distances(&coords);
        let mut coord_orientations = Orientations::new(&coords);
        let mut dist_orientations = Orientations::new(&distances);

        let mut count = 0;
        while let (Some(coord), Some(dist)) = (coord_orientations.next(), dist_orientations.next()) {
            println!("{}", count);
            assert_eq!(to_unsigned_hashset(&coords_to_distances(&coord)), to_unsigned_hashset(&dist));
            count += 1;
        }

        assert_eq!(count, 24);
    }

    #[test]
    fn test_part1() {
        let scanners = read_input("example.txt"); // A better name for this would be beacons probably but it's too late now

        let (beacons, _scanners) = part1(scanners);
        assert_eq!(beacons.dim().0, 79);
    }

    #[test]
    fn test_part2() {
        let expected = HashSet::from([
            [68,-1246,-43],
            [1105,-1205,1229],
            [-92,-2380,-20],
            [-20,-1133,1061]
        ]);

        let scanners = read_input("example.txt"); // A better name for this would be beacons probably but it's too late now

        let (_beacons, scanners) = part1(scanners);
        let actual: HashSet<[i32; 3]> = scanners
            .values()
            .map(|m| m.0.as_slice().unwrap().try_into().unwrap())
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_read_input() {
        let expected = vec![
        Matrix::from([
                     [801,940,813],
                     [620,888,825],
                     [987,567,-489],
                     [-283,662,455],
                     [64,125,-67],
                     [-556,-453,660],
                     [898,-661,-511],
                     [878,634,-457],
                     [-362,870,-789],
                     [696,-414,521],
                     [725,892,765],
                     [-683,-735,-482],
                     [615,-408,505],
                     [-642,-411,520],
                     [630,-364,389],
                     [-353,635,442],
                     [-357,783,-710],
                     [-730,-720,-602],
                     [906,720,-516],
                     [965,-768,-610],
                     [-302,752,-719],
                     [171,57,78],
                     [-707,-337,671],
                     [-367,646,507],
                     [-661,-699,-444],
                     [983,-703,-484],
                     ].as_slice()),
        Matrix::from([
                     [713,-349,676],
                     [-661,858,-701],
                     [-457,-818,-388],
                     [-777,406,868],
                     [131,44,40],
                     [809,729,963],
                     [730,657,-509],
                     [669,630,-449],
                     [-616,-700,464],
                     [-731,840,-632],
                     [-522,-902,-289],
                     [-658,-675,432],
                     [564,670,-425],
                     [-644,854,-758],
                     [820,702,771],
                     [-667,345,799],
                     [-35,-108,163],
                     [-711,394,752],
                     [707,-384,701],
                     [658,-344,518],
                     [-695,-694,667],
                     [414,-423,-685],
                     [537,-430,-625],
                     [-514,-883,-554],
                     [465,-431,-784],
                     [817,840,812],
                     ].as_slice()),
        Matrix::from([
                     [608,227,-767],
                     [-12,-4,131],
                     [631,736,790],
                     [675,687,743],
                     [741,675,859],
                     [-705,451,-458],
                     [625,223,-710],
                     [-856,379,762],
                     [-920,-664,-450],
                     [-807,-699,-556],
                     [624,-812,-468],
                     [-766,424,-518],
                     [-497,-916,765],
                     [-522,-855,743],
                     [745,-831,-541],
                     [-118,-134,14],
                     [-845,352,954],
                     [641,-845,892],
                     [652,-793,827],
                     [-442,-752,745],
                     [765,-824,935],
                     [695,241,-641],
                     [-853,241,803],
                     [738,-724,-511],
                     [-770,-659,-382],
                     [-600,411,-434],
                     ].as_slice()),
        Matrix::from([
                     [-774,631,-744],
                     [837,784,-657],
                     [-284,-752,796],
                     [-487,493,544],
                     [719,646,659],
                     [806,-758,506],
                     [795,605,674],
                     [-739,-588,-321],
                     [607,-516,-521],
                     [-634,-613,-432],
                     [-745,670,-843],
                     [704,710,-587],
                     [739,-738,424],
                     [-445,-727,788],
                     [-512,620,407],
                     [96,136,85],
                     [600,600,732],
                     [660,-640,-487],
                     [-510,509,546],
                     [-6,-10,-31],
                     [743,-696,500],
                     [-765,707,-787],
                     [-420,-801,800],
                     [-505,-612,-292],
                     [720,730,-723],
                     [583,-587,-623],
                     ].as_slice()),
        Matrix::from([
                     [-508,-795,-648],
                     [-610,784,878],
                     [-643,312,-527],
                     [-622,734,857],
                     [-572,301,-539],
                     [844,333,906],
                     [751,-584,-659],
                     [658,-616,-729],
                     [-590,-904,-645],
                     [-625,-637,581],
                     [615,336,-413],
                     [635,342,-402],
                     [102,-52,82],
                     [925,350,741],
                     [708,-918,615],
                     [-529,-765,-626],
                     [583,-538,-726],
                     [-546,823,817],
                     [463,330,-476],
                     [683,-883,621],
                     [766,342,787],
                     [-617,480,-596],
                     [-708,-618,458],
                     [866,-923,670],
                     [-665,-714,601],
                     ].as_slice()),
        Matrix::from([
                     [762,461,819        ],
                     [939,-581,-807      ],
                     [749,-487,-802      ],
                     [-539,-341,611      ],
                     [787,410,-483       ],
                     [-528,-484,-727     ],
                     [-534,612,570       ],
                     [-486,515,545       ],
                     [-673,-468,-785     ],
                     [674,508,712        ],
                     [-481,-460,-841     ],
                     [-657,576,516       ],
                     [374,-504,588       ],
                     [-742,412,-631      ],
                     [-622,-344,624      ],
                     [-700,487,-610      ],
                     [-521,-367,705      ],
                     [483,-416,687       ],
                     [28,-36,-43         ],
                     [455,-453,549       ],
                     [768,446,-452       ],
                     [768,498,886        ],
                     [764,298,-576       ],
                     [-733,628,-577      ],
                     [884,-513,-811      ],
                     ].as_slice()),
        Matrix::from([
                     [400,661,-364       ],
                     [422,699,-400       ],
                     [-680,-700,642      ],
                     [851,-688,587       ],
                     [-681,-648,731      ],
                     [649,832,902        ],
                     [-445,-631,-799     ],
                     [-381,-732,-742     ],
                     [513,-700,-531      ],
                     [-500,478,-629      ],
                     [659,783,889        ],
                     [-442,554,-680      ],
                     [-802,316,774       ],
                     [-740,-681,584      ],
                     [746,-681,489       ],
                     [353,541,-379       ],
                     [-343,-604,-649     ],
                     [-916,345,790       ],
                     [711,-611,600       ],
                     [531,-814,-643      ],
                     [722,627,916        ],
                     [21,-101,25         ],
                     [620,-763,-501      ],
                     [-717,338,814       ],
                     [-106,39,106        ],
                     [-504,657,-650      ],
                     ].as_slice()),
        Matrix::from([
                     [457,-470,383       ],
                     [-799,-448,619      ],
                     [526,-588,-697      ],
                     [-468,-362,-697     ],
                     [-759,-631,668      ],
                     [474,-389,548       ],
                     [-443,-394,-655     ],
                     [-241,359,-561      ],
                     [626,631,571        ],
                     [809,639,593        ],
                     [466,-548,502       ],
                     [-449,-597,-734     ],
                     [643,751,573        ],
                     [671,348,-446       ],
                     [-260,624,-553      ],
                     [-794,-606,674      ],
                     [-712,854,426       ],
                     [667,360,-603       ],
                     [-325,458,-496      ],
                     [-720,755,375       ],
                     [415,-471,-686      ],
                     [172,28,67          ],
                     [500,-568,-686      ],
                     [14,-114,65         ],
                     [803,304,-499       ],
                     [-713,695,498       ],
                     ].as_slice()),
        Matrix::from([
                     [762,-352,345       ],
                     [-40,104,34         ],
                     [572,-411,322       ],
                     [-709,781,-567      ],
                     [-702,-405,450      ],
                     [-539,511,638       ],
                     [597,-264,330       ],
                     [782,-791,-442      ],
                     [612,471,-350       ],
                     [674,-678,-473      ],
                     [-603,894,-602      ],
                     [592,416,-514       ],
                     [-535,744,-607      ],
                     [-100,-39,-70       ],
                     [634,689,634        ],
                     [-598,475,632       ],
                     [-664,-764,-542     ],
                     [-453,-402,455      ],
                     [739,-667,-421      ],
                     [590,496,-411       ],
                     [-694,-771,-435     ],
                     [-621,-650,-454     ],
                     [-514,470,597       ],
                     [-553,-324,471      ],
                     [641,603,759        ],
                     [682,763,704        ],
                     ].as_slice()),
        Matrix::from([
                     [306,-809,653       ],
                     [-748,-517,723      ],
                     [563,-536,-494      ],
                     [-650,649,-755      ],
                     [470,-864,633       ],
                     [-473,716,424       ],
                     [-678,-591,675      ],
                     [300,701,-546       ],
                     [-707,653,-913      ],
                     [523,-499,-668      ],
                     [394,786,361        ],
                     [-155,-52,-96       ],
                     [-581,746,492       ],
                     [398,838,421        ],
                     [-641,-810,-473     ],
                     [558,-386,-562      ],
                     [-583,619,-918      ],
                     [-682,-769,-498     ],
                     [-803,-677,682      ],
                     [3,51,-50           ],
                     [309,527,-652       ],
                     [387,866,524        ],
                     [-569,810,405       ],
                     [367,-784,631       ],
                     [-676,-805,-557     ],
                     [291,559,-691       ],
                     ].as_slice()),
        Matrix::from([
                     [500,730,715        ],
                     [-512,600,-593      ],
                     [887,-817,496       ],
                     [-623,775,716       ],
                     [-415,-624,409      ],
                     [666,564,-358       ],
                     [61,-100,88         ],
                     [-530,705,748       ],
                     [-698,772,794       ],
                     [486,796,815        ],
                     [612,-552,-592      ],
                     [528,-718,-605      ],
                     [570,801,641        ],
                     [-598,-575,449      ],
                     [-668,-462,-637     ],
                     [-860,-473,-664     ],
                     [593,488,-448       ],
                     [788,-868,488       ],
                     [-709,-465,-775     ],
                     [728,-745,495       ],
                     [-573,-569,414      ],
                     [-501,756,-610      ],
                     [552,-613,-565      ],
                     [559,568,-266       ],
                     [-462,580,-683      ],
                     ].as_slice()),
        Matrix::from([
                     [-309,-696,693      ],
                     [387,-401,-669      ],
                     [-584,-384,-829     ],
                     [-732,601,410       ],
                     [-372,792,-785      ],
                     [665,665,570        ],
                     [763,-513,718       ],
                     [620,-478,694       ],
                     [-371,685,-777      ],
                     [-333,-549,769      ],
                     [497,700,523        ],
                     [556,-463,-602      ],
                     [-631,-398,-818     ],
                     [-287,-605,682      ],
                     [-745,457,510       ],
                     [608,637,-590       ],
                     [464,-398,-561      ],
                     [-671,647,503       ],
                     [-53,-41,-59        ],
                     [497,-514,694       ],
                     [618,687,-754       ],
                     [-600,-547,-755     ],
                     [-458,754,-739      ],
                     [654,732,-712       ],
                     [453,656,615        ],
                     ].as_slice()),
        Matrix::from([
                     [372,532,360        ],
                     [639,489,-773       ],
                     [-752,-373,385      ],
                     [-452,-478,-778     ],
                     [369,-586,704       ],
                     [-386,424,-726      ],
                     [-748,-422,249      ],
                     [-740,-447,410      ],
                     [569,-658,-642      ],
                     [-615,446,506       ],
                     [-599,372,343       ],
                     [108,-129,-66       ],
                     [541,599,422        ],
                     [-376,-436,-774     ],
                     [-473,462,-795      ],
                     [-38,24,-32         ],
                     [449,640,357        ],
                     [-646,383,357       ],
                     [-387,416,-833      ],
                     [603,447,-728       ],
                     [535,-534,-695      ],
                     [-458,-416,-841     ],
                     [340,-562,470       ],
                     [480,-561,599       ],
                     [593,611,-678       ],
                     [671,-587,-618      ],
                     ].as_slice()),
        Matrix::from([
                     [-681,-444,-607     ],
                     [-759,305,789       ],
                     [660,-375,472       ],
                     [-760,464,717       ],
                     [803,-332,505       ],
                     [-440,706,-824      ],
                     [462,686,-769       ],
                     [475,-436,-333      ],
                     [371,642,-812       ],
                     [532,-358,-326      ],
                     [762,520,622        ],
                     [-671,-577,-763     ],
                     [34,-114,-80        ],
                     [-614,622,-829      ],
                     [-369,-352,713      ],
                     [596,408,642        ],
                     [-43,-12,42         ],
                     [-540,692,-758      ],
                     [579,528,644        ],
                     [352,622,-711       ],
                     [530,-398,-381      ],
                     [-509,-421,766      ],
                     [778,-423,508       ],
                     [-767,404,840       ],
                     [-367,-432,701      ],
                     [-696,-515,-617     ],
                     ].as_slice()),
        Matrix::from([
                     [-406,-409,768      ],
                     [-851,263,645       ],
                     [670,578,-625       ],
                     [-390,-794,-693     ],
                     [-784,370,546       ],
                     [-582,617,-486      ],
                     [-325,-493,743      ],
                     [738,-791,-847      ],
                     [-869,328,519       ],
                     [61,-103,74         ],
                     [756,620,-485       ],
                     [821,694,-636       ],
                     [844,311,637        ],
                     [-306,-729,-792     ],
                     [-501,623,-470      ],
                     [-516,667,-354      ],
                     [-325,-737,-527     ],
                     [475,-984,551       ],
                     [-360,-473,625      ],
                     [606,-844,-803      ],
                     [548,-970,346       ],
                     [538,-957,395       ],
                     [797,296,517        ],
                     [602,-687,-791      ],
                     [775,332,479        ],
                     ].as_slice()),
        Matrix::from([
                     [223,371,655        ],
                     [-17,-102,-128      ],
                     [-769,565,685       ],
                     [-728,308,-669      ],
                     [484,-478,-525      ],
                     [-464,-619,-619     ],
                     [258,516,-525       ],
                     [464,-452,-536      ],
                     [325,-683,843       ],
                     [-901,499,735       ],
                     [-408,-755,-723     ],
                     [-463,289,-661      ],
                     [-452,-623,-776     ],
                     [-562,373,-726      ],
                     [226,340,702        ],
                     [-770,-773,688      ],
                     [-785,484,691       ],
                     [367,-367,-536      ],
                     [-167,-74,8         ],
                     [329,284,612        ],
                     [493,497,-488       ],
                     [-630,-687,646      ],
                     [534,-754,847       ],
                     [346,-720,772       ],
                     [290,491,-567       ],
                     [-619,-838,762      ],
                     ].as_slice()),
        Matrix::from([
                     [-603,847,502       ],
                     [502,-304,-487      ],
                     [498,512,-878       ],
                     [426,353,-877       ],
                     [7,69,-157          ],
                     [-499,903,638       ],
                     [-306,-495,375      ],
                     [100,-68,-25        ],
                     [-797,-687,-425     ],
                     [-496,749,-490      ],
                     [-320,-290,448      ],
                     [571,576,461        ],
                     [593,568,644        ],
                     [-561,848,-406      ],
                     [710,-284,685       ],
                     [707,-375,660       ],
                     [375,452,-885       ],
                     [-333,-339,438      ],
                     [514,-384,-485      ],
                     [743,-327,542       ],
                     [573,456,478        ],
                     [-881,-712,-506     ],
                     [-817,-569,-460     ],
                     [449,-373,-627      ],
                     [-553,583,-425      ],
                     [-446,859,506       ],
                     ].as_slice()),
        Matrix::from([
                     [-803,753,-681      ],
                     [482,-607,645       ],
                     [-2,-14,-9          ],
                     [-713,-509,535      ],
                     [530,-808,-516      ],
                     [690,701,442        ],
                     [585,828,382        ],
                     [-618,-555,657      ],
                     [417,-699,686       ],
                     [-635,-613,-488     ],
                     [-741,604,544       ],
                     [-668,-545,493      ],
                     [554,716,-702       ],
                     [659,-668,-515      ],
                     [699,748,-748       ],
                     [764,-779,-540      ],
                     [-717,571,337       ],
                     [-582,-579,-612     ],
                     [-665,773,-545      ],
                     [-771,665,352       ],
                     [420,-776,578       ],
                     [-686,745,-531      ],
                     [-558,-477,-541     ],
                     [685,745,-830       ],
                     [672,824,531        ],
                     ].as_slice()),
        Matrix::from([
                     [252,457,679        ],
                     [745,-871,-463      ],
                     [295,600,712        ],
                     [463,767,-337       ],
                     [-619,-593,-531     ],
                     [-381,-549,-542     ],
                     [633,-683,774       ],
                     [795,-707,720       ],
                     [752,-672,-439      ],
                     [-608,681,540       ],
                     [-600,498,562       ],
                     [-732,-790,520      ],
                     [-77,114,76         ],
                     [-935,504,-530      ],
                     [-488,664,542       ],
                     [307,704,-364       ],
                     [-879,640,-634      ],
                     [-668,-710,625      ],
                     [-577,-755,621      ],
                     [258,694,-351       ],
                     [-891,636,-477      ],
                     [-457,-478,-548     ],
                     [343,482,676        ],
                     [33,-71,-5          ],
                     [803,-684,793       ],
                     [774,-757,-479      ],
                     ].as_slice()),
        Matrix::from([
                     [-405,-715,587      ],
                     [364,-869,-720      ],
                     [-369,711,-581      ],
                     [-432,-625,497      ],
                     [-450,-623,600      ],
                     [-463,656,-608      ],
                     [414,-848,-732      ],
                     [42,-94,-11         ],
                     [806,411,525        ],
                     [-719,569,678       ],
                     [-728,-476,-459     ],
                     [836,536,-753       ],
                     [325,-808,-656      ],
                     [-787,571,781       ],
                     [-791,-587,-480     ],
                     [812,422,505        ],
                     [668,605,-727       ],
                     [818,485,353        ],
                     [-461,734,-497      ],
                     [806,-778,656       ],
                     [815,-897,734       ],
                     [-900,-460,-527     ],
                     [-747,520,839       ],
                     [798,-732,815       ],
                     [847,548,-682       ],
                     ].as_slice()),
        Matrix::from([
                     [415,644,-892       ],
                     [-728,765,-806      ],
                     [-550,-764,325      ],
                     [78,14,39           ],
                     [698,-752,-507      ],
                     [-570,-663,306      ],
                     [-788,888,-813      ],
                     [598,595,-852       ],
                     [-500,-710,-586     ],
                     [530,-685,-553      ],
                     [666,-439,367       ],
                     [515,761,373        ],
                     [356,715,381        ],
                     [573,589,-887       ],
                     [-784,690,550       ],
                     [437,880,364        ],
                     [-486,-776,-486     ],
                     [818,-502,361       ],
                     [-570,-781,-660     ],
                     [-887,837,-805      ],
                     [623,-755,-585      ],
                     [-65,128,-70        ],
                     [-664,692,533       ],
                     [-506,-721,416      ],
                     [646,-428,373       ],
                     [-672,670,716       ],
                     ].as_slice()),
        Matrix::from([
                     [-625,719,839       ],
                     [832,518,718        ],
                     [876,-652,639       ],
                     [-438,-756,-679     ],
                     [942,405,-645       ],
                     [566,517,742        ],
                     [85,11,60           ],
                     [-405,-732,-886     ],
                     [-658,657,-692      ],
                     [-535,675,-781      ],
                     [711,-683,-730      ],
                     [-499,-786,-871     ],
                     [664,-661,594       ],
                     [-662,570,-841      ],
                     [865,402,-537       ],
                     [709,482,718        ],
                     [143,-89,-122       ],
                     [-418,-751,612      ],
                     [-428,-807,695      ],
                     [832,-696,547       ],
                     [-670,745,785       ],
                     [773,467,-578       ],
                     [-456,732,723       ],
                     [721,-633,-566      ],
                     [-406,-781,784      ],
                     [675,-728,-688      ],
                     ].as_slice()),
        Matrix::from([
                     [-368,311,-273      ],
                     [-673,-455,-217     ],
                     [-719,-598,-281     ],
                     [803,594,-627       ],
                     [500,479,792        ],
                     [774,533,-456       ],
                     [-754,311,535       ],
                     [614,-349,-491      ],
                     [433,516,798        ],
                     [770,-435,485       ],
                     [632,-398,513       ],
                     [749,630,-504       ],
                     [-210,-704,520      ],
                     [607,526,704        ],
                     [-722,514,562       ],
                     [-294,331,-444      ],
                     [-295,-677,562      ],
                     [-611,394,592       ],
                     [708,-323,-509      ],
                     [-325,364,-297      ],
                     [695,-326,-592      ],
                     [150,24,123         ],
                     [-754,-480,-301     ],
                     [731,-486,404       ],
                     [-228,-578,455      ],
                     ].as_slice()),
        Matrix::from([
                     [79,35,108          ],
                     [-536,-482,-465     ],
                     [486,-414,487       ],
                     [-105,82,23         ],
                     [-653,-543,690      ],
                     [716,381,-669       ],
                     [-600,-564,799      ],
                     [686,697,402        ],
                     [-512,292,-691      ],
                     [-604,-356,-560     ],
                     [-579,401,-723      ],
                     [743,840,319        ],
                     [823,314,-570       ],
                     [765,-546,-390      ],
                     [758,296,-696       ],
                     [-374,589,523       ],
                     [-390,812,488       ],
                     [574,-521,534       ],
                     [874,-645,-478      ],
                     [480,-347,524       ],
                     [-644,-596,658      ],
                     [8,-101,-48         ],
                     [793,-684,-427      ],
                     [-465,-371,-431     ],
                     [-665,373,-664      ],
                     [716,808,522        ],
                     [-431,723,469       ],
                     ].as_slice()),
        Matrix::from([
                     [-638,440,-740      ],
                     [-530,-457,-634     ],
                     [353,-251,-638      ],
                     [454,-232,-618      ],
                     [402,791,581        ],
                     [-602,530,721       ],
                     [-479,619,703       ],
                     [396,-342,-682      ],
                     [784,-410,402       ],
                     [-558,-742,716      ],
                     [336,734,-618       ],
                     [839,-412,362       ],
                     [19,161,91          ],
                     [341,560,-702       ],
                     [558,801,448        ],
                     [-558,-443,-655     ],
                     [548,816,519        ],
                     [57,-16,-27         ],
                     [832,-355,440       ],
                     [-699,632,-710      ],
                     [-475,470,616       ],
                     [-550,-593,-772     ],
                     [407,660,-688       ],
                     [-732,425,-686      ],
                     [-587,-614,585      ],
                     [-101,126,-64       ],
                     [-555,-660,730      ],
                     ].as_slice()),
        Matrix::from([
                     [348,416,486        ],
                     [484,-581,-433      ],
                     [-392,-530,-995     ],
                     [510,775,-955       ],
                     [-715,552,-909      ],
                     [-396,-826,583      ],
                     [-417,-736,589      ],
                     [548,-690,663       ],
                     [466,-687,772       ],
                     [-547,617,256       ],
                     [-710,588,-895      ],
                     [-678,422,-876      ],
                     [446,836,-916       ],
                     [-394,-767,733      ],
                     [-610,572,305       ],
                     [-429,-640,-849     ],
                     [431,539,505        ],
                     [439,-638,-612      ],
                     [434,815,-866       ],
                     [413,-569,-444      ],
                     [391,559,582        ],
                     [-522,640,403       ],
                     [-65,17,-107        ],
                     [527,-668,633       ],
                     [-367,-515,-784     ],
                     ].as_slice()),
        Matrix::from([
                     [500,-631,852       ],
                     [32,-125,33         ],
                     [622,673,585        ],
                     [521,-789,783       ],
                     [-466,598,354       ],
                     [-768,-873,546      ],
                     [559,-722,-776      ],
                     [-304,-525,-317     ],
                     [-220,-500,-398     ],
                     [188,51,63          ],
                     [-479,521,494       ],
                     [-300,319,-746      ],
                     [652,610,692        ],
                     [-795,-717,483      ],
                     [589,349,-460       ],
                     [764,724,674        ],
                     [532,266,-317       ],
                     [559,469,-349       ],
                     [452,-623,-843      ],
                     [485,-751,805       ],
                     [-368,591,459       ],
                     [-447,286,-777      ],
                     [-781,-922,556      ],
                     [-284,-643,-352     ],
                     [-276,266,-703      ],
                     [517,-749,-861      ],    
                     ].as_slice()),
                ];

        let input = read_input("input.txt");
        assert_eq!(input, expected);
    }
}
