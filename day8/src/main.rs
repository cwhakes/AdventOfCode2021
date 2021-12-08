use std::collections::BTreeSet;
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
    let lines = input.lines();
    lines
        .map(|line| {
            Line::from_str(line)
                .output
                .iter()
                .filter(|d| matches!(d.len(), 2 | 4 | 3 | 7))
                .count()
        })
        .sum::<usize>() as i32
}

fn get_answer2(input: &str) -> i32 {
    let lines = input.lines();
    lines
        .map(|line| Line::from_str(line).analyze().output())
        .sum::<i32>() as i32
}

struct Line {
    input: Vec<BTreeSet<char>>,
    output: Vec<BTreeSet<char>>,
    digits: Vec<BTreeSet<char>>,
}

impl Line {
    fn from_str(s: &str) -> Self {
        let (input, output) = s.split_once('|').unwrap();
        Self {
            input: input
                .split_whitespace()
                .map(|d| d.chars().collect())
                .collect(),
            output: output
                .split_whitespace()
                .map(|d| d.chars().collect())
                .collect(),
            digits: vec![BTreeSet::new(); 10],
        }
    }

    fn output(&self) -> i32 {
        self.output
            .iter()
            .map(|d| {
                self.digits
                    .iter()
                    .enumerate()
                    .find(|(_, x)| &d == x)
                    .unwrap()
                    .0
            })
            .fold(0, |acc, dig| acc * 10 + dig as i32)
    }

    fn raw_digits(&self) -> impl Iterator<Item = &BTreeSet<char>> {
        self.input.iter().chain(&self.output)
    }

    fn analyze(&mut self) -> &Self {
        self.digits[8] = "abcdefg".chars().collect();

        let one = self.raw_digits().find(|d| d.len() == 2);
        if let Some(one) = one {
            self.digits[1] = one.clone();
        }

        let seven = self.raw_digits().find(|d| d.len() == 3);
        if let Some(seven) = seven {
            self.digits[7] = seven.clone();
        }

        let four = self.raw_digits().find(|d| d.len() == 4);
        if let Some(four) = four {
            self.digits[4] = four.clone();
        }

        assert!(!self.digits[4].is_empty());
        let nine = self
            .raw_digits()
            .filter(|d| d.len() == 6)
            .find(|d| d.is_superset(&self.digits[4]));
        if let Some(nine) = nine {
            self.digits[9] = nine.clone();
        }

        assert!(!self.digits[1].is_empty());
        let zero = self
            .raw_digits()
            .filter(|d| d.len() == 6)
            .find(|d| d.is_superset(&self.digits[1]) && **d != self.digits[9]);
        if let Some(zero) = zero {
            self.digits[0] = zero.clone();
        }

        let six = self
            .raw_digits()
            .filter(|d| d.len() == 6)
            .find(|d| **d != self.digits[9] && **d != self.digits[0]);
        if let Some(six) = six {
            self.digits[6] = six.clone();
        }

        let three = self
            .raw_digits()
            .filter(|d| d.len() == 5)
            .find(|d| d.is_superset(&self.digits[1]) && d.is_superset(&self.digits[7]));
        if let Some(three) = three {
            self.digits[3] = three.clone();
        }

        assert!(!self.digits[6].is_empty());
        let five = self
            .raw_digits()
            .filter(|d| d.len() == 5)
            .find(|d| d.is_subset(&self.digits[6]));
        if let Some(five) = five {
            self.digits[5] = five.clone();
        }

        let two = self
            .raw_digits()
            .filter(|d| d.len() == 5)
            .find(|d| **d != self.digits[3] && **d != self.digits[5]);
        if let Some(two) = two {
            self.digits[2] = two.clone();
        }

        &*self
    }
}
