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
    
    let line_count = input.lines().count();
    let bit_count = input.lines().next().unwrap().chars().count();

    let mut bits = vec![0; bit_count];

    for line in input.lines() {
        for (idx, bit) in line.chars().enumerate() {
            if bit == '1' {
                bits[idx] += 1;
            }
        }
    }
    
    let mut gamma = 0;
    let mut espilon = 0;

    for bit in bits {
        gamma *= 2;
        espilon *= 2;

        if bit > line_count / 2 {
            gamma += 1;
        } else {
            espilon += 1;
        }
    }

    gamma * espilon
}

fn get_answer2(input: &str) -> i32 {

    let mut o2_values = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    let mut co2_values = o2_values.clone();

    for index in 0..o2_values[0].len() {
        if o2_values.len() > 1 {
            let bit_criteria = bit_criteria(&o2_values, index);
            o2_values = o2_values.drain(..).filter(|v| v[index] == bit_criteria).collect();
        }
        if co2_values.len() > 1 {
            let bit_criteria = bit_criteria(&co2_values, index);
            co2_values = co2_values.drain(..).filter(|v| v[index] != bit_criteria).collect();
        }
    }

    let mut o2_value = 0;
    for bit in &o2_values[0] {
        o2_value *= 2;
        if *bit == '1' {
            o2_value += 1;
        }
    }

    let mut co2_value = 0;
    for bit in &co2_values[0] {
        co2_value *= 2;
        if *bit == '1' {
            co2_value += 1;
        }
    }

    o2_value * co2_value
}

fn bit_criteria(input: &[Vec<char>], index: usize) -> char {
    let mut count = 0;
    for line in input {
        if line[index] == '1' {
            count += 1;
        }
    }

    //Offset in case of ties
    if count >= (input.len() + 1) / 2 {
        '1'
    } else {
        '0'
    }
}
