use std::collections::{BTreeSet, VecDeque};
use std::fs::File;
use std::io::Read;
use std::ops;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let scanners = get_scanners(&buf);
    let (scanners, probes) = get_map(scanners);
    println!("{}", probes.len());

    println!("{}", longest_distance(scanners));
}

fn get_scanners(input: &str) -> impl Iterator<Item = Vec<Coord>> + '_ {
    input.split("--- scanner ").skip(1).map(|scan| {
        scan.trim()
            .lines()
            .skip(1)
            .flat_map(Coord::from_str)
            .collect::<Vec<_>>()
    })
}

fn get_map(mut scanners: impl Iterator<Item = Vec<Coord>>) -> (BTreeSet<Coord>, BTreeSet<Coord>) {
    let mut probes = BTreeSet::new();
    probes.extend(scanners.next().unwrap());
    let mut queue = VecDeque::new();
    queue.extend(scanners);

    let mut scanners = BTreeSet::new();
    'queue: while let Some(scanner) = queue.pop_front() {
        for known in probes.clone().iter() {
            for r in 0..24 {
                for unknown in scanner.iter() {
                    let count = scanner
                        .iter()
                        .map(|probe| (*probe - *unknown).rotate(r) + *known)
                        .filter(|probe| probes.contains(probe))
                        .count();

                    if count >= 12 {
                        scanners.insert((Coord::new(0, 0, 0) - *unknown).rotate(r) + *known);

                        probes.extend(
                            scanner
                                .iter()
                                .map(|probe| (*probe - *unknown).rotate(r) + *known),
                        );
                        continue 'queue;
                    }
                }
            }
        }

        queue.push_back(scanner);
    }

    (scanners, probes)
}

fn longest_distance(map: BTreeSet<Coord>) -> i64 {
    let mut max = 0;
    for a in map.iter() {
        for b in map.iter() {
            max = max.max((a.x - b.x) + (a.y - b.y) + (a.z - b.z));
        }
    }
    max
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn from_str(input: &str) -> Option<Self> {
        let mut iter = input.trim().split(',');
        let x = iter.next()?.parse().ok()?;
        let y = iter.next()?.parse().ok()?;
        let z = iter.next()?.parse().ok()?;
        Some(Self::new(x, y, z))
    }

    fn rotate(self, n: u8) -> Self {
        let Self { x, y, z } = self;

        match n {
            0 => Self::new(x, y, z),
            1 => Self::new(x, z, -y),
            2 => Self::new(x, -y, -z),
            3 => Self::new(x, -z, y),

            4 => Self::new(-x, y, -z),
            5 => Self::new(-x, -z, -y),
            6 => Self::new(-x, -y, z),
            7 => Self::new(-x, z, y),

            8 => Self::new(y, z, x),
            9 => Self::new(y, x, -z),
            10 => Self::new(y, -z, -x),
            11 => Self::new(y, -x, z),

            12 => Self::new(-y, z, -x),
            13 => Self::new(-y, -x, -z),
            14 => Self::new(-y, -z, x),
            15 => Self::new(-y, x, z),

            16 => Self::new(z, x, y),
            17 => Self::new(z, y, -x),
            18 => Self::new(z, -x, -y),
            19 => Self::new(z, -y, x),

            20 => Self::new(-z, x, -y),
            21 => Self::new(-z, -y, -x),
            22 => Self::new(-z, -x, y),
            23 => Self::new(-z, y, x),

            _ => panic!("Rotation out of bounds"),
        }
    }
}

impl ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
