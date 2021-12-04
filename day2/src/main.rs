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

fn get_answer1(input: &str) -> i32 {
    let mut depth =  0;
    let mut distance = 0;
    
    for instruction in input.lines() {
        match parse_instruction(instruction).unwrap() {
            Direction::Forward(len) => distance += len,
            Direction::Down(len) => depth += len,
            Direction::Up(len) => depth -= len,
        }
    }
    
    depth * distance
}

fn get_answer2(input: &str) -> i32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut distance = 0;
    
    for instruction in input.lines() {
        match parse_instruction(instruction).unwrap() {
            Direction::Forward(len) => {
                distance += len;
                depth += aim * len;
            },
            Direction::Down(len) => aim += len,
            Direction::Up(len) => aim -= len,
        }
    }
    
    depth * distance
}


enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse_instruction(input: &str) -> Option<Direction> {
    let mut iter = input.split(' ');
    let dir = iter.next()?;
    let len = iter.next()?.parse::<i32>().ok()?;
    match dir {
        "forward" => Some(Direction::Forward(len)),
        "down" => Some(Direction::Down(len)),
        "up" => Some(Direction::Up(len)),
        _ => None,
    }
}
