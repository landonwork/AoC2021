use std::fs;

#[derive(Copy, Clone)]
struct Line((usize, usize), (usize, usize));

fn main() {

    let input: String = fs::read_to_string("input.txt").unwrap();
    let mut lines: Vec<Line> = Vec::new();
    for line in input.trim().split('\n') {
        let start_end: Vec<String> = line.split(" -> ").map(|x| x.to_string()).collect();
        let start: Vec<String> = start_end[0].split(',').map(|x| x.to_string()).collect();
        let end: Vec<String> = start_end[1].split(',').map(|x| x.to_string()).collect();
        let start_x: usize = start[0].to_string().parse::<usize>().unwrap();
        let start_y: usize = start[1].to_string().parse::<usize>().unwrap();
        let end_x: usize = end[0].to_string().parse::<usize>().unwrap();
        let end_y: usize = end[1].to_string().parse::<usize>().unwrap();
        lines.push(Line((start_x, start_y), (end_x, end_y)))
    }
    // println!("{:?}", lines[0].1.1);
    part_1(lines.clone());
    part_2(lines);
}

fn part_1(lines: Vec<Line>) {
    let mut map: [[usize; 1000]; 1000] = [[0; 1000]; 1000];
    for line in lines.iter() {

        if is_horizontal(&line) {
            // println!("It's horizontal!");
            // println!("(({},{}),({},{}))", line.0.0, line.0.1, line.1.0, line.1.1);
            let lesser = the_lesser(line.0.0, line.1.0); // Want to make sure
            let greater = the_greater(line.0.0, line.1.0); // the range steps up by 1
            let range = lesser..=greater; // Need an inclusive range
            for x in range {
                let y: usize = line.0.1 as usize;
                map[x][y] += 1;
            }
            continue; // In case any of the lines have a length of 1
        }             // Otherwise, they would count as both horizontal and vertical

        if is_vertical(&line) {
            // println!("It's vertical!");
            // println!("(({},{}),({},{}))", line.0.0, line.0.1, line.1.0, line.1.1);
            let lesser = the_lesser(line.0.1, line.1.1);
            let greater = the_greater(line.0.1, line.1.1);
            let range = lesser..=greater;
            for y in range {
                let x: usize = line.0.0 as usize;
                map[x][y] += 1;
            }
        }
    }
    
    let mut count: usize = 0;
    for i in 1..1000 {
        for j in 1..1000 {
            count += (map[i][j] >= 2) as usize;
        }
    }

    // println!("{:?}", map);
    println!("Part 1: {}", count);
}

fn part_2(lines: Vec<Line>) {
    let mut map: [[usize; 1000]; 1000] = [[0; 1000]; 1000];
    for line in lines.iter() {

        if is_horizontal(&line) {
            // println!("It's horizontal!");
            // println!("(({},{}),({},{}))", line.0.0, line.0.1, line.1.0, line.1.1);
            let lesser = the_lesser(line.0.0, line.1.0); // Want to make sure
            let greater = the_greater(line.0.0, line.1.0); // the range steps up by 1
            let range = lesser..=greater; // Need an inclusive range
            for x in range {
                let y: usize = line.0.1 as usize;
                map[x][y] += 1;
            }
            continue; // In case any of the lines have a length of 1
        }             // Otherwise, they would count as both horizontal and vertical

        if is_vertical(&line) {
            // println!("It's vertical!");
            // println!("(({},{}),({},{}))", line.0.0, line.0.1, line.1.0, line.1.1);
            let lesser = the_lesser(line.0.1, line.1.1);
            let greater = the_greater(line.0.1, line.1.1);
            let range = lesser..=greater;
            for y in range {
                let x: usize = line.0.0 as usize;
                map[x][y] += 1;
            }
            continue;
        }

        println!("It's diagonal!");
        let parity1   = which_lesser(line.0.0, line.1.0);
        let parity2   = which_lesser(line.0.1, line.1.1);
        let lesser_x  = the_lesser(  line.0.0, line.1.0);
        let greater_x = the_greater( line.0.0, line.1.0);
        let lesser_y  = the_lesser(  line.0.1, line.1.1);
        let greater_y = the_greater( line.0.1, line.1.1);

        if parity1 == parity2 {
            println!("Drawing straight!");
            for (x,y) in (lesser_x..=greater_x).zip(lesser_y..=greater_y) {
                map[x][y] += 1;
            }
        } else {
            println!("Drawing orthogonal!");
            for (x,y) in (lesser_x..=greater_x).zip((lesser_y..=greater_y).rev()) {
                map[x][y] += 1;
            }
        }
    }
    
    let mut count: usize = 0;
    for i in 1..1000 {
        for j in 1..1000 {
            count += (map[i][j] >= 2) as usize;
        }
    }

    // println!("{:?}", map);
    println!("Part 2: {}", count);
}

fn is_horizontal(l: &Line) -> bool {
    l.0.1 == l.1.1
}

fn is_vertical(l: &Line) -> bool {
    l.0.0 == l.1.0
}

fn the_greater(a: usize, b: usize) -> usize {
    if a < b {return b;}
    else {return a;}
}

fn the_lesser(a: usize, b: usize) -> usize {
    if a < b {return a;}
    else {return b;}
}

// fn which_greater(a: usize, b: usize) -> usize {
//     if a < b {return 1;}
//     else {return 0;}
// }

fn which_lesser(a: usize, b: usize) -> usize {
    if a < b {return 0;}
    else {return 1;}
}

