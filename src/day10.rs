use std::time::Instant;

use advent_of_code_2024::task_argument_with_input;
use nom::InputIter;

fn main() {
    let time = Instant::now();
    let res = task_argument_with_input("inputs/10.txt", task1, task2);
    println!("Res: {}, Took: {:?}", res, time.elapsed());
}

fn parse_input(input: String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.iter_elements()
                .map(|e| e.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn task1(input: String) -> String {
    let map = parse_input(input);

    let count: u32 = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, _)| {
                    let mut vec = check(&map, 0, y as i32, x as i32);
                    vec.sort();
                    vec.dedup();
                    vec.len() as u32
                })
                .sum::<u32>()
        })
        .sum();

    count.to_string()
}

fn task2(input: String) -> String {
    let map = parse_input(input);

    let count: u32 = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, _)| check(&map, 0, y as i32, x as i32).len() as u32)
                .sum::<u32>()
        })
        .sum();

    count.to_string()
}

fn check(map: &[Vec<u32>], level: u32, y: i32, x: i32) -> Vec<(i32, i32)> {
    if out_of_bounds(map, y, x) {
        return Vec::new();
    }
    if map[y as usize][x as usize] != level || level > 9 {
        return Vec::new();
    }
    if level == 9 {
        return vec![(y, x)];
    };

    DIRECTIONS
        .iter()
        .flat_map(|(y_offset, x_offset)| check(map, level + 1, y + y_offset, x + x_offset))
        .collect()
}

fn out_of_bounds(map: &[Vec<u32>], y: i32, x: i32) -> bool {
    y >= map.len() as i32 || y < 0 || x < 0 || x >= map.first().unwrap().len() as i32
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[cfg(test)]
mod tests {
    use crate::{task1, task2};

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_task1() {
        let res = task1(INPUT.to_string());
        println!("Res: {}", res);
        assert_eq!("36", res);
    }

    #[test]
    fn test_task2() {
        let res = task2(INPUT.to_string());
        println!("Res: {}", res);
        assert_eq!("81", res);
    }
}
