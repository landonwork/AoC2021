use std::{ops::Range, cmp::{max, min}};

#[derive(Clone, Debug)]
struct Cuboid {
    x_range: Range<i64>,
    y_range: Range<i64>,
    z_range: Range<i64>,
    pub positive: bool
}

impl Cuboid {

    fn volume(&self) -> i64 {
        let x = self.x_range.end - self.x_range.start + 1;
        let y = self.y_range.end - self.y_range.start + 1;
        let z = self.z_range.end - self.z_range.start + 1;
        x * y * z * (if self.positive { 1 } else { -1 })
    }

    fn intersection(&self, rhs: &Cuboid) -> Option<Cuboid> { // The right-hand side will be the ones that are already in the queue
        let x_range = Range { 
            start: max(self.x_range.start, rhs.x_range.start),
            end:   min(self.x_range.end,   rhs.x_range.end  ),
        };
        let y_range = Range { 
            start: max(self.y_range.start, rhs.y_range.start),
            end:   min(self.y_range.end,   rhs.y_range.end  ),
        };
        let z_range = Range { 
            start: max(self.z_range.start, rhs.z_range.start),
            end:   min(self.z_range.end,   rhs.z_range.end  ),
        };

        match ([x_range, y_range, z_range], !rhs.positive).try_into() {
            Ok(cuboid) => Some(cuboid),
            Err(()) => None,
        }
    }
}

impl TryFrom<([Range<i64>; 3], bool)> for Cuboid {
    type Error = ();

    fn try_from(data: ([Range<i64>; 3], bool)) -> Result<Self, Self::Error> {
        if data.0.iter().all(|range| range.start <= range.end) {
            let ([x_range, y_range, z_range], positive) = data;
            Ok(Self { x_range, y_range, z_range, positive })
        } else {
            Err(())
        }
    }
}

fn main() {
    let bootup = read_input("input.txt").unwrap();

    println!("Part 1: {}", part1(&bootup));
    println!("Part 2: {}", part2(&bootup));
}

fn part1(bootup: &Vec<Cuboid>) -> i64 {

    let mut blockchain = build_blockchain(bootup);
    let blocks_on = blockchain.iter().map(|cuboid| cuboid.volume()).sum::<i64>();

    let interior: Cuboid = (
        [
            Range { start: -50, end: 50 },
            Range { start: -50, end: 50 },
            Range { start: -50, end: 50 },
        ], false
    )
    .try_into()
    .unwrap();

    add_cuboid(&mut blockchain, &interior);
    let blocks_on_wo_interior = blockchain.iter().map(|cuboid| cuboid.volume()).sum::<i64>();

    blocks_on - blocks_on_wo_interior
}

fn part2(bootup: &Vec<Cuboid>) -> i64 {
    let blockchain = build_blockchain(bootup);
    blockchain.iter().map(|cuboid| cuboid.volume()).sum::<i64>()
}

fn build_blockchain(bootup: &Vec<Cuboid>) -> Vec<Cuboid> {
    let mut blockchain = Vec::with_capacity(1000);
    bootup.iter().for_each(|cuboid| add_cuboid(&mut blockchain, cuboid));
    blockchain
}

fn add_cuboid(blockchain: &mut Vec<Cuboid>, cuboid: &Cuboid) {

    let mut new_layers = blockchain.iter().map(|layer| {
        cuboid.intersection(layer)
    })
    .filter(|layer| layer.is_some())
    .map(|layer| layer.unwrap())
    .collect::<Vec<_>>();

    if cuboid.positive { new_layers.push(cuboid.clone()); }

    blockchain.append(&mut new_layers);
}


fn read_input(path: &str) -> Result<Vec<Cuboid>, ()> {

    let s = std::fs::read_to_string(path).unwrap();
    let mut cuboids = Vec::with_capacity(420);

    for line in s.lines() {
        let mut split = line.split(' ');
        let on_off = split.next().unwrap() == "on";
        let ranges: [Range<i64>; 3] = split.next().unwrap().split(',').map(|text| {
            let mut split = text.split("..");
            let start = split.next().unwrap()[2..].parse().unwrap();
            Range { start, end: split.next().unwrap().parse().unwrap() }
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

        cuboids.push((ranges, on_off).try_into()?);
    }

    Ok(cuboids)
}
