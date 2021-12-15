use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let answer = get_answer(&buf, 10);
    println!("{}", answer);

    let answer = get_answer(&buf, 40);
    println!("{}", answer);
}

fn get_answer(input: &str, iterations: usize) -> usize {
    let mut iter = input.lines();
    let template: Vec<_> = iter.next().unwrap().chars().collect();
    let rules: BTreeMap<(char, char), char> = iter
        .skip(1)
        .map(|line| {
            let mut iter = line.chars();
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();
            let c = iter.nth(4).unwrap();
            ((a, b), c)
        })
        .collect();

    let first_char = template[0];

    let mut pair_counter: BTreeMap<_, usize> = BTreeMap::new();
    for w in template.windows(2) {
        *pair_counter.entry((w[0], w[1])).or_default() += 1;
    }

    for _ in 0..iterations {
        let mut new_pair_counter = BTreeMap::new();
        for ((a, b), count) in pair_counter.into_iter() {
            if let Some(&c) = rules.get(&(a, b)) {
                *new_pair_counter.entry((a, c)).or_default() += count;
                *new_pair_counter.entry((c, b)).or_default() += count;
            }
        }
        pair_counter = new_pair_counter;
    }

    let mut counter: BTreeMap<char, usize> = BTreeMap::new();
    *counter.entry(first_char).or_default() += 1usize;
    for ((_, b), count) in pair_counter {
        *counter.entry(b).or_default() += count;
    }

    counter.values().max().unwrap() - counter.values().min().unwrap()
}
