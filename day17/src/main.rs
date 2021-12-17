use std::fs::File;
use std::io::Read;
use std::ops::RangeInclusive;

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
    let target = get_target(input);
    // Completely unnecessary, but I thought part 2 would be harder
    let x_vel = quadratic(*target.0.end());

    let (y_max, _y_vel) = (0..2000)
        .filter_map(|y_vel| {
            if let (true, y_max) = hits_target((x_vel, y_vel), target.clone()) {
                Some((y_max, y_vel))
            } else {
                None
            }
        })
        .max()
        .unwrap();

    y_max
}

fn get_answer2(input: &str) -> usize {
    let target = get_target(input);

    (0..=200)
        .flat_map(|x_vel| (-200..200).map(move |y_vel| (x_vel, y_vel)))
        .filter(|vel| hits_target(*vel, target.clone()).0)
        .count()
}

fn get_target(input: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let input = input.strip_prefix("target area: x=").unwrap();
    let (x_range, y_range) = input.trim().split_once(", y=").unwrap();
    let (x0, x1) = x_range.split_once("..").unwrap();
    let (y0, y1) = y_range.split_once("..").unwrap();
    let x0 = x0.parse::<i32>().unwrap();
    let x1 = x1.parse::<i32>().unwrap();
    let y0 = y0.parse::<i32>().unwrap();
    let y1 = y1.parse::<i32>().unwrap();

    (x0..=x1, y0..=y1)
}

fn hits_target(
    (mut x_vel, mut y_vel): (i32, i32),
    range: (RangeInclusive<i32>, RangeInclusive<i32>),
) -> (bool, i32) {
    let (mut x_loc, mut y_loc) = (0, 0);
    let mut y_max = 0;
    while (x_vel != 0 && x_loc <= *range.0.end()) || y_loc >= *range.1.start() {
        x_loc += x_vel;
        y_loc += y_vel;
        y_max = y_max.max(y_loc);

        x_vel = (x_vel - 1).max(0);
        y_vel -= 1;

        if range.0.contains(&x_loc) && range.1.contains(&y_loc) {
            return (true, y_max);
        }
    }

    (false, y_max)
}

fn quadratic(c: i32) -> i32 {
    (-0.5 + (0.25 + 2.0 * c as f32).sqrt()) as i32
}
