use std::{
    collections::{hash_map::Entry, HashMap},
    time::Instant,
};

use advent_of_code_2024::task_argument_with_input;
use nom::InputIter;

fn main() {
    let time = Instant::now();
    let res = task_argument_with_input("inputs/8.txt", task1, task2);
    println!("Res: {}, Took: {:?}", res, time.elapsed());
}

fn task1(input: String) -> String {
    let Input {
        antennas,
        mut antinodes,
    } = parse_input(input);

    antennas.iter().for_each(|(char, list)| {
        list.iter().for_each(|coords| {
            list.iter().for_each(|coords2| {
                if coords != coords2 {
                    let dist = (coords.0 - coords2.0, coords.1 - coords2.1);
                    let a = (coords.0 + dist.0, coords.1 + dist.1);
                    let b = (coords2.0 - dist.0, coords2.1 - dist.1);
                    add_antinode(&mut antinodes, a);
                    add_antinode(&mut antinodes, b);
                }
            })
        })
    });

    let res: usize = antinodes
        .iter()
        .map(|line| line.iter().filter(|e| **e).count())
        .sum();
    res.to_string()
}

fn add_antinode(antinodes: &mut [Vec<bool>], location: (i32, i32)) -> bool {
    if location.0 < 0
        || location.1 < 0
        || location.0 >= antinodes.len() as i32
        || location.1 >= antinodes.first().unwrap().len() as i32
    {
        return false;
    }
    antinodes[location.0 as usize][location.1 as usize] = true;
    true
}

fn task2(input: String) -> String {
    let Input {
        antennas,
        mut antinodes,
    } = parse_input(input);

    antennas.iter().for_each(|(_, list)| {
        list.iter().for_each(|coords| {
            list.iter().for_each(|coords2| {
                if coords != coords2 {
                    add_antinode(&mut antinodes, *coords);
                    add_antinode(&mut antinodes, *coords2);
                    let dist = (coords.0 - coords2.0, coords.1 - coords2.1);
                    let mut pos = (coords.0 + dist.0, coords.1 + dist.1);
                    loop {
                        if !add_antinode(&mut antinodes, pos) {
                            break;
                        }
                        pos = (pos.0 + dist.0, pos.1 + dist.1);
                    }

                    let mut pos = (coords2.0 - dist.0, coords2.1 - dist.1);
                    loop {
                        if !add_antinode(&mut antinodes, pos) {
                            break;
                        }
                        pos = (pos.0 - dist.0, pos.1 - dist.1);
                    }
                }
            })
        })
    });

    let res: usize = antinodes
        .iter()
        .map(|line| line.iter().filter(|e| **e).count())
        .sum();
    res.to_string()
}

pub struct Input {
    antennas: HashMap<char, Vec<(i32, i32)>>,
    antinodes: Vec<Vec<bool>>,
}

fn parse_input(input: String) -> Input {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.iter_indices().for_each(|(x, char)| {
            if char != '.' {
                let item = (y as i32, x as i32);
                if let Entry::Vacant(e) = antennas.entry(char) {
                    e.insert(vec![item]);
                } else {
                    antennas.get_mut(&char).unwrap().push((y as i32, x as i32));
                }
            }
        })
    });

    let antinodes = vec![vec![false; width]; height];
    Input {
        antennas,
        antinodes,
    }
}
