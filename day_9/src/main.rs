use std::fs;

type Map = Vec<Vec<u32>>;

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    let map: Map = raw
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

    println!("Part 2: {}", part2(&map));
}

fn part2(map: &Map) -> u128 {
    let mut basins: Map = vec![vec![0; map[0].len()]; map.len()];
    let mut row = 0usize;
    let mut col = 0usize;
    let mut n_basins = 0u32;
    let w = map[0].len();
    let h = map.len();

    while col < map.len() {
        if basins[row][col] != 0 || map[row][col] == 9 {
            row = (row + 1) % map.len();
            if row == 0 { col += 1; }
            continue
        }
        n_basins += 1;
        explore_basin(row, col, map, &mut basins, n_basins);
    }

    let mut basin_sizes: Vec<u32> = vec![0; n_basins as usize];
    for row in 0..h {
        for col in 0..w {
            if basins[row][col] != 0 {
                basin_sizes[(basins[row][col] - 1) as usize] += 1
            }
        }
    }
    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes[0..3].iter().fold(1, |a,b| a* (*b as u128))
}

fn explore_basin(row: usize, col: usize, map: &Map,
                 basins: &mut Map, n: u32) -> () {

    let w = map[0].len();
    let h = map.len();

    if row >= h || col >= w {
        return ()
    }

    if map[row][col] == 9 || basins[row][col] != 0 {
        return ()
    }
    
    basins[row][col] = n;

    if row != 0 {
        explore_basin(row-1, col, map, basins, n);
    }
    explore_basin(row+1, col, map, basins, n);
    if col != 0 {
        explore_basin(row, col-1, map, basins, n);
    }
    explore_basin(row, col+1, map, basins, n);
}

fn is_low_point(x: usize, y: usize, map: &Map) -> bool {

    let w: usize = map[0].len();
    let h: usize = map.len();

    if x != 0 {
        if map[x][y] >= map[x - 1][y] {
            return false;
        }
    }
    if x != h - 1 {
        if map[x][y] >= map[x + 1][y] {
            return false;
        }
    }
    
    if y != 0 {
        if map[x][y] >= map[x][y - 1] {
            return false;
        }
    }
    if y != w - 1 {
        if map[x][y] >= map[x][y + 1] {
            return false;
        }
    }

    true
}
