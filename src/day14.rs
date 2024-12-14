use std::{
    io::Read,
    str::FromStr,
    thread,
    time::{Duration, Instant},
};

use advent_of_code_2024::task_argument_with_input;
use rayon::iter::Positions;

fn main() {
    let time = Instant::now();
    let res = task_argument_with_input("inputs/14.txt", task1, task2);
    println!("Res: {}, Took: {:?}", res, time.elapsed());
}

#[derive(Debug)]
struct Vec2(i32, i32);

impl FromStr for Vec2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos: String = s.chars().skip(2).collect();
        let mut pos = pos.split(",");
        let x = pos.next().unwrap().parse::<i32>().unwrap();
        let y = pos.next().unwrap().parse::<i32>().unwrap();
        Ok(Self(x, y))
    }
}

fn parse_input(input: String) -> (Vec<Vec2>, Vec<Vec2>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let pos = parts.next().unwrap();
            let pos = Vec2::from_str(pos).unwrap();
            let vel = parts.next().unwrap();
            let vel = Vec2::from_str(vel).unwrap();
            (pos, vel)
        })
        .unzip()
}

fn task1(input: String) -> String {
    let width = 101;
    let height = 103;

    let seconds = 100;
    let (mut pos, vels) = parse_input(input);
    pos.iter_mut().zip(vels).for_each(|(a, b)| {
        a.0 += b.0 * seconds;
        a.1 += b.1 * seconds;
        a.0 = a.0.rem_euclid(width); // wrap
        a.1 = a.1.rem_euclid(height);
    });

    let x_middle = width / 2;
    let y_middle = height / 2;
    let q1 = pos
        .iter()
        .filter(|pos| pos.0 < x_middle && pos.1 < y_middle)
        .count();
    let q2 = pos
        .iter()
        .filter(|pos| pos.0 > x_middle && pos.1 < y_middle)
        .count();
    let q3 = pos
        .iter()
        .filter(|pos| pos.0 < x_middle && pos.1 > y_middle)
        .count();
    let q4 = pos
        .iter()
        .filter(|pos| pos.0 > x_middle && pos.1 > y_middle)
        .count();
    let res = q1 * q2 * q3 * q4;
    res.to_string()
}

fn draw(map: &[Vec<i32>]) {
    print!("{}[2J", 27 as char);

    map.iter().for_each(|line| {
        line.iter().for_each(|num| {
            if *num == 0 {
                print!(" ");
            } else {
                print!("{}", num)
            }
        });
        println!();
    });
}
fn task2(input: String) -> String {
    let width = 101;
    let height = 103;

    let (mut pos, vels) = parse_input(input);

    for second in 0.. {
        pos.iter_mut().zip(vels.iter()).for_each(|(pos, vel)| {
            pos.0 += vel.0;
            pos.1 += vel.1;
            pos.0 = pos.0.rem_euclid(width);
            pos.1 = pos.1.rem_euclid(height);
        });

        let mut map = vec![vec![0; width as usize]; height as usize];

        for Vec2(x, y) in pos.iter() {
            map[*y as usize][*x as usize] += 1;
        }

        // there seems to be some pattern with lot of bots in line 25
        let should_draw = map[25].iter().sum::<i32>() > 27;

        //let should_draw = true;
        if should_draw {
            draw(&map);
            println!("draw: on secs {}", second);
            std::io::stdin().read_exact(&mut [0u8]).unwrap();
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, task1};

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn parse_test() {
        parse_input(INPUT.to_string());
    }

    #[test]
    fn test_task1() {
        let res = task1(INPUT.to_string());
        println!("res {}", res);
    }
}
