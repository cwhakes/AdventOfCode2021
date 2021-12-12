use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

use petgraph::graphmap::UnGraphMap;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let answer = get_answer(&buf, false);
    println!("{}", answer);

    let answer = get_answer(&buf, true);
    println!("{}", answer);
}

fn get_answer(input: &str, visit_twice: bool) -> usize {
    let map = UnGraphMap::from_edges(input.lines().map(|line| {
        let (a, b) = line.split_once('-').unwrap();
        (Cave::new(a), Cave::new(b), ())
    }));

    let mut queue = VecDeque::from([(vec![Cave::START], true)]);
    let mut output = Vec::new();

    while let Some(path) = queue.pop_front() {
        let current_cave = path.0.last().unwrap();
        for next_cave in map.neighbors(*current_cave) {
            let mut new_path = if next_cave.is_big || !path.0.contains(&next_cave) {
                path.clone()
            } else if visit_twice && path.1 && !matches!(next_cave, Cave::START | Cave::END) {
                (path.0.clone(), false)
            } else {
                continue;
            };

            new_path.0.extend([next_cave]);
            if next_cave == Cave::END {
                output.push(new_path);
            } else {
                queue.push_back(new_path);
            }
        }
    }

    output.len()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Cave<'a> {
    name: &'a str,
    is_big: bool,
}

impl<'a> Cave<'a> {
    const START: Self = Self {
        name: "start",
        is_big: false,
    };
    const END: Self = Self {
        name: "end",
        is_big: false,
    };

    fn new(input: &'a str) -> Self {
        Self {
            name: input,
            is_big: input.chars().all(char::is_uppercase),
        }
    }
}
