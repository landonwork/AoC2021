use std::fs;
use regex::Regex;
use std::collections::HashSet;

type Board = [[usize;5];5];

fn main() {
    let raw: String = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = raw
        .trim()
        .split('\n')
        .collect();
    
    let calls: &str = input[0];
    let calls: Vec<usize> = calls
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut boards: Vec<Board> = build_boards(&input[1..]);

    println!("Part 1: {}", part1(&calls, &boards));
    println!("Part 2: {}", part2(&calls, &mut boards));
}

fn build_boards(board_str: &[&str]) -> Vec<Board> {
    
    let mut boards: Vec<Board> = Vec::new();
    let mut i: usize = 0;
    let mut board: Board = [[0; 5]; 5];

    for line in board_str.into_iter() {

        if line.eq(&"") {
            continue;
        }
        
        let p = Regex::new(r"\s+").expect("Invalid regex");
        let row: Vec<_> = p
            .split(line.trim())
            .map(|x| {
                match x.parse::<usize>() {
                    Ok(n) => n,
                    Err(error) => panic!("Cannot parse value ({}): {:?}", x, error),
                }
            })
            .collect();

        for j in 0..5 {
            board[i % 5][j] = row[j];
        }

        if i % 5 == 4 {
            boards.push(board)
        }
        i += 1;
    }
    boards
}

fn part1(calls: &Vec<usize>, boards: &Vec<Board>) -> usize {
    let mut called: HashSet<usize> = HashSet::new();
    for call in calls {
        called.insert(*call);
        for board in boards {
            if check_board(&board, &called) {
                let mut s = 0;
                for i in 0..5 {
                    for j in 0..5 {
                        if !called.contains(&board[i][j]) {
                            s += board[i][j];
                        }
                    }
                }
                return s * *call;
            }
        }
    }
    0
}

fn part2(calls: &Vec<usize>, boards: &mut Vec<Board>) -> usize {
    let mut called: HashSet<usize> = HashSet::new();
    for call in calls {
        called.insert(*call);
        if boards.len() == 1 && check_board(&boards[0], &called) {
            let mut s = 0;
            for i in 0..5 {
                for j in 0..5 {
                    if !called.contains(&boards[0][i][j]) {
                        s += boards[0][i][j];
                    }
                }
            }
            return s * *call;
        }
        boards.retain(|b| !check_board(&b, &called));
    }
    0
}

fn check_board(b: &Board, called: &HashSet<usize>) -> bool {
    // Check diagonals
    // let diag1: bool = (0..5)
    //     .fold(true, |acc, x| acc && called.contains(&b[x][x]));
    // let diag2: bool = (0..5)
    //     .fold(true, |acc, x| acc && called.contains(&b[x][4-x]));
    // if diag1 || diag2 {
    //     return true;
    // }

    // Check rows
    for i in 0..5 {
        let this_one: bool = (0..5)
            .fold(true, |acc, x| acc && called.contains(&b[i][x]));
        if this_one {
            return true;
        }
    }

    // Check columns
    for j in 0..5 {
        let this_one: bool = (0..5)
            .fold(true, |acc, x| acc && called.contains(&b[x][j]));
        if this_one {
            return true;
        }
    }

    false
}

