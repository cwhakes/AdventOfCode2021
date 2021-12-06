use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let answer = get_answer(&buf, false);
    println!("{}", answer);

    let answer = get_answer(&buf, true);
    println!("{}", answer);
}

fn get_answer(input: &str, include_diagonals: bool) -> i32 {
    let segments: Vec<_> = input.lines().flat_map(Segment::from_str).collect();

    let x_max = segments.iter().map(|s| s.x1.max(s.x2)).max().unwrap();
    let y_max = segments.iter().map(|s| s.y1.max(s.y2)).max().unwrap();

    let mut count = 0;
    for x in 0..=x_max {
        for y in 0..=y_max {
            if 2 <= segments
                .iter()
                .filter(|s| s.contains((x, y), include_diagonals))
                .count()
            {
                count += 1;
            }
        }
    }

    count
}

#[derive(Debug)]
struct Segment {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Segment {
    fn from_str(input: &str) -> Option<Self> {
        let (x1, input) = input.split_once(',')?;
        let (y1, input) = input.split_once(" -> ")?;
        let (x2, y2) = input.split_once(',')?;

        Some(Self {
            x1: x1.parse().ok()?,
            y1: y1.parse().ok()?,
            x2: x2.parse().ok()?,
            y2: y2.parse().ok()?,
        })
    }

    fn contains(&self, (x, y): (i32, i32), include_diagonals: bool) -> bool {
        fn within_span(x: i32, (x1, x2): (i32, i32)) -> bool {
            (x1..=x2).contains(&x) || (x2..=x1).contains(&x)
        }

        let Self { x1, y1, x2, y2 } = *self;
        (within_span(y, (y1, y2)) && within_span(x, (x1, x2)))
            && ((x == x1 && x == x2)
                || (y == y1 && y == y2)
                || (include_diagonals
                    && (x - x1).abs() == (y - y1).abs()
                    && (x - x2).abs() == (y - y2).abs()))
    }
}
