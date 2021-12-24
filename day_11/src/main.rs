use std::fs;

type Octos = Vec<Vec<usize>>;

mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let rows = ["5483143223",
                    "2745854711", 
                    "5264556173",
                    "6141336146",
                    "6357385478",
                    "4167524645",
                    "2176841721",
                    "6882881134",
                    "4846848554",
                    "5283751526",];
        let mut dumbos: Octos = rows.iter().map(
            |x| x.chars().map(|y| y.to_digit(10).unwrap() as usize).collect()
            ).collect();
        let ans = part1(&mut dumbos);
        println!("{}", ans);
        assert_eq!(ans, 1656);
    }

    #[test]
    fn second_part() {
        let rows = ["5483143223",
                    "2745854711", 
                    "5264556173",
                    "6141336146",
                    "6357385478",
                    "4167524645",
                    "2176841721",
                    "6882881134",
                    "4846848554",
                    "5283751526",];
        let mut dumbos: Octos = rows.iter().map(
            |x| x.chars().map(|y| y.to_digit(10).unwrap() as usize).collect()
            ).collect();
        
        assert_eq!(part2(&mut dumbos), 195);
    }
}

fn main() {
    let raw: String = fs::read_to_string("input.txt")
        .unwrap();
    let rows: Vec<&str> = raw
        .trim()
        .split('\n')
        .collect();
    let mut octos: Octos = Vec::with_capacity(raw.len());
    for s in rows {
        octos.push(
            s.chars().map(
                |x| x.to_digit(10).unwrap() as usize
            ).collect()
        );
    }
    let mut octos1 = octos.clone();
    let mut octos2 = octos;
    
    println!("Part 1: {}", part1(&mut octos1));
    println!("Part 2: {}", part2(&mut octos2));
}

fn increment_all(octos: &mut Octos) -> () {
    for i in 0..octos.len() {
        for j in 0..octos[0].len() {
            octos[i][j] += 1;
        }
    }
}

fn flash(i: usize, j: usize, octos: &mut Octos, n_flashes: &mut usize) -> () {
    if octos[i][j] > 9 {
        *n_flashes += 1;
        octos[i][j] = 0;
        for k in -1i32..=1 {
            for l in -1i32..=1 {
                let ind1: usize = ((i as i32)+k) as usize;
                let ind2: usize = ((j as i32)+l) as usize;
                if ind1 < octos.len() && ind2 < octos[0].len() {
                    flash(ind1, ind2, octos, n_flashes);
                }
            }
        }
    } else if octos[i][j] > 0 {
        octos[i][j] += 1;
    }
}

fn step(octos: &mut Octos, n_flashes: &mut usize) -> () { 
    increment_all(octos);
    // Flashes
    for i in 0..octos.len() {
        for j in 0..octos[0].len() {
            if octos[i][j] > 9 {
                flash(i, j, octos, n_flashes);
            }
        }
    }
}

fn part1(octos: &mut Octos) -> usize {

    let mut n_flashes: usize = 0;

    for _ in 0..100 {
        step(octos, &mut n_flashes);
    }

    n_flashes
}

fn part2(octos: &mut Octos) -> usize {
    let mut n_flashes = 0;
    let mut step_counter = 0;
    let max_flashes = octos.len() * octos[0].len();

    loop {
        step(octos, &mut n_flashes);
        step_counter += 1;
        println!("{}", n_flashes);
        if n_flashes == max_flashes {
            break;
        }
        n_flashes = 0;
    }
    step_counter
}
