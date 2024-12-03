use std::{
    env,
    io::{self, BufRead},
};

use advent_of_code_2024::{get_input, task_argument};

fn main() {
    task_argument(part1, part2);
}

fn input_to_integer_vecs(input_lines: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    input_lines
        .iter()
        .map(|line| {
            let mut elements = line.split_ascii_whitespace();
            let left = elements.next().unwrap().parse::<i32>().unwrap();
            let right = elements.next().unwrap().parse::<i32>().unwrap();
            (left, right)
        })
        .unzip()
}

fn part1() {
    let input_lines = get_input();
    let (mut left, mut right) = input_to_integer_vecs(input_lines);

    left.sort();
    right.sort();

    let res = left
        .into_iter()
        .zip(right)
        .fold(0, |acc, tuple| acc + (tuple.0 - tuple.1).abs());
    println!("res: {}", res)
}

fn part2() {
    let input_lines = get_input();
    let (left, right) = input_to_integer_vecs(input_lines);

    let similarity_score = left.iter().fold(0, |total, elem| {
        total
            + elem
                * right
                    .iter()
                    .filter(|right_elem| elem == *right_elem)
                    .count() as i32
    });
    println!("similarity_score: {}", similarity_score)
}
