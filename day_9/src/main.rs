use std::fs;

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    let map: Vec<Vec<u32>> = raw
        .trim()
        .split('\n')
        .map(
            |x| x.chars().map(|c| c.to_digit(10).unwrap()).collect()
            )
        .collect();

    let mut risk: u32 = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if is_low_point(i, j, &map) {
                risk += map[i][j] + 1;
            }
        }
    }
    println!("Part 1: {}", risk);
}

fn is_low_point(x: usize, y: usize, map: &Vec<Vec<u32>>) -> bool {

    let w: usize = map[0].len();
    let h: usize = map.len();

    if x != 0 {
        if map[x][y] >= map[x - 1][y] {
            return false;
        }
    }
    if x != w - 1 {
        if map[x][y] >= map[x + 1][y] {
            return false;
        }
    }
    
    if y != 0 {
        if map[x][y] >= map[x][y - 1] {
            return false;
        }
    }
    if y != h - 1 {
        if map[x][y] >= map[x][y + 1] {
            return false;
        }
    }

    true
}
