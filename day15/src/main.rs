use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let risk_map = RiskMap::new1(&buf);
    let answer = get_answer(risk_map);
    println!("{}", answer);

    let risk_map = RiskMap::new2(&buf);
    let answer = get_answer(risk_map);
    println!("{}", answer);
}

fn get_answer(mut risk_map: RiskMap) -> u32 {
    let mut total_risks = BTreeMap::new();
    total_risks.insert((0, 0), 0);

    while let Some((&coord, &total_risk)) = total_risks.iter().min_by_key(|(_, &r)| r) {
        if coord == (risk_map.x_len - 1, risk_map.y_len - 1) {
            return total_risk;
        }

        total_risks.remove(&coord);
        risk_map.map.get_mut(&coord).unwrap().1 = true;

        let (x, y) = coord;
        for coord in [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)] {
            if let Some((risk, false)) = risk_map.map.get(&coord) {
                let new_risk = total_risk + risk;
                if let Some(old_risk) = total_risks.get_mut(&coord) {
                    *old_risk = new_risk.min(*old_risk);
                } else {
                    total_risks.insert(coord, new_risk);
                }
            }
        }
    }

    0
}

struct RiskMap {
    map: BTreeMap<(i32, i32), (u32, bool)>,
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
                    .map(move |(x, c)| ((x as i32, y as i32), (c.to_digit(10).unwrap(), false)))
            })
            .collect();

        Self { map, x_len, y_len }
    }

    fn new2(input: &str) -> Self {
        let Self { map, x_len, y_len } = Self::new1(input);

        let mut new_risk_map = BTreeMap::new();

        for x in 0..5 {
            for y in 0..5 {
                new_risk_map.extend(map.iter().map(|((x_0, y_0), (risk, _))| {
                    (
                        (x_0 + x * x_len, y_0 + y * y_len),
                        ((risk + x as u32 + y as u32 - 1) % 9 + 1, false),
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
