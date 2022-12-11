//! Day 23
use std::{
    collections::{HashMap, BinaryHeap},
    iter::Extend,
    str::FromStr,
};
use day_23::Board;

fn main() {
    let s1 = std::fs::read_to_string("input.txt").unwrap();
    let board1 = Board::<15>::from_str(s1.as_str()).unwrap();
    let s2 = std::fs::read_to_string("input2.txt").unwrap();
    let board2 = Board::<23>::from_str(s2.as_str()).unwrap();

    println!("{:?}", &board1);
    println!("Part 1: {}", a_star(board1));
    println!("{:?}", &board2);
    println!("Part 2: {}", a_star(board2));
}

fn a_star<const N: usize>(board: Board<N>) -> i32 {
    let mut goal = board.clone();
    // I think I have to do a little dijkstra's
    let mut explored = HashMap::from([(board.clone(), board.energy())]);
    let mut queue = BinaryHeap::from([board]);
    let mut count = 0;

    while let Some(board) = queue.pop() {
        if board.loss() < goal.loss() {
            goal = board.clone();
        }
        if goal.loss() == 0 {
            println!("{:?}", &goal);
            return goal.energy()
        }

        let nexts: Vec<_> = board
            .next_boards()
            .into_iter()
            .filter(|next| explored.get(next).unwrap_or(&i32::MAX) > &next.energy())
            .collect();
        nexts.iter().for_each(|next| { explored.insert(next.clone(), next.energy()); });
        queue.extend(nexts);

        count += 1;
        if count % 100000 == 0 {
            println!("{count}");
        }
    }

    0
}
