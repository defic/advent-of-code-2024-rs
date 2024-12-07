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

    let mut total = 0;
    for (answer, mut members) in problems {
        let mut accs: Vec<u64> = vec![members.remove(0)];

        for member in members {
            accs = accs
                .iter()
                .flat_map(|acc| vec![acc * member, acc + member, combine(*acc, member)])
                .collect()
        }
        if accs.contains(&answer) {
            total += answer;
        }
    }

    total.to_string()
}

fn parse_input(input: String) -> Vec<(u64, Vec<u64>)> {
    let problems: Vec<_> = input
        .lines()
        .map(|line| {
            let mut split = line.split(":");
            let answer = split.next().unwrap().parse::<u64>().unwrap();
            let mut members = split.next().unwrap().to_string();
            members.remove(0); //space
            let members: Vec<_> = members.split(" ").flat_map(|e| e.parse::<u64>()).collect();
            (answer, members)
        })
        .collect();
    problems
}

fn combine(num1: u64, num2: u64) -> u64 {
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
