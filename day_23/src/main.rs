//! Day 23
#![feature(int_log)]

use std::collections::HashMap;
use rayon::prelude::*;
use day_23::{State, read_two_rows, read_four_rows};

fn main() {
    let start1 = read_two_rows("input.txt");
    let end1 = read_two_rows("part1_solved.txt");
    // let state2 = read_four_rows("input2.txt");
    
    println!("Part 1: {}", part1(start1, end1));
    // println!("Part 2: {}", part2(state2));
}

// We are using a meet-in-the-middle type approach
// Because not every next state in the path will have the same total energy,
// I wanted to alternate between moving forward and moving backward
// using levels of energy rather than number of steps
// in order to make sure the first path found was the shortest, but I think
// that would slow things down because the computer will be making a bunch
// of moves that are just A's going back and forth.
//
// Instead I'm going to have a maximum energy limit set forward and I will
// do a true breadth-first search from each end. At each step, I'll create
// a new HashMap<State, i32>. Only States with a total energy cost of
// less than the established upper limit will be included. That limit
// initially is 58000 because I guessed that high on my puzzle and I know
// it was too high. That and dropping the old HashMaps should help put a
// cap on my memory usage (as well as the meet-in-the-middle algorithm, maybe).
// Once the step sets start overlapping, the limit will continue to be updated
// (and minimized) until there are no paths remaining that have an energy cost
// less than the estimated least energy cost. Then I return that energy cost.
fn part1(start: State<2>, end: State<2>) -> i32 {
    let mut ripples = [HashMap::from([(start, 0)]), HashMap::from([(end, 0)])];
    let mut limit: i32 = 58000;

    let mut count = 0;
    loop {
        let r = count % 2;
        let ripple = &ripples[r];
        let states: HashMap<_, _> = ripple.par_iter().flat_map( |(state, energy)|
            state
                .neighbors(r)
                .into_par_iter()
                .filter_map(|(next_state, next_energy)| {
                    let e = *energy + next_energy;
                    if e < limit {
                        Some((next_state, *energy + next_energy))
                    } else {
                        None
                    }
                })
        ).collect();

        println!("{}", states.len());

        if states.is_empty() { break limit }
        if limit != 58000 {
            let min_start = ripples[0].par_iter().min_by_key(|(_state, energy)| *energy).unwrap().1;
            let min_end = ripples[1].par_iter().min_by_key(|(_state, energy)| *energy).unwrap().1;
            if min_start + min_end > limit { break limit }
        }

        ripples[r] = states;
        let larger_ripple = (&ripples[..]).iter().max_by_key(|rip| rip.len()).unwrap(); // Reference to larger hashmap
        let smaller_ripple = (&ripples[..]).iter().min_by_key(|rip| rip.len()).unwrap(); // Reference to smaller hashmap
        let limit_candidate = smaller_ripple.par_iter().filter_map(|(state, energy)| {
            if larger_ripple.contains_key(state) {
                Some(energy + larger_ripple.get(state).unwrap())
            } else {
                None
            }
        }).min();
        match limit_candidate {
            Some(new_limit) if new_limit < limit => {
                limit = new_limit;
            },
            _ => ()
        }

        count += 1;

        if count % 1000 == 0 { println!("{count}") }
    }
}

