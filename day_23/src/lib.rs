use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    collections::HashMap,
    fmt::{Debug, Display, Error, Formatter},
    hash::{Hash, Hasher},
    str::FromStr
};

use lazy_static::lazy_static;

// y, x (d_row, d_col)
const MOVEMENT: [(i32, i32); 10] = [
    (-1, 0), (1, 0), (0, -1), (0, 1),
    (-1, -1), (-1, 1), (1, -1), (1, 1),
    (0, -2), (0, 2)
];

const EMPTY: i32 = 0;
const A: i32 = 1;
const B: i32 = 10;
const C: i32 = 100;
const D: i32 = 1000;
pub const SIZES: [i32; 4] = [A, B, C, D];
const IND_TO_COORD: [(i32, i32); 23] = [
    (1, 1), (1, 2), (1, 4), (1, 6), (1, 8), (1, 10), (1, 11),
    (2, 3), (2, 5), (2, 7), (2, 9),
    (3, 3), (3, 5), (3, 7), (3, 9),
    (4, 3), (4, 5), (4, 7), (4, 9),
    (5, 3), (5, 5), (5, 7), (5, 9)
];

lazy_static! {
    static ref COORD_TO_IND: HashMap<(i32, i32), usize> = HashMap::from([
        ((1, 1), 0), ((1, 2), 1), ((1, 4), 2), ((1, 6), 3), ((1, 8), 4), ((1, 10), 5), ((1, 11), 6),
        ((2, 3), 7), ((2, 5), 8), ((2, 7), 9), ((2, 9), 10),
        ((3, 3), 11), ((3, 5), 12), ((3, 7), 13), ((3, 9), 14),
        ((4, 3), 15), ((4, 5), 16), ((4, 7), 17), ((4, 9), 18),
        ((5, 3), 19), ((5, 5), 20), ((5, 7), 21), ((5, 9), 22)
    ]);
}

#[derive(Clone)]
pub struct Board<const N: usize> {
    inner: [i32; N],
    energy: i32, // distance traveled
    loss: i32 // a-star heuristic
}

impl<const N: usize> Board<N> {

    const ROWS: i32 = ((N - 7) / 4) as i32;

    pub fn new(inner: [i32; N], energy: i32) -> Self {
        assert_eq!((N - 7) % 4, 0);

        let mut slf = Self { inner, energy, loss: 0 };
        let loss = slf.calc_loss();
        slf.set_loss(loss);
        slf
    }

    pub fn with_loss(inner: [i32; N], energy: i32, loss: i32) -> Self {
        assert_eq!((N - 7) % 4, 0);

        Self { inner, energy, loss }
    }

    fn amph_displacement(amph: i32, (row, col): (i32, i32)) -> i32 {
        let proper_col = (2 * SIZES.iter().position(|&size| size == amph).unwrap() + 3) as i32;
        if col == proper_col {
            Self::ROWS + 1 - row
        } else {
            Self::ROWS + (col - proper_col).abs() + row - 1
        }
    }

    pub fn energy(&self) -> i32 {
        self.energy
    }

    pub fn loss(&self) -> i32 {
        self.loss
    }

    fn calc_loss(&self) -> i32 {
        #[allow(non_snake_case)]
        let mut loss = -(Self::ROWS * (Self::ROWS - 1) / 2) * 1111;

        self.inner.iter().enumerate().for_each(|(i, amph)| {
            if amph == &EMPTY { return; }
            loss += amph * Self::amph_displacement(*amph, IND_TO_COORD[i]);
        });

        loss
    }

    fn set_loss(&mut self, loss: i32) {
        self.loss = loss;
    }

    pub fn is_room_ready(&self, col: i32) -> bool {
        assert_eq!(col % 2, 1);

        (0..Self::ROWS).all(|row| {
            let amph = self.inner[6 + (col as usize / 2) + row as usize * 4];
            let proper_amph = match col { // Is 10 ^ 0 not 1?
                3 => 1,
                5 => 10,
                7 => 100,
                9 => 1000,
                _ => unreachable!(),
            };
            amph == EMPTY || amph == proper_amph
        })
    }

    pub fn next_boards(&self) -> Vec<Self> {
        let mut nexts = Vec::new();

        for (i, &amph) in self.inner.iter().enumerate() {

            if amph == EMPTY { continue }

            let (row, col) = IND_TO_COORD[i];

            for &(y, x) in MOVEMENT.iter() {
                let new_row = row + y;
                let new_col = col + x;

                // Does this where I am trying to move even exist?
                let Some(&new_ind) = COORD_TO_IND.get(&(new_row, new_col)) else { continue };
                if new_ind >= N { continue }

                let proper_col = (2 * SIZES.iter().position(|&size| size == amph).unwrap() + 3) as i32;

                if self.inner[new_ind] == EMPTY && // the new place is empty
                    match y {
                        0 => !(x.abs() == 2 && new_row != 1), // if we are moving sideways, we cannot be in a room and moving 2 at the same time
                        -1 => !(col == proper_col && self.is_room_ready(proper_col)), // if we are moving up, we must not already be in the correct room that is ready
                        1 => new_col == proper_col && self.is_room_ready(proper_col), // if we are moving down, it must be the correct room that is ready
                        _ => unreachable!(),
                    }
                {
                    let mut new_inner = self.inner;
                    // Swap the amphipod into the new spot
                    new_inner.swap(i, new_ind);
                    // Calculate the energy expended by the travel
                    let energy: i32 = (x.abs() + y.abs()) * amph + self.energy();
                    // Calculate the change in loss
                    let old_dist = Self::amph_displacement(amph, (row, col));
                    let new_dist = Self::amph_displacement(amph, (new_row, new_col));
                    let change_in_loss = (new_dist - old_dist) * amph;
                    
                    nexts.push(Board::with_loss(new_inner, energy, self.loss() + change_in_loss));
                }
            }
        }

        nexts
    }
}

impl<const N: usize> Debug for Board<N> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for row in 0..=Self::ROWS+1 {
            for col in 0..13 {
                let ind = COORD_TO_IND.get(&(row, col));
                write!(
                    f,
                    "{}",
                    match ind {
                        Some(&n) if n < N => match self.inner[n] {
                            A => 'A',
                            B => 'B',
                            C => 'C',
                            D => 'D',
                            EMPTY => '.',
                            _ => unreachable!(),
                        },
                        _ => ' ',
                    }
                )?;
            }
            writeln!(f)?;
        }
        writeln!(
            f,
            "Energy: {}",
            self.energy
            )?;
        writeln!(
            f,
            "Loss: {}",
            self.loss
            )?;

        Ok(())
    }
}

impl<const N: usize> Display for Board<N> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for row in 0..=Self::ROWS+1 {
            for col in 0..13 {
                let ind = COORD_TO_IND.get(&(row, col));
                write!(
                    f,
                    "{}",
                    match ind {
                        Some(&n) if n < N => match self.inner[n] {
                            A => 'A',
                            B => 'B',
                            C => 'C',
                            D => 'D',
                            EMPTY => '.',
                            _ => unreachable!(),
                        },
                        _ => ' ',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


impl<const N: usize> Eq for Board<N> { }

impl<const N: usize> FromStr for Board<N> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = [EMPTY; N];
        let mut i = 0;

        for c in s.chars() {
            inner[i] = match c {
                '.' => EMPTY,
                'A' => A,
                'B' => B,
                'C' => C,
                'D' => D,
                _   => continue,
            };

            i += 1;
        }
        assert_eq!(i, N);

        Ok(Self::new(inner, 0))
    }
}

impl<const N: usize> Hash for Board<N> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher
    {
        // I am purposefully excluding `energy` and `loss` from the Hash
        // trait. I want to order by loss and energy, but not equate by them.
        self.inner.iter().for_each(|amph| 
            state.write(&amph.to_le_bytes()[..])
        );
    }
}

impl<const N: usize> Ord for Board<N> {
    // BinaryHeaps pop off the largest one first, so the Board with the
    // smallest F-costs will be bigger
    fn cmp(&self, other: &Self) -> Ordering {
        let self_f_cost = self.energy + self.loss;
        let other_f_cost = other.energy + other.loss;

        match self_f_cost.cmp(&other_f_cost) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => match self.loss.cmp(&other.loss) {
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less,
                Ordering::Equal => self.inner.cmp(&other.inner),
            }
        }
    }
}

impl<const N: usize> PartialEq for Board<N> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<const N: usize> PartialOrd for Board<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let o = self.cmp(other);
        if o == Ordering::Equal {
            None
        } else {
            Some(o)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashSet, BinaryHeap};

    #[test]
    fn test_ind_coord() {
        (0..23).for_each(|i| {
            let coord = IND_TO_COORD[i];
            assert_eq!(COORD_TO_IND.get(&coord).unwrap(), &i);
        })
    }

    #[test]
    fn test_from_str() {
        let s = r#"
        AB C D . ..
          A B C D
          A B C D
        "#;
        let board = Board::<15>::from_str(s).unwrap();
    }

    #[test]
    fn test_is_room_ready() {
        let s = r#"
        .. C . D .C
          A B . .
          A B . D
        "#;
        let board = Board::<15>::from_str(s).unwrap();
        assert!([3i32,5,7,9].iter().all(|&col| board.is_room_ready(col)));
    }

    #[test]
    fn test_room_is_not_ready() {
        let s = r#"
        AB . . . ..
          B . C C
          . A D D
        "#;
        let board = Board::<15>::from_str(s).unwrap();
        assert!([3i32,5,7,9].iter().all(|&col| !board.is_room_ready(col)));
    }

    #[test]
    fn test_zero_loss() {
        let s = r#"
        .. . . . ..
          A B C D
          A B C D
          A B C D
          A B C D
        "#;
        let board = Board::<23>::from_str(s).unwrap();
        assert_eq!(board.loss(), 0);
    }

    #[test]
    fn test_hash() {
        let inner = [ 0,0,0,0,0,0,0 ];
        let board1 = Board::<7>::with_loss(inner.clone(), 0, 0);
        let board2 = Board::<7>::with_loss(inner, 10, 10);
        assert_eq!(&board1, &board2);

        let mut hash = HashSet::from([board1]);
        hash.insert(board2);

        assert_eq!(hash.len(), 1);
    }

    #[test]
    fn test_ordering() {
        // TODO
        let mut heap = BinaryHeap::<Board<15>>::with_capacity(5);
        let board1 = r#"
        .. . . . ..
          A B C D
          A B C D
        "#.parse().unwrap();
        heap.push(board1);
        assert_eq!(heap.pop().unwrap().loss(), 0);
    }
}
