use std::fs;
use regex::Regex;

struct Board {
    nums: [[i32; 5]; 5],
    called: [[bool; 5]; 5],
}

fn main() {
    let raw: String = read_input("input.txt");
    let input: Vec<String> = raw
        .trim()
        .split('\n')
        .map(|x| x.to_string())
        .collect();
    
    
    let calls: &str = &input[0];
    let calls: Vec<i32> = calls
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    // println!("{:?}", calls);

    let mut boards: Vec<Board> = build_boards(&input[1..]);
    // println!("{}", boards.len());
    
    let mut winner: Option<&Board> = None;
    for n in calls.iter() {
        for i in 0..boards.len() {
            mark_board(&mut boards[i], *n); // so there's a problem with this line
            if check_board(&boards[i]) {
                winner = Some(&boards[i]); // and this line
                break; // Because the borrow checker can't tell that
            } // the former will never be executed again :/
        }
        match winner {
            None => (),
            Some(_) => {break;}
        }
    }
    let winner: &Board = winner.unwrap();
    println!("{:?}\n{:?}", winner.nums, winner.called);
}

fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn read_input(filename: &str) -> String {
    let result = fs::read_to_string(filename).expect("Something went wrong reading the file");
    result
}

fn build_boards(board_str: &[String]) -> Vec<Board> {
    
    let mut boards: Vec<Board> = Vec::new();
    let mut i: usize = 0;
    let mut board: [[i32; 5]; 5] = [[0; 5]; 5];

    for line in board_str.into_iter() {

        if line.eq("") {
            // println!("Skipping");
            continue;
        }
        
        let p = Regex::new(r"\s+").expect("Invalid regex");
        let row: Vec<_> = p
            .split(line.trim())
            .map(|x| {
                match x.to_string().trim().parse::<i32>() {
                    Ok(n) => n,
                    Err(error) => panic!("Cannot parse value ({}): {:?}", x, error),
                }
            })
            .collect();
        // println!("{:?}", row);

        for j in 0..5 {
            board[i % 5][j] = row[j];
        }

        if i % 5 == 4 {
            boards.push(Board {
                nums: board,
                called: [[false; 5]; 5],
            });

        }
        i += 1;
    }
    boards
}

fn mark_board<'a>(b: &'a mut Board, n: i32) {
    for i in 0..5 {
        for j in 0..5{
            if b.nums[i][j] == n {
                b.called[i][j] = true;
            }
        }
    }
}

fn check_board(b: &Board) -> bool {
    // Check diagonals
    let diag1: bool = b.called[0][0] & b.called[1][1] & b.called[2][2] & b.called[3][3] & b.called[4][4];
    let diag2: bool = b.called[0][4] & b.called[1][3] & b.called[2][2] & b.called[3][1] & b.called[4][0];
    if diag1 | diag2 {return true;}

    // Check rows
    for i in 0..5 {
        let mut rows: bool = false;
        for j in 0..5 {
            rows = rows | b.called[i][j];
        }
        if rows {return true;}
    }

    // Check columns
    for j in 0..5 {
        let mut cols: bool = false;
        for i in 0..5 {
            cols = cols | b.called[i][j];
        }
        if cols {return true;}
    }

    false

}
