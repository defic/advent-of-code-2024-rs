use std::{collections::HashSet, hash::Hash, process::exit, time::Instant};

use advent_of_code_2024::task_argument;
use nom::InputIter;

fn main() {
    task_argument(task1, task2);
}

fn task1() {
    let input = std::fs::read_to_string("inputs/day6.txt").unwrap();
    let input: Vec<_> = input.lines().collect();
    let res = solve1(input);
    println!("res: {}", res);
}

fn get_walls(input: &Vec<&str>) -> Vec<Vec<bool>> {
    let walls: Vec<_> = input
        .iter()
        .cloned()
        .map(|e| {
            e.iter_elements()
                .map(|char| char == '#')
                .collect::<Vec<_>>()
        })
        .collect();
    walls
}

fn solve1(input: Vec<&str>) -> usize {
    let walls = get_walls(&input);
    let mut guard: (i32, i32) = Default::default();

    for (y, line) in input.iter().enumerate() {
        if let Some((x, _)) = line.iter_indices().find(|(x, char)| *char == '^') {
            guard = (y as i32, x as i32);
            break;
        }
    }

    let height = walls.len();
    let width = walls.first().unwrap().len();
    let mut tracks = vec![vec![false; width]; height];
    let mut direction = Direction::Up;
    loop {
        let mut steps_to_obstacle = Vec::new();
        let Ok(pos) = next_stop(guard, &direction, &walls, &mut steps_to_obstacle) else {
            break;
        };
        guard = pos;
        direction = direction.next();
        steps_to_obstacle.iter().for_each(|step| {
            tracks[step.0 as usize][step.1 as usize] = true;
        });
    }

    tracks.into_iter().flatten().filter(|x| *x).count()
}

fn next_stop(
    mut guard: (i32, i32),
    direction: &Direction,
    walls: &Vec<Vec<bool>>,
    steps: &mut Vec<(i32, i32)>,
) -> Result<(i32, i32), (i32, i32)> {
    loop {
        let next_step = (
            guard.0 + direction.vector().0,
            guard.1 + direction.vector().1,
        );

        if out_of_bounds(next_step, walls) {
            return Err(guard);
        }
        if is_wall(next_step, walls) {
            return Ok(guard);
        }
        guard = next_step;
        steps.push(guard);
    }
}

fn is_wall(pos: (i32, i32), walls: &Vec<Vec<bool>>) -> bool {
    walls[pos.0 as usize][pos.1 as usize]
}

fn out_of_bounds(next_step: (i32, i32), walls: &Vec<Vec<bool>>) -> bool {
    next_step.0 < 0
        || next_step.1 < 0
        || next_step.0 >= walls.len() as i32
        || next_step.1 >= walls.first().unwrap().len() as i32
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

fn task2() {
    let time = Instant::now();
    let input = std::fs::read_to_string("inputs/day6.txt").unwrap();
    let input: Vec<_> = input.lines().collect();
    let res = solve2(input);
    println!("res: {}, elapsed: {:?}", res, time.elapsed());
}

fn solve2(input: Vec<&str>) -> usize {
    let walls = get_walls(&input);
    let mut guard: (i32, i32) = Default::default();

    for (y, line) in input.iter().enumerate() {
        if let Some((x, _)) = line.iter_indices().find(|(_, char)| *char == '^') {
            guard = (y as i32, x as i32);
            break;
        }
    }

    let height = walls.len();
    let width = walls.first().unwrap().len();
    let mut direction = Direction::Up;

    let mut wall_options = Vec::new();
    let mut tracks = vec![vec![false; width]; height];
    let mut wall_clone = walls.clone(); //

    loop {
        let mut exiting = false;
        let mut steps_to_obstacle = Vec::new();
        let pos: (i32, i32) = match next_stop(guard, &direction, &walls, &mut steps_to_obstacle) {
            Ok(pos) => pos,
            Err(pos) => {
                exiting = true;
                pos
            }
        };

        for obstacle in steps_to_obstacle.iter() {
            if tracks[obstacle.0 as usize][obstacle.1 as usize] {
                continue;
            }
            wall_clone[obstacle.0 as usize][obstacle.1 as usize] = true;
            if !can_exit(guard, direction.clone(), &wall_clone) {
                wall_options.push(*obstacle);
            }
            wall_clone[obstacle.0 as usize][obstacle.1 as usize] = false;
        }

        if exiting {
            break;
        }

        guard = pos;
        direction = direction.next();

        steps_to_obstacle.iter().for_each(|step| {
            tracks[step.0 as usize][step.1 as usize] = true;
        });
    }

    wall_options.len()
}

fn can_exit(mut guard: (i32, i32), mut direction: Direction, walls: &Vec<Vec<bool>>) -> bool {
    let mut visited_stops: HashSet<((i32, i32), Direction)> = HashSet::new();
    loop {
        let mut steps_to_obstacle = Vec::new();
        let Ok(pos) = next_stop(guard, &direction, walls, &mut steps_to_obstacle) else {
            break;
        };

        if !visited_stops.insert((pos, direction.clone())) {
            return false;
        }

        guard = pos;
        direction = direction.next();
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::{get_walls, is_wall, solve1, solve2};

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
        let input: Vec<_> = TEST_INPUT.lines().collect();
        let count = solve1(input);
        println!("count: {}", count);
    }

    #[test]
    fn task2_test() {
        let input: Vec<_> = TEST_INPUT.lines().collect();
        let count = solve2(input);
        println!("count: {}", count);
    }

    #[test]
    fn wall_test() {
        let input: Vec<_> = TEST_INPUT.lines().collect();
        let walls = get_walls(&input);

        assert!(is_wall((0, 4), &walls));
    }
}
