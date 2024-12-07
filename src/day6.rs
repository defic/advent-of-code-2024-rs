use std::time::Instant;

use advent_of_code_2024::task_argument_with_input;
use nom::InputIter;
use rayon::prelude::*;

fn main() {
    let time = Instant::now();
    let res = task_argument_with_input("inputs/day6.txt", task1, task2);
    println!("Res: {}, Took: {:?}", res, time.elapsed());
}

fn task1(input: String) -> String {
    let input: Vec<_> = input.lines().collect();
    let walls = get_walls(&input);
    let guard = guard_pos(&input);
    let direction = Direction::Up;
    let steps = get_steps(guard, direction, &walls);
    let mut steps_coords: Vec<_> = steps.into_iter().map(|e| e.0).collect();
    steps_coords.sort();
    steps_coords.dedup();
    steps_coords.len().to_string()
}

fn task2(input: String) -> String {
    let input: Vec<_> = input.lines().collect();
    let walls = get_walls(&input);
    let guard = guard_pos(&input);
    let direction = Direction::Up;
    let steps = get_steps(guard, direction, &walls);

    let steps_clone = steps.clone();
    let possible_walls: Vec<_> = steps
        .into_par_iter()
        .enumerate()
        .skip(1)
        .filter(|(index, pos_and_dir)| {
            let already_walked = &steps_clone[..*index].iter().find(|e| e.0 == pos_and_dir.0);
            let start = steps_clone[index - 1].0;
            already_walked.is_none()
                && !can_exit(start, pos_and_dir.1.clone(), &walls, pos_and_dir.0)
        })
        .map(|e| e.1 .0)
        .collect();

    possible_walls.len().to_string()
}

fn get_steps(
    mut start: (i32, i32),
    mut direction: Direction,
    walls: &Vec<Vec<bool>>,
) -> Vec<((i32, i32), Direction)> {
    let mut steps: Vec<((i32, i32), Direction)> = Vec::new();
    steps.push((start, direction.clone()));
    loop {
        let Ok(pos) = next_stop(start, &direction, &walls, &mut steps, None) else {
            break;
        };
        start = pos;
        direction = direction.next();
    }
    steps
}

fn can_exit(
    mut guard: (i32, i32),
    mut direction: Direction,
    walls: &[Vec<bool>],
    extra_wall: (i32, i32),
) -> bool {
    let mut visited_stops: Vec<((i32, i32), Direction)> = Vec::new();
    loop {
        let mut steps_to_obstacle = Vec::new();
        let Ok(pos) = next_stop(
            guard,
            &direction,
            walls,
            &mut steps_to_obstacle,
            Some(extra_wall),
        ) else {
            break;
        };

        let stop = (pos, direction.clone());
        if visited_stops.contains(&stop) {
            return false; // in a loop!
        }
        visited_stops.push(stop);

        guard = pos;
        direction = direction.next();
    }
    true
}

fn next_stop(
    mut guard: (i32, i32),
    direction: &Direction,
    walls: &[Vec<bool>],
    steps: &mut Vec<((i32, i32), Direction)>,
    extra_wall: Option<(i32, i32)>,
) -> Result<(i32, i32), (i32, i32)> {
    loop {
        let next_step = (
            guard.0 + direction.vector().0,
            guard.1 + direction.vector().1,
        );

        if out_of_bounds(next_step, walls) {
            return Err(guard);
        }
        if is_wall(next_step, walls) || Some(next_step) == extra_wall {
            return Ok(guard);
        }
        guard = next_step;
        steps.push((guard, direction.clone()));
    }
}

fn is_wall(pos: (i32, i32), walls: &[Vec<bool>]) -> bool {
    walls[pos.0 as usize][pos.1 as usize]
}

fn out_of_bounds(next_step: (i32, i32), walls: &[Vec<bool>]) -> bool {
    next_step.0 < 0
        || next_step.1 < 0
        || next_step.0 >= walls.len() as i32
        || next_step.1 >= walls.first().unwrap().len() as i32
}

fn guard_pos(input: &Vec<&str>) -> (i32, i32) {
    for (y, line) in input.iter().enumerate() {
        if let Some((x, _)) = line.iter_indices().find(|(_, char)| *char == '^') {
            return (y as i32, x as i32);
        }
    }
    panic!("No guard pos")
}

fn get_walls(input: &Vec<&str>) -> Vec<Vec<bool>> {
    let walls: Vec<_> = input
        .iter()
        .map(|e| {
            e.iter_elements()
                .map(|char| char == '#')
                .collect::<Vec<_>>()
        })
        .collect();
    walls
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    pub fn vector(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_walls, is_wall, task1, task2};

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn task1_test() {
        let count = task1(TEST_INPUT.to_string());
        println!("count: {}", count);
    }

    #[test]
    fn task2_test() {
        let count = task2(TEST_INPUT.to_string());
        println!("count: {}", count);
    }

    #[test]
    fn wall_test() {
        let input: Vec<_> = TEST_INPUT.lines().collect();
        let walls = get_walls(&input);

        assert!(is_wall((0, 4), &walls));
    }
}
