use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let set = get_answer(&buf, 1);
    println!("{}", set.len());

    let set = get_answer(&buf, usize::MAX);
    let max_x = *set.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *set.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if set.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn get_answer(input: &str, iters: usize) -> BTreeSet<(i32, i32)> {
    let (dots, folds) = input.split_once("\n\n").unwrap();

    let mut set = dots
        .lines()
        .map(|s| {
            let (x, y) = s.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<BTreeSet<(i32, i32)>>();

    for (ori, inst) in folds
        .lines()
        .take(iters)
        .map(|line| line.strip_prefix("fold along ").unwrap())
        .map(|line| line.split_once('=').unwrap())
    {
        let inst = inst.parse().unwrap();
        set = set
            .into_iter()
            .flat_map(|(x, y)| match (ori, x.cmp(&inst), y.cmp(&inst)) {
                ("x", Ordering::Less, _) | ("y", _, Ordering::Less) => Some((x, y)),
                ("x", Ordering::Greater, _) => Some((2 * inst - x, y)),
                ("y", _, Ordering::Greater) => Some((x, 2 * inst - y)),
                _ => None,
            })
            .collect();
    }
    set
}
