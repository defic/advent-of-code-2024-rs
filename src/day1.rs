use std::{
    env,
    io::{self, BufRead},
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first() {
        Some(val) if val == "1" => part1(),
        Some(val) if val == "2" => part2(),
        Some(_) => eprintln!("Invalid argument. Only 1 or 2 are accepted"),
        None => eprintln!("Provide argument 1 or 2"),
    }
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

fn get_input() -> Vec<String> {
    let stdin = io::stdin();
    let input_lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();
    input_lines
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
        total + elem * find_occurances(elem, &right)
    });
    println!("similarity_score: {}", similarity_score)
}

fn find_occurances(number: &i32, list: &[i32]) -> i32 {
    list.iter().fold(
        0,
        |count, item| if number == item { count + 1 } else { count },
    )
}
