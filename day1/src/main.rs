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

fn get_answer1(input: &str) -> usize {
    let mut iter = input.lines().map(|s| s.parse::<i32>().unwrap());
    let mut old = iter.next().unwrap();

    let mut count = 0;

    for depth in iter {
        if depth > old  {
            count += 1;
        }
        old = depth;
    }

    count
}

fn get_answer2(input: &str) -> usize {
    let mut iter = input.lines().map(|s| s.parse::<i32>().unwrap());
    let mut a = iter.next().unwrap();
    let mut b = iter.next().unwrap();
    let mut c = iter.next().unwrap();

    let mut old = a + b + c; 

    let mut count = 0;

    for depth in iter {
        a = b;
        b = c;
        c = depth;

        let new = a + b + c;

        if new > old  {
            count += 1;
        }
        old = new;
    }

    count
}
