use std::{fmt, time::Instant};

use advent_of_code_2024::task_argument_with_input;
use nom::InputIter;

fn main() {
    let time = Instant::now();
    let res = task_argument_with_input("inputs/day9.txt", task1, task2);
    println!("Res: {}, Took: {:?}", res, time.elapsed());
}

fn task1(input: String) -> String {
    let input = parse_input(input);
    let mut filesystem = input
        .iter()
        .flat_map(|block| block.write())
        .collect::<Vec<String>>();
    omnipod(&mut filesystem);
    let res = checksum(filesystem);
    res.to_string()
}
fn task2(input: String) -> String {
    let mut input = parse_input(input);
    omnipod2(&mut input);
    let filesystem = input
        .iter()
        .flat_map(|block| block.write())
        .collect::<Vec<String>>();
    let res = checksum(filesystem);
    res.to_string()
}

#[derive(Clone)]
enum Block {
    File { index: usize, size: usize },
    Free,
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Block::Free => write!(f, "."),
            Block::File { index, size } => {
                let s = index.to_string().repeat(*size);
                write!(f, "{}", s)
            }
        }
    }
}

impl Block {
    fn write(&self) -> Vec<String> {
        match self {
            Block::File { index, size } => vec![index.to_string(); *size],
            Block::Free => vec![".".to_string()],
        }
    }
}

fn parse_input(input: String) -> Vec<Block> {
    let vec: Vec<_> = input.chars().collect();
    vec.chunks(2)
        .enumerate()
        .flat_map(|(index, chars)| {
            let file_size = chars[0].to_digit(10).unwrap();
            let block = Block::File {
                index,
                size: file_size as usize,
            };
            let mut output = vec![block];
            if chars.len() > 1 && chars[1] != '\n' {
                let empty_size = chars[1].to_digit(10).unwrap();
                if empty_size > 0 {
                    output.append(&mut vec![Block::Free; empty_size as usize]);
                }
            }
            output
        })
        .collect()
}

fn omnipod(input: &mut [String]) {
    let mut index = 0;
    let mut last_index = input.len() - 1;
    'main: loop {
        if index >= input.len() {
            break;
        }
        if input[index] == "." {
            'last: loop {
                if last_index <= index {
                    break 'main;
                }
                if input[last_index] == "." {
                    last_index -= 1;
                    continue;
                }
                break 'last;
            }
            input.swap(index, last_index);
        }

        index += 1;
    }
}

fn omnipod2(input: &mut Vec<Block>) {
    let mut last_index = input.len() - 1;

    loop {
        let block = input[last_index].clone();
        match block {
            Block::File { index: _, size } => {
                if let Some(pos) = input
                    .windows(size)
                    .position(|window| window.iter().all(|x| matches!(x, Block::Free)))
                {
                    if pos < last_index {
                        let file_to_move = input.remove(last_index);
                        input.insert(pos, file_to_move);
                        let items: Vec<_> = input
                            .splice(pos + 1..pos + 1 + size, std::iter::empty())
                            .collect();

                        input.splice(last_index - size + 1..last_index - size + 1, items);
                    }
                }
            }
            Block::Free => (),
        }
        if last_index == 0 {
            break;
        }
        last_index -= 1;
    }
}

fn checksum(input: Vec<String>) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, e)| *e != ".")
        .map(|(index, el)| index * el.parse::<usize>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{checksum, omnipod, omnipod2, parse_input};

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_input_parsing() {
        let res = parse_input(TEST_INPUT.to_string());
        let res: Vec<_> = res.iter().flat_map(|block| block.write()).collect();
        let res = res.join("");
        println!("output: {}", res);
        assert_eq!(
            "00...111...2...333.44.5555.6666.777.888899".to_string(),
            res
        )
    }

    #[test]
    fn test_omnipod() {
        let res = parse_input(TEST_INPUT.to_string());
        let mut filesystem = res
            .iter()
            .flat_map(|block| block.write())
            .collect::<Vec<String>>();
        println!("input: {}", filesystem.join(""));
        omnipod(&mut filesystem);
        let res = filesystem.join("");
        println!("output: {:?}", res);
    }

    #[test]
    fn test_omnipod2() {
        let mut res = parse_input(TEST_INPUT.to_string());
        let filesystem = res
            .iter()
            .flat_map(|block| block.write())
            .collect::<Vec<String>>();
        println!("input: {}", filesystem.join(""));
        omnipod2(&mut res);
        let filesystem = res
            .iter()
            .flat_map(|block| block.write())
            .collect::<Vec<String>>();
        println!("output: {}", filesystem.join(""));
    }

    #[test]
    fn test_checksum() {
        let res = parse_input(TEST_INPUT.to_string());
        let mut filesystem = res
            .iter()
            .flat_map(|block| block.write())
            .collect::<Vec<String>>();
        omnipod(&mut filesystem);
        let res = checksum(filesystem);
        println!("checksum: {:?}", res);
    }

    #[test]
    fn test_checksum2() {
        let mut res = parse_input(TEST_INPUT.to_string());
        let filesystem = res
            .iter()
            .flat_map(|block| block.write())
            .collect::<Vec<String>>();
        println!("input: {}", filesystem.join(""));
        omnipod2(&mut res);
        let filesystem = res
            .iter()
            .flat_map(|block| block.write())
            .collect::<Vec<String>>();
        println!("output: {}", filesystem.join(""));
        let res = checksum(filesystem);
        println!("checksum: {:?}", res);
    }
}
