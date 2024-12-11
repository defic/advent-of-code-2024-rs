use std::{collections::HashMap, time::Instant};

use advent_of_code_2024::task_argument_with_input;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let time = Instant::now();
    let res = task_argument_with_input("inputs/11.txt", task1, task2);
    println!("Res: {}, Took: {:?}", res, time.elapsed());
}

fn parse_input(input: String) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|val| val.parse::<u64>().unwrap())
        .collect()
}

fn task1(input: String) -> String {
    let mut input = parse_input(input);
    println!("got input: {:?}", input);

    for x in 0..25 {
        input = solve1(input);
    }

    input.len().to_string()
}

//bruteforce
fn solve1(input: Vec<u64>) -> Vec<u64> {
    input
        .iter()
        .flat_map(|val| match val {
            0 => vec![1],
            x if (x.ilog10() + 1) % 2 == 0 => {
                let (left, right) = split_number(*x);
                vec![left, right]
            }
            _ => vec![val * 2024],
        })
        .collect()
}

fn split_number(num: u64) -> (u64, u64) {
    let digits = num.ilog10() + 1;
    let mid = digits / 2;
    let divisor = 10_u64.pow(mid);

    let right = num % divisor;
    let left = num / divisor;

    (left, right)
}

fn task2(input: String) -> String {
    let input = parse_input(input);
    println!("got input: {:?}", input);

    let mut counter: HashMap<u64, usize> = HashMap::new();

    for stone in &input {
        counter.insert(*stone, 1);
    }

    for _ in 0..75 {
        let mut new_counter: HashMap<u64, usize> = HashMap::with_capacity(counter.capacity());
        counter.iter().for_each(|(val, og_count)| {
            let stones = match val {
                0 => vec![1],
                x if (x.ilog10() + 1) % 2 == 0 => {
                    let (left, right) = split_number(*x);
                    vec![left, right]
                }
                val => vec![val * 2024],
            };

            for stone in stones {
                *new_counter.entry(stone).or_insert(0) += og_count;
            }
        });
        counter = new_counter;
    }
    let res: usize = counter.values().sum();
    res.to_string()
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve1};

    const INPUT: &str = "0 1 10 99 999";
    #[test]
    fn test() {
        let input = parse_input(INPUT.to_string());
        let res = solve1(input);
        println!("res: {:?}", res)
    }

    #[test]
    fn test2() {
        let mut input = parse_input("125 17".to_string());

        for x in 0..6 {
            input = solve1(input);
        }

        let res = input
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        assert_eq!(
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2",
            &res
        );

        for x in 6..25 {
            input = solve1(input);
        }

        assert_eq!(55312, input.len());
    }
}
