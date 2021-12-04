use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;

use lazy_static::lazy_static;

lazy_static! {
    static ref WINNING_COMBOS: [BTreeSet<usize>; 12] = [
        BTreeSet::from([0, 1, 2, 3, 4]),
        BTreeSet::from([5, 6, 7, 8, 9]),
        BTreeSet::from([10, 11, 12, 13, 14]),
        BTreeSet::from([15, 16, 17, 18, 19]),
        BTreeSet::from([20, 21, 22, 23, 24]),
        BTreeSet::from([0, 5, 10, 15, 20]),
        BTreeSet::from([1, 6, 11, 16, 21]),
        BTreeSet::from([2, 7, 12, 17, 22]),
        BTreeSet::from([3, 8, 13, 18, 23]),
        BTreeSet::from([4, 9, 14, 19, 24]),
        BTreeSet::from([0, 6, 12, 18, 24]),
        BTreeSet::from([4, 8, 12, 16, 20]),
    ];
}

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
    let (nums, mut boards) = parse_nums_and_boards(input);
    let mut winner = None;

    'outer: for num in nums {
        for (board, matches) in &mut boards {
            if let Some((idx, _)) = board.iter().enumerate().find(|(_, n)| **n == num) {
                matches.insert(idx);
            }
        }

        for (board, matches) in boards.iter() {
            for combo in WINNING_COMBOS.iter() {
                if matches.is_superset(combo) {
                    winner = Some((num, board.clone(), matches.clone()));
                    break 'outer;
                }
            }
        }
    }

    if let Some((num, board, matches)) = winner {
        num * score_board(&board, &matches)
    } else {
        0
    }
}

fn get_answer2(input: &str) -> i64 {
    let (nums, mut boards) = parse_nums_and_boards(input);
    let mut winner = None;

    'outer: for num in nums {
        for (board, matches) in &mut boards {
            if let Some((idx, _)) = board.iter().enumerate().find(|(_, n)| **n == num) {
                matches.insert(idx);
            }
        }

        winner = Some((num, boards[0].0.clone(), boards[0].1.clone()));

        boards = boards
            .drain(..)
            .filter(|(_, matches)| {
                for combo in WINNING_COMBOS.iter() {
                    if matches.is_superset(combo) {
                        return false;
                    }
                }
                true
            })
            .collect();

        if boards.is_empty() {
            break 'outer;
        }
    }

    if let Some((num, board, matches)) = winner {
        num * score_board(&board, &matches)
    } else {
        0
    }
}

fn parse_nums_and_boards(
    input: &str,
) -> (
    impl Iterator<Item = i64> + '_,
    Vec<(Vec<i64>, BTreeSet<usize>)>,
) {
    let mut iter = input.split("\n\n");
    let nums = iter
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i64>().unwrap());

    let boards = iter
        .map(|b| {
            b.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|board| (board, BTreeSet::<usize>::new()))
        .collect::<Vec<_>>();

    (nums, boards)
}

fn score_board(board: &[i64], matches: &BTreeSet<usize>) -> i64 {
    board
        .iter()
        .enumerate()
        .filter(|(i, _)| !matches.contains(i))
        .map(|(_, n)| n)
        .sum()
}
