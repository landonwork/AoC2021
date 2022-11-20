#![feature(int_log)]

use std::{cmp::{Ord, PartialOrd, Ordering}, collections::HashMap, fmt::{Display, Error, Formatter}, hash::{Hash, Hasher}};
use lazy_static::lazy_static;

const EMPTY: i32 = 0;
const A: i32 = 1;
const B: i32 = 10;
const C: i32 = 100;
const D: i32 = 1000;

// y, x (d_row, d_col)
const MOVEMENT: [(i32, i32); 10] = [
    (-1, 0), (1, 0), (0, -1), (0, 1),
    (-1, -1), (-1, 1), (1, -1), (1, 1),
    (0, -2), (0, 2)
];

struct Map(pub Vec<(usize, usize)>);

impl Map {
    fn new(rows: usize) -> Self {
        let mut map = vec![
            (1, 1),
            (1, 2),
            (1, 4),
            (1, 6),
            (1, 8),
            (1, 10),
            (1, 11),
        ];
        for i in 0..rows {
            for j in 0..4 {
                map.push( (2 + i, 3 + 2 * j) );
            }
        }
        Map(map)
    }
}

lazy_static! {
    static ref TWO_ROWS: Map = Map::new(2);
    static ref FOUR_ROWS: Map = Map::new(4);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State<const N: usize> {
    inner: Inner
}

pub type Inner = HashMap<(usize, usize), i32>;

impl<const N: usize> State<N> {
    fn is_room_ready(&self, col: usize) -> bool {
        assert_eq!(col % 2, 1);
        (2..2+N).all(|row| {
            let value = *self.inner.get(&(row, col)).unwrap();
            let proper_value = match col { // Is 10 ^ 0 not 1?
                3 => 1,
                5 => 10,
                7 => 100,
                9 => 1000,
                _ => unreachable!(),
            };
            value == EMPTY || value == proper_value
        })
    }
}

impl State<2> {
    pub fn new(inner: Inner) -> Self {
        assert_eq!(inner.len(), 7 + 4 * 2);
        State { inner }
    }

    pub fn neighbors(&self, dir: usize) -> Vec<(Self, i32)> {
        match dir {
            0 => self.next_states(),
            1 => self.prev_states(),
            _ => unreachable!(),
        }
    }

    // Tuple is next state and energy required to move to that state
    pub fn next_states(&self) -> Vec<(Self, i32)> {
        let mut nexts = Vec::new();

        for (&(row, col), &value) in self.inner.iter() {
            for &(y, x) in MOVEMENT.iter() {
                if value > 0 { // If there is a non-empty space selected,
                    let new_row = (row as i32 + y) as usize;
                    let new_col = (col as i32 + x) as usize;
                    let new_place = (new_row, new_col);
                    let proper_col = (value.ilog(10) * 2 + 3) as usize;

                    match self.inner.get(&new_place) {
                        Some(&other_value) if (
                            other_value == EMPTY && // the new place is empty
                            match y {
                                0 => !(x.abs() == 2 && new_row != 1), // if we are moving sideways, we cannot be in a room and moving 2 at the same time
                                -1 => !(col == proper_col && self.is_room_ready(proper_col)), // if we are moving up, we must not already be in the correct room that is ready
                                1 => new_col == proper_col && self.is_room_ready(proper_col), // if we are moving down, it must be the correct room that is ready
                                _ => unreachable!(),
                            }
                        ) => {
                            let mut new_inner = self.inner.clone();
                            *new_inner.get_mut(&new_place).unwrap() = value;
                            *new_inner.get_mut(&(row, col)).unwrap() = other_value;

                            let energy: i32 = (x.abs() + y.abs()) * value;
                            nexts.push((State::<2>::new(new_inner), energy));
                        },
                        _ => (),
                    }
                }
            }
        }

        nexts
    }

    pub fn prev_states(&self) -> Vec<(Self, i32)> {
        let mut prevs = Vec::new();

        for (&(row, col), &value) in self.inner.iter() {
            for &(y, x) in MOVEMENT.iter() {
                if value > 0 { // If there is a non-empty space selected,
                    let new_row = (row as i32 + y) as usize;
                    let new_col = (col as i32 + x) as usize;
                    let new_place = (new_row, new_col);
                    let proper_col = (value.ilog(10) * 2 + 3) as usize;

                    match self.inner.get(&new_place) {
                        Some(&other_value) if (
                            other_value == EMPTY && // the new place is empty
                            match y {
                                0 => !(x.abs() == 2 && new_row != 1), // if we are moving sideways, we cannot be in a room and moving 2 at the same time
                                -1 => col == proper_col && self.is_room_ready(proper_col), // if we are moving up, we must already be in the correct room that is ready
                                1 => true, // if we are moving down, there is nothing stopping us
                                _ => unreachable!(),
                            }
                        ) => {
                            let mut new_inner = self.inner.clone();
                            *new_inner.get_mut(&new_place).unwrap() = value;
                            *new_inner.get_mut(&(row, col)).unwrap() = other_value;

                            let energy: i32 = (x.abs() + y.abs()) * value;
                            prevs.push((State::<2>::new(new_inner), energy));
                        },
                        _ => (),
                    }
                }
            }
        }

        prevs
    }
}

impl State<4> {
    pub fn new(inner: Inner) -> Self {
        assert_eq!(inner.len(), 7 + 4 * 4);
        State { inner }
    }

    // Tuple is next state and energy required to move to that state
    pub fn next_states(&self) -> Vec<(Self, i32)> {
        let mut nexts = Vec::new();

        for (&(row, col), &value) in self.inner.iter() {
            for &(y, x) in MOVEMENT.iter() {
                if value > 0 { // If there is a non-empty space selected,
                    let new_row = (row as i32 + y) as usize;
                    let new_col = (col as i32 + x) as usize;
                    let new_place = (new_row, new_col);
                    let proper_col = (value.ilog(10) * 2 + 3) as usize;

                    match self.inner.get(&new_place) {
                        Some(&other_value) if (
                                other_value == EMPTY && // and the new place is empty
                                (new_col == proper_col || y != 1) && // and either we are moving to the right room now or we are not descending
                                !(x.abs() == 2 && new_row == 1) // we are not trying to jump from one room to another
                            ) => {
                            let mut new_inner = self.inner.clone();
                            *new_inner.get_mut(&new_place).unwrap() = value;
                            *new_inner.get_mut(&(row, col)).unwrap() = other_value;

                            let energy: i32 = (x.abs() + y.abs()) * value;
                            nexts.push((State::<4>::new(new_inner), energy));
                        },
                        _ => (),
                    }
                }
            }
        }

        nexts
    }
}

impl<const N: usize> Display for State<N> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for row in 0..=N+1 {
            for col in 0..13 {
                write!(f,
                    "{}",
                    match self.inner.get(&(row, col)) {
                        Some(&n) => match n {
                            A => 'A',
                            B => 'B',
                            C => 'C',
                            D => 'D',
                            EMPTY => '.',
                            _ => unreachable!(),
                        },
                        None => ' ',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Hash for State<2> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher
    {
        for place in &TWO_ROWS.0 {
            state.write(&self.inner.get(place).unwrap().to_le_bytes()[..])
        }
    }
}

impl Hash for State<4> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher
    {
        for place in &FOUR_ROWS.0 {
            state.write(&self.inner.get(place).unwrap().to_le_bytes()[..])
        }
    }
}

impl<const N: usize> Ord for State<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut iter = (TWO_ROWS.0[..]).iter();
        loop {
            let next = iter.next().unwrap_or(&(0,0));
            let here = self.inner.get(next);
            let there = other.inner.get(next);
            match (here, there) {
                (Some(here), Some(there)) if here != there => {
                    break here.cmp(there)
                },
                (Some(_), Some(_)) => (),
                (None, None) => break Ordering::Equal,
                _ => unreachable!(),
            }
        }
    }
}

impl<const N: usize> PartialOrd for State<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.cmp(other) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => None,
            Ordering::Greater => Some(Ordering::Greater),
        }
    }
}

pub fn read_two_rows(path: &str) -> State<2> {
    let mut inner = HashMap::new();

    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .flat_map(|line| line.chars())
        .zip((TWO_ROWS.0).iter())
        .for_each(|(c, coord)| {
            match c {
                'A' => inner.insert(*coord, A),
                'B' => inner.insert(*coord, B),
                'C' => inner.insert(*coord, C),
                'D' => inner.insert(*coord, D),
                '.' => inner.insert(*coord, EMPTY),
                _ => unreachable!(),
            };
        });

    State::<2>::new(inner)
}

pub fn read_four_rows(path: &str) -> State<4> {
    let mut inner = HashMap::new();

    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .flat_map(|line| line.chars())
        .zip((FOUR_ROWS.0).iter())
        .for_each(|(c, coord)| {
            match c {
                'A' => inner.insert(*coord, A),
                'B' => inner.insert(*coord, B),
                'C' => inner.insert(*coord, C),
                'D' => inner.insert(*coord, D),
                '.' => inner.insert(*coord, EMPTY),
                _ => unreachable!(),
            };
        });

    State::<4>::new(inner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BinaryHeap, HashSet};

    #[test]
    fn test_read() {
        let state = read_two_rows("input.txt");
    }

    #[test]
    fn test_hash() {
        let mut set = HashSet::new();
        let state = read_two_rows("input.txt");
        set.insert(state.clone());
        set.insert(state);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_next() {
        let start = read_two_rows("input.txt");
        println!("{}", &start);

        let mut next_states = start
            .next_states()
            .into_iter()
            .map(|(state, _energy)| state)
            .collect::<BinaryHeap<_>>();

        // next_states.iter().for_each(|(state, energy)| println!("{}Energy: {}", state, energy));
        let state = next_states.pop().unwrap();
        println!("{}", state);
        
        next_states = state
            .next_states()
            .into_iter()
            .map(|(state, _energy)| state)
            .collect::<BinaryHeap<_>>();
        next_states.pop();
        next_states.pop();

        let state = next_states.pop().unwrap();
        println!("{}", state);

        next_states = state
            .next_states()
            .into_iter()
            .map(|(state, _energy)| state)
            .collect::<BinaryHeap<_>>();
        next_states.pop();
        next_states.pop();

        let state = next_states.pop().unwrap();
        println!("{}", state);

        next_states = state
            .next_states()
            .into_iter()
            .map(|(state, _energy)| state)
            .collect::<BinaryHeap<_>>();
        next_states.pop();
        next_states.pop();

        let state = next_states.pop().unwrap();
        println!("{}", state);

        next_states = state
            .next_states()
            .into_iter()
            .map(|(state, _energy)| state)
            .collect::<BinaryHeap<_>>();
        // next_states.pop();
        // next_states.pop();

        let state = next_states.pop().unwrap();
        println!("{}", state);
        // println!("YOU ARE HERE ^^^^");

        next_states = state
            .next_states()
            .into_iter()
            .map(|(state, _energy)| state)
            .collect::<BinaryHeap<_>>();

        let state = next_states.pop().unwrap();
        println!("{}", state);
        // println!("YOU ARE HERE ^^^^");

        next_states = state
            .next_states()
            .into_iter()
            .map(|(state, _energy)| state)
            .collect::<BinaryHeap<_>>();

        let state = next_states.pop().unwrap();
        println!("{}", state);
        println!("YOU ARE HERE ^^^^");

        next_states = state
            .next_states()
            .into_iter()
            .map(|(state, _energy)| state)
            .collect::<BinaryHeap<_>>();

        next_states.into_iter().for_each(|state| println!("{}", state));
        // println!("{}", next_states.pop().unwrap());
        println!("{}", state.is_room_ready(3));
    }
}





