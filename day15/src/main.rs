use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let risk_map = RiskMap::new1(&buf);
    let answer = get_answer(&risk_map);
    println!("{}", answer);

    let risk_map = RiskMap::new2(&buf);
    let answer = get_answer(&risk_map);
    println!("{}", answer);
}

fn get_answer(risk_map: &RiskMap) -> u32 {
    let mut visited = BTreeMap::new();
    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse((0, (0, 0))));

    while let Some(Reverse((total_risk, coord))) = frontier.pop() {
        // `frontier` is non-decreasing
        // the first value we see is the best
        if visited.contains_key(&coord) {
            continue;
        } else {
            visited.insert(coord, total_risk);
        }

        if coord == (risk_map.x_len - 1, risk_map.y_len - 1) {
            return total_risk;
        }

        let (x, y) = coord;
        for coord in [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)] {
            // This check is not strictly necessary
            // It just saves us a push
            if !visited.contains_key(&coord) {
                if let Some(risk) = risk_map.map.get(&coord) {
                    frontier.push(Reverse((total_risk + risk, coord)));
                }
            }
        }
    }

    0
}

struct RiskMap {
    map: BTreeMap<(i32, i32), u32>,
    x_len: i32,
    y_len: i32,
}

impl RiskMap {
    fn new1(input: &str) -> Self {
        let x_len = input.lines().next().unwrap().len() as i32;
        let y_len = input.lines().count() as i32;

        let map = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as i32, y as i32), c.to_digit(10).unwrap()))
            })
            .collect();

        Self { map, x_len, y_len }
    }

    fn new2(input: &str) -> Self {
        let Self { map, x_len, y_len } = Self::new1(input);

        let mut new_risk_map = BTreeMap::new();

        for x in 0..5 {
            for y in 0..5 {
                new_risk_map.extend(map.iter().map(|((x_0, y_0), risk)| {
                    (
                        (x_0 + x * x_len, y_0 + y * y_len),
                        (risk + x as u32 + y as u32 - 1) % 9 + 1,
                    )
                }));
            }
        }

        Self {
            map: new_risk_map,
            x_len: x_len * 5,
            y_len: y_len * 5,
        }
    }
}
