use std::ops::Index;
use std::marker::Copy;

struct Map<T: Copy> {
    m: Vec<Vec<T>>
}

impl Index<(usize, usize)> for Map<u32>
{
    type Output = u32;

    fn index(&self, (row, col): (usize, usize)) -> &u32 {
        &self.m[row][col]
    }
}


fn main() {
    let raw: String = std::fs::read_to_string("input.txt").unwrap();
    let rows: Vec<&str> = raw.trim().split('\n').collect();
    let m: Vec<Vec<u32>> = rows.into_iter().map(
        |row| row.chars().map(|c| c.to_digit(10).unwrap()).collect()
        ).collect();
    let map: Map<u32> = Map { m };

    // println!("{:?}", get_shape(&map));
    assert_eq!(7, map[(1,2)]);
    // println!("Part 1: {}", part1(&map));
    // println!("Part 2: {}", part2(&map));
}

fn get_shape<T: Copy>(map: Map<T>) -> (usize, usize) {
    (map.m.len(), map.m[0].len()) // (rows, cols)
}

fn part1(m: &Vec<Vec<u32>>) -> u32 {
    10
}

fn part2(m: &Vec<Vec<u32>>) -> u32 {
    10
}
