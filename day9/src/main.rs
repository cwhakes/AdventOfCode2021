use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let answer = get_answer1(&buf);
    println!("{}", answer);

    let answer = get_answer2(&buf);
    println!("{}", answer);
}

fn get_answer1(input: &str) -> u32 {
    let cave = Cave::from_str(input);

    cave.low_points().map(|(_k, v)| v + 1).sum::<u32>()
}

fn get_answer2(input: &str) -> usize {
    let cave = Cave::from_str(input);

    let basins: BTreeSet<_> = cave
        .low_points()
        .map(|(k, _)| cave.basin_size(*k))
        .collect();
    basins.iter().rev().take(3).product()
}

struct Cave(BTreeMap<(i32, i32), u32>);

impl Cave {
    fn from_str(input: &str) -> Self {
        let cave = (input.lines().enumerate())
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, height)| {
                    let height = height.to_digit(10).unwrap();
                    ((x as i32, y as i32), height)
                })
            })
            .collect();

        Self(cave)
    }

    fn get(&self, (x, y): (i32, i32)) -> Option<u32> {
        self.0.get(&(x, y)).copied()
    }

    fn low_point_at(&self, (x, y): (i32, i32)) -> bool {
        let point = self.get((x, y)).unwrap();

        (self.get((x, y - 1)).into_iter())
            .chain(self.get((x - 1, y)))
            .chain(self.get((x + 1, y)))
            .chain(self.get((x, y + 1)))
            .all(|p| p > point)
    }

    fn low_points(&self) -> impl Iterator<Item = (&(i32, i32), &u32)> {
        self.0.iter().filter(|(k, _)| self.low_point_at(**k))
    }

    fn basin_size(&self, (x, y): (i32, i32)) -> usize {
        let mut basin = BTreeSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((x, y));

        while let Some((x, y)) = queue.pop_front() {
            if let Some(0..=8) = self.get((x, y)) {
                if basin.insert((x, y)) {
                    queue.push_back((x, y - 1));
                    queue.push_back((x - 1, y));
                    queue.push_back((x + 1, y));
                    queue.push_back((x, y + 1));
                }
            }
        }

        basin.len()
    }
}
