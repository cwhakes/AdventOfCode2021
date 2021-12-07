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

fn get_answer1(input: &str) -> i64 {
    let crabs: Vec<_> = input.split(',').map(|s| s.trim().parse::<i64>().unwrap()).collect();


    (0..=*crabs.iter().max().unwrap())
        .map(|target| crabs.iter().map(|d| (d-target).abs()).sum())
        .min()
        .unwrap()
}

fn get_answer2(input: &str) -> i64 {
    let crabs: Vec<_> = input.split(',').map(|s| s.trim().parse::<i64>().unwrap()).collect();


    (0..=*crabs.iter().max().unwrap())
        .map(|target| crabs.iter().map(|d| (((d-target).pow(2)+(d-target).abs())/2)).sum())
        .min()
        .unwrap()
}
