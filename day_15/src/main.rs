// Alright, I don't have the will to do this right now,
// but I think I know how to do what Dalon is doing
// 1. Keep a list of what hasn't been visited (find out what a binaryheap is)
//    Actually, I bet it is like a vector but it pushes onto the left.
//    (That is not what a binary heap is)
// 2. From the starting point, update distances
// 3. If you updated it and it had already been visited, mark it to be
//    visited again.
// 4. Continue doing this until there is nothing left to visit
use std::collections::{HashMap,BinaryHeap};
use std::cmp::Ordering;

#[macro_use]
extern crate lazy_static;

#[derive(Hash, Copy, Clone, Eq, PartialEq)]
struct Point {
    p: (i32, i32),
}

// Because of how the cmp method is designed, the Points that are closer to
// the origin will be popped off the BinaryHeap first. This should eliminate
// some backtracking.
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (-self.p.0 - self.p.1).cmp(&(-other.p.0 - other.p.1))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Map = HashMap<Point, i32>;
type Queue = BinaryHeap<Point>;

fn main() {
    // Read input
    let raw: String = std::fs::read_to_string("input.txt").unwrap();
    let mut m: Map = HashMap::new();

    // Build hashmap
    let (mut row, mut col) = (0, 0);
    for c in raw.chars() {
        if c != '\n' {
            m.insert(Point{p:(row, col)}, c.to_digit(10).unwrap() as i32);
            col += 1;
            if col == 100 {
                col = 0;
                row += 1;
            }
        }
    }

    // Solve
    println!("Part 1: {}", part1(m.clone()));
    println!("Part 2: {}", part2(m));
}

fn dijkstra(p: Point, m: &Map, dist: &mut Map, q: &mut Queue) {
    
    lazy_static! {
        static ref DIRS: [Point; 4] =  [
            Point{p:( 0, -1)},
            Point{p:( 1,  0)},
            Point{p:( 0,  1)},
            Point{p:(-1,  0)},
        ];
    }

    for &dir in DIRS.iter() {
        // For each neighbor, find the total distance required to reach it
        // from the current state with the current distance traveled
        let neighbor = Point{p:(p.p.0 + dir.p.0, p.p.1 + dir.p.1)};
        if let Some(d) = dist.get(&neighbor) {
            let alt = dist.get(&p).unwrap() + m.get(&neighbor).unwrap();
            if alt < *d {
                // If the alternative distance of moving to the next state from
                // the current state is less than the previously believed
                // least distance, then change the least distance and add the
                // next state to the queue
                *dist.get_mut(&neighbor).unwrap() = alt;
                q.push(neighbor);
            }
        }
    }
}

fn part1(m: Map) -> i32 {
    // Dijkstra's algorithm (kinda sorta)
    // Put one thing into the queue
    let mut q: Queue = BinaryHeap::from([Point{p:(0,0)}]);
    // Create a new copy of the least distance hashmap
    let mut d: Map = m.clone();
    // Set all distances to the maximum
    for (_, val) in d.iter_mut() {
        *val = i32::MAX;
    }

    // Set the least distance of starting point to 0
    *d.get_mut(&Point{p:(0,0)}).unwrap() = 0;
    
    // While there are still things in the queue, continue to explore the
    // graph. New Points will be added to the graph as Points are visited and
    // shown to have a smaller distance than previously believed.
    loop {
        if let Some(p) = q.pop() {
            dijkstra(p, &m, &mut d, &mut q);
        } else {
            break;
        }
    }

    *d.get(&Point{p:(99,99)}).unwrap()
}

fn part2(m: Map) -> i32 {
    let mut big_map = HashMap::with_capacity(100*100*5*5);
    for i in 0..500 {
        for j in 0..500 {
            let &(mut val) = m.get(&Point{p:(i % 100,j % 100)}).unwrap();
            val += (i / 100) + (j / 100);
            while val > 9 {
                val -= 9;
            }
            big_map.insert(Point{p:(i,j)}, val);
        }
    }
    let mut d: Map = big_map.clone();
    for (_, val) in d.iter_mut() {
        *val = i32::MAX;
    }

    *d.get_mut(&Point{p:(0,0)}).unwrap() = 0;
    
    let mut q: Queue = BinaryHeap::from([Point{p:(0,0)}]);

    loop {
        if let Some(p) = q.pop() {
            dijkstra(p, &big_map, &mut d, &mut q);
        } else {
            break;
        }
    }

    *d.get(&Point{p:(499,499)}).unwrap()
}
