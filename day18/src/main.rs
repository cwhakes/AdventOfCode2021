use std::fmt::{self, Display};
use std::fs::File;
use std::io::Read;
use std::ops::Add;

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
    let sum = input
        .lines()
        .map(Snailfish::from_str)
        .reduce(Snailfish::add)
        .unwrap();
    //println!("{}", sum);
    sum.magnitude()
}

fn get_answer2(input: &str) -> i64 {
    let snailfishes = input.lines().map(Snailfish::from_str).collect::<Vec<_>>();
    (0..snailfishes.len())
        .flat_map(|x| (0..snailfishes.len()).map(move |y| (x, y)))
        .filter(|(x, y)| x != y)
        .map(|(x, y)| (snailfishes[x].clone() + snailfishes[y].clone()).magnitude())
        .max()
        .unwrap()
}

#[derive(Clone)]
enum Snailfish {
    Pair(Box<Snailfish>, Box<Snailfish>),
    Literal(i64),
}

impl Snailfish {
    fn from_str(input: &str) -> Self {
        let (_, mut snailfish) = nom::snailfish(input).unwrap();
        snailfish.reduce();
        snailfish
    }

    fn new_pair(a: i64, b: i64) -> Self {
        Self::Pair(Box::new(Self::Literal(a)), Box::new(Self::Literal(b)))
    }

    fn reduce(&mut self) {
        while self.explode(1).is_some() || self.split() {}
    }

    fn explode(&mut self, level: usize) -> Option<(i64, i64)> {
        if let Self::Pair(left, right) = self {
            if level > 4 {
                if let (&Self::Literal(a), &Self::Literal(b)) = (&**left, &**right) {
                    *self = Self::Literal(0);
                    return Some((a, b));
                }
            }

            if let Some((a, b)) = left.explode(level + 1) {
                right.add_left(b);
                return Some((a, 0));
            }

            if let Some((a, b)) = right.explode(level + 1) {
                left.add_right(a);
                return Some((0, b));
            }
        }
        None
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Pair(a, b) => a.split() || b.split(),
            Self::Literal(n) => {
                if *n > 9 {
                    let a = *n / 2;
                    let b = (*n as f32 / 2.0).round() as i64;
                    *self = Self::new_pair(a, b);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn add_left(&mut self, n: i64) {
        match self {
            Self::Literal(num) => *num += n,
            Self::Pair(left, _) => left.add_left(n),
        }
    }

    fn add_right(&mut self, n: i64) {
        match self {
            Self::Literal(num) => *num += n,
            Self::Pair(_, right) => right.add_right(n),
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            Self::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
            Self::Literal(n) => *n,
        }
    }
}

impl Add for Snailfish {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut out = Self::Pair(Box::new(self), Box::new(other));
        out.reduce();
        out
    }
}

impl Display for Snailfish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pair(a, b) => write!(f, "[{}, {}]", a, b),
            Self::Literal(n) => write!(f, "{}", n),
        }
    }
}

mod nom {
    use super::Snailfish;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::i64,
        combinator::{cut, map},
        sequence::{preceded, separated_pair, terminated},
        IResult,
    };

    pub(super) fn snailfish(s: &str) -> IResult<&str, Snailfish> {
        alt((snailfish_pair, snailfish_literal))(s)
    }

    fn snailfish_pair(s: &str) -> IResult<&str, Snailfish> {
        map(
            preceded(
                tag("["),
                cut(terminated(
                    separated_pair(snailfish, tag(","), snailfish),
                    tag("]"),
                )),
            ),
            |(left, right)| Snailfish::Pair(Box::new(left), Box::new(right)),
        )(s)
    }

    fn snailfish_literal(s: &str) -> IResult<&str, Snailfish> {
        map(i64, Snailfish::Literal)(s)
    }
}
