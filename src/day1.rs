use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input_lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let input: Vec<_> = input_lines
        .iter()
        .map(|line| {
            let mut elements = line.split_ascii_whitespace();
            (elements.next().unwrap(), elements.next().unwrap())
        })
        .collect();
    let mut left = Vec::new();
    let mut right = Vec::new();

    input.into_iter().for_each(|tuple| {
        left.push(tuple.0.parse::<i32>().unwrap());
        right.push(tuple.1.parse::<i32>().unwrap());
    });

    left.sort();
    right.sort();

    dbg!(&left);
    dbg!(&right);

    let res = left
        .into_iter()
        .zip(right)
        .fold(0, |acc, tuple| acc + (tuple.0 - tuple.1).abs());
    println!("res: {}", res)
}
