use std::time::Instant;

use advent_of_code_2024::task_argument_with_input;

fn main() {
    let time = Instant::now();
    let res = task_argument_with_input("inputs/day7.txt", task1, task2);
    println!("Res: {}, Took: {:?}", res, time.elapsed());
}

fn task1(input: String) -> String {
    let problems = parse_input(input);

    let mut total = 0;
    for (answer, mut members) in problems {
        let mut accs: Vec<u64> = vec![members.remove(0)];

        for member in members {
            accs = accs
                .iter()
                .flat_map(|acc| vec![acc * member, acc + member])
                .collect()
        }
        if accs.contains(&answer) {
            total += answer;
        }
    }

    total.to_string()
}

fn task2(input: String) -> String {
    let problems = parse_input(input);
    let operators = &[Operator::Add, Operator::Multiply, Operator::Concat];
    problems
        .iter()
        .filter(|(goal, members)| has_solution2(operators, *goal, members[0], &members[1..]))
        .map(|c| c.0)
        .sum::<u64>()
        .to_string()
}

fn has_solution2(operators: &[Operator], goal: u64, current: u64, rest: &[u64]) -> bool {
    if rest.is_empty() {
        return goal == current;
    }

    if current > goal {
        return false;
    }

    let first = rest.first().unwrap();

    operators
        .iter()
        .any(|op| has_solution2(operators, goal, op.calc(current, *first), &rest[1..]))
}

enum Operator {
    Multiply,
    Add,
    Concat,
}

impl Operator {
    fn calc(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concat => concat(a, b),
        }
    }
}

fn parse_input(input: String) -> Vec<(u64, Vec<u64>)> {
    let problems: Vec<_> = input
        .lines()
        .map(|line| {
            let (answer, rest) = line.split_once(":").unwrap();
            let answer = answer.parse::<u64>().unwrap();
            let members: Vec<_> = rest
                .split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            (answer, members)
        })
        .collect();
    problems
}

fn concat(num1: u64, num2: u64) -> u64 {
    let num2_len = count_digits(num2);
    let multiplier = 10_u64.pow(num2_len);
    num1 * multiplier + num2
}

fn count_digits(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n != 0 {
        n /= 10;
        count += 1;
    }
    count
}
