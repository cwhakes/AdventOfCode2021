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
    input
        .lines()
        .map(|line| {
            let mut stack = Vec::new();
            for (brace, close) in line.chars().map(Brace::new) {
                if close {
                    if Some(brace) != stack.pop() {
                        return match brace {
                            Brace::Angle => 25137,
                            Brace::Curly => 1197,
                            Brace::Paren => 3,
                            Brace::Square => 57,
                        };
                    }
                } else {
                    stack.push(brace);
                }
            }
            0
        })
        .sum()
}

fn get_answer2(input: &str) -> i64 {
    let mut output: Vec<_> = input
        .lines()
        .map(|line| {
            let mut stack = Vec::new();
            for (brace, close) in line.chars().map(Brace::new) {
                if close {
                    if Some(brace) != stack.pop() {
                        return 0;
                    }
                } else {
                    stack.push(brace);
                }
            }
            stack.into_iter().rev().fold(0, |acc, brace| {
                acc * 5
                    + match brace {
                        Brace::Angle => 4,
                        Brace::Curly => 3,
                        Brace::Paren => 1,
                        Brace::Square => 2,
                    }
            })
        })
        .filter(|x| *x != 0)
        .collect();
    output.sort();
    output[output.len() / 2]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Brace {
    Angle,
    Curly,
    Paren,
    Square,
}

impl Brace {
    fn new(c: char) -> (Self, bool) {
        match c {
            '<' => (Self::Angle, false),
            '{' => (Self::Curly, false),
            '(' => (Self::Paren, false),
            '[' => (Self::Square, false),
            '>' => (Self::Angle, true),
            '}' => (Self::Curly, true),
            ')' => (Self::Paren, true),
            ']' => (Self::Square, true),
            c => panic!("{}", c),
        }
    }
}
