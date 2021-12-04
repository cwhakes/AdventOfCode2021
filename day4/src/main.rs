use std::fs::File;
use std::io::Read;
use std::collections::BTreeSet;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let answer = get_answer1(&buf);
    println!("{}", answer);

    let answer = get_answer2(&buf);
    println!("{}", answer);
}

const WINS: [[usize; 5]; 12] = [
    [0, 1, 2, 3, 4],
    [5, 6, 7, 8, 9],
    [10, 11, 12, 13, 14],
    [15, 16, 17, 18, 19],
    [20, 21, 22, 23, 24],
    [0, 5, 10, 15, 20],
    [1, 6, 11, 16, 21],
    [2, 7, 12, 17, 22],
    [3, 8, 13, 18, 23],
    [4, 9, 14, 19, 24],
    [0, 6, 12, 18, 24],
    [4, 8, 12, 16, 20],
];

fn get_answer1(input: &str) -> i64 {
    let mut iter = input.split("\n\n");
    let nums = iter.next().unwrap().split(',').map(|n| n.parse::<i64>().unwrap());

    let mut boards = iter
        .map(|b| b.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .map(|board| (board, BTreeSet::<usize>::new()))
        .collect::<Vec<_>>();

    let mut winner = None;

    'outer: for num in nums {
        for (board, matches) in &mut boards {
            if let Some((idx, _)) = board.iter().enumerate().find(|(_,n)| **n==num) {
                matches.insert(idx);
            }
        }

        for (board, matches) in boards.iter() {
            for win in &WINS {
                if matches.is_superset(&BTreeSet::from(*win)) {
                    winner = Some((num, board.clone(), matches.clone()));
                    break 'outer;
                }
            }
        }

    }

    if let Some((num, board, matches)) = winner {
        let sum: i64 = board.into_iter().enumerate().filter(|(i, _)| !matches.contains(i)).map(|(_,n)| n).sum();
        sum * num
    } else {
        0
    }
}

fn get_answer2(input: &str) -> i64 {
    let mut iter = input.split("\n\n");
    let nums = iter.next().unwrap().split(',').map(|n| n.parse::<i64>().unwrap());

    let mut boards = iter
        .map(|b| b.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .map(|board| (board, BTreeSet::<usize>::new()))
        .collect::<Vec<_>>();

    let mut winner = None;

    'outer: for num in nums {
        for (board, matches) in &mut boards {
            if let Some((idx, _)) = board.iter().enumerate().find(|(_,n)| **n==num) {
                matches.insert(idx);
            }
        }

        winner = Some((num, boards[0].0.clone(), boards[0].1.clone()));

        boards = boards.drain(..).filter(|(_, matches)| {
            for win in &WINS {
                if matches.is_superset(&BTreeSet::from(*win)) {
                    return false;
                }
            }
            true
        }).collect();

        if boards.is_empty() {
            break 'outer;
        }
    }
    

    if let Some((num, board, matches)) = winner {
        let sum: i64 = board.into_iter().enumerate().filter(|(i, _)| !matches.contains(i)).map(|(_,n)| n).sum();
        sum * num
    } else {
        0
    }
}
