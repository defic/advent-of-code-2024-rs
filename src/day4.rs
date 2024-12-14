use std::time::{self, Instant};

use advent_of_code_2024::{get_input, task_argument};
use rayon::{iter::ParallelIterator, slice::ParallelSlice};

fn main() {
    task_argument(task1, task2);
}

fn task1() {
    let lines: Vec<String> = std::fs::read_to_string("inputs/input_day4.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    let res = solve1(lines);
    println!("res: {}", res);
}

fn solve1(input_lines: Vec<String>) -> i32 {
    let input_lines: Vec<_> = input_lines
        .into_iter()
        .map(|string| string.chars().collect::<Vec<char>>())
        .collect();

    let mut total_hits = 0;
    for (y, line) in input_lines.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 'X' {
                total_hits += check_xmas(y, x, &input_lines);
            }
        }
    }
    total_hits
}

fn check_xmas(y: usize, x: usize, input_lines: &Vec<Vec<char>>) -> i32 {
    //count hits

    let directions: Vec<(i32, i32)> = vec![
        (1, 0),   //N
        (1, 1),   //NE
        (0, 1),   //E
        (-1, 1),  //SE
        (-1, 0),  //S
        (-1, -1), //SW
        (0, -1),  //W
        (1, -1),  //NW
    ];

    let mut hits = 0;
    for dir in directions {
        if let Ok(()) = check_direction(y as i32, x as i32, dir, input_lines) {
            hits += 1;
        }
    }
    hits
}

fn check_direction(
    y: i32,
    x: i32,
    direction: (i32, i32),
    input_lines: &Vec<Vec<char>>,
) -> Result<(), ()> {
    let mut index = 0;
    while let Ok(char) = get_char_in_coord(
        (y + direction.0 * index, x + direction.1 * index),
        input_lines,
    ) {
        let hit = match index {
            0 => char == 'X',
            1 => char == 'M',
            2 => char == 'A',
            3 => char == 'S',
            _ => true,
        };
        if !hit {
            return Err(());
        }
        if index == 3 {
            return Ok(());
        }
        index += 1;
    }
    Err(())
}

fn get_char_in_coord(coord: (i32, i32), input_lines: &Vec<Vec<char>>) -> Result<char, ()> {
    if coord.0 < 0 && coord.1 < 0 {
        return Err(());
    }

    let coord = (coord.0 as usize, coord.1 as usize);
    let char = input_lines.get(coord.0).ok_or(())?.get(coord.1).ok_or(())?;
    Ok(*char)
}

fn task2() {
    let time = Instant::now();
    let file = std::fs::read_to_string("inputs/input_day4.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();

    let res = solve2(lines);
    println!("res: {}, in {:?}", res, time.elapsed());
}

fn solve2(input_lines: Vec<&str>) -> usize {
    let input_lines: Vec<_> = input_lines
        .into_iter()
        .map(|string| string.chars().collect::<Vec<char>>())
        .collect();

    let res: usize = input_lines
        .windows(3)
        .map(|arr| {
            arr[1]
                .iter()
                .enumerate()
                .skip(1) // not checking leftmost
                .take(arr[1].len() - 2) // not checking rightmost
                .filter(|(index, char)| **char == 'A' && is_mas(*index, arr))
                .count()
        })
        .sum();
    res
}

fn is_mas(index: usize, arr: &[Vec<char>]) -> bool {
    let chars = ['S', 'M'];
    const INDEXES: [(usize, i32); 2] = [(0, -1), (2, 1)];
    const INDEXES2: [(usize, i32); 2] = [(2, -1), (0, 1)];

    let first = INDEXES
        .iter()
        .zip(chars.iter())
        .all(|((y, x), char)| arr[*y][(index as i32 + *x) as usize] == *char);

    let first_rev = INDEXES
        .iter()
        .zip(chars.iter().rev())
        .all(|((y, x), char)| arr[*y][(index as i32 + *x) as usize] == *char);

    let second = INDEXES2
        .iter()
        .zip(chars.iter())
        .all(|((y, x), char)| arr[*y][(index as i32 + *x) as usize] == *char);

    let second_rev = INDEXES2
        .iter()
        .zip(chars.iter().rev())
        .all(|((y, x), char)| arr[*y][(index as i32 + *x) as usize] == *char);

    (first || first_rev) && (second || second_rev)
}

fn is_match(
    center: (usize, usize),
    relative_coords: Vec<(i32, i32)>,
    input_lines: &Vec<Vec<char>>,
) -> Result<(), ()> {
    let mas = ['M', 'A', 'S'];
    for (index, coord) in relative_coords.iter().enumerate() {
        let coord = (center.0 as i32 + coord.0, center.1 as i32 + coord.1);
        let hit = get_char_in_coord(coord, input_lines)?;
        if hit != mas[index] {
            return Err(());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{solve1, solve2};

    #[test]
    fn test_task1() {
        let input = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];

        let val = solve1(input);
        println!("Res: {}", val)
    }

    #[test]
    fn test_task2() {
        let input = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];

        let val = solve2(input);
        println!("Res: {}", val)
    }
}
