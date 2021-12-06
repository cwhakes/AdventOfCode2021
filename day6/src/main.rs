use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let answer = get_answer(&buf, 80);
    println!("{}", answer);

    let answer = get_answer(&buf, 256);
    println!("{}", answer);
}

fn get_answer(input: &str, days: usize) -> i64 {
    let mut jellies = VecDeque::from([0i64; 9]);
    for jelly in input.split(',').map(|s| s.trim().parse::<usize>().unwrap()) {
        jellies[jelly] += 1;
    }

    for _ in 0..days {
        let spawner_count = *jellies.get(0).unwrap();
        jellies.rotate_left(1);
        *jellies.get_mut(6).unwrap() += spawner_count;
    }

    jellies.iter().sum()
}
