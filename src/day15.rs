use std::{
    io::Read,
    str::FromStr,
    thread::{self, panicking},
    time::{Duration, Instant},
};

use advent_of_code_2024::task_argument_with_input;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};
use rayon::iter;

fn main() {
    let time = Instant::now();
    let res = task_argument_with_input("inputs/15.txt", task1, task2);
    println!("Res: {}, Took: {:?}", res, time.elapsed());
}

#[derive(Default, Clone, Debug)]
struct Pos(usize, usize);

impl Pos {
    fn shift(&self, input: &Move) -> Self {
        let pos = self;
        match input {
            Move::Up => Pos(pos.0.checked_sub(1).unwrap(), pos.1),
            Move::Right => Pos(pos.0, pos.1.checked_add(1).unwrap()),
            Move::Down => Pos(pos.0.checked_add(1).unwrap(), pos.1),
            Move::Left => Pos(pos.0, pos.1.checked_sub(1).unwrap()),
        }
    }
}

struct Level {
    player: Pos,
    elements: Vec<Vec<Option<Element>>>,
}

impl Level {
    fn draw(&self) {
        print!("{}[2J", 27 as char);
        //println!();
        self.elements.iter().for_each(|line| {
            line.iter().for_each(|elem| match elem {
                Some(Element::Box) => print!("O"),
                Some(Element::Wall) => print!("#"),
                Some(Element::Player) => print!("@"),
                Some(Element::WideboxA) => print!("["),
                Some(Element::WideboxB) => print!("]"),
                None => print!(" "),
            });
            println!();
        });
    }

    fn gps_sum(&self) -> usize {
        self.elements
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, e)| {
                        matches!(e, Some(Element::Box)) || matches!(e, Some(Element::WideboxA))
                    })
                    .map(|x| y * 100 + x.0)
                    .sum::<usize>()
            })
            .sum()
    }

    fn make_wide(&mut self) {
        let width = self.elements.first().unwrap().len();
        let height = self.elements.len();
        let mut elements = vec![vec![None::<Element>; width * 2]; height];

        self.elements.iter().enumerate().for_each(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(i, opt)| opt.clone().map(|val| (i, val)))
                .for_each(|(x, e)| {
                    let x = x * 2;
                    match e {
                        Element::Wall => {
                            elements[y][x] = Some(Element::Wall);
                            elements[y][x + 1] = Some(Element::Wall);
                        }
                        Element::Box => {
                            elements[y][x] = Some(Element::WideboxA);
                            elements[y][x + 1] = Some(Element::WideboxB);
                        }
                        Element::Player => {
                            elements[y][x] = Some(Element::Player);
                            elements[y][x + 1] = None
                        }
                        Element::WideboxA => panic!("Cannot make wide box wider"),
                        Element::WideboxB => panic!("Cannot make wide box wider"),
                    }
                });
        });

        self.player = Pos(self.player.0, self.player.1 * 2);
        self.elements = elements;
    }

    fn player_move(&mut self, input: &Move) {
        let pos = self.player.clone();
        if self.can_move(pos.clone(), input) {
            let removed = self.elements[pos.0][pos.1].take().unwrap();
            let new_pos = pos.shift(input);
            self.move_chain(new_pos.clone(), input, removed);
            self.player = new_pos;
        }
    }

    fn move_chain(&mut self, pos: Pos, input: &Move, e: Element) {
        let removed = std::mem::replace(&mut self.elements[pos.0][pos.1], Some(e));
        if let Some(e) = removed {
            if *input == Move::Up || *input == Move::Down {
                match e {
                    Element::WideboxA => {
                        let pos_b = Pos(pos.0, pos.1 + 1);
                        let removed = self.elements[pos_b.0][pos_b.1].take();
                        self.move_chain(pos_b.shift(input), input, removed.unwrap())
                    }
                    Element::WideboxB => {
                        let pos_a = Pos(pos.0, pos.1 - 1);
                        let removed = self.elements[pos_a.0][pos_a.1].take();

                        self.move_chain(pos_a.shift(input), input, removed.unwrap())
                    }
                    _ => (),
                }
            }
            self.move_chain(pos.shift(input), input, e);
        }
    }

    fn can_move(&self, pos: Pos, input: &Move) -> bool {
        let new_pos = pos.shift(input);

        match self.elements[new_pos.0][new_pos.1] {
            Some(Element::Box) => self.can_move(new_pos, input),
            Some(Element::WideboxA) => {
                if *input == Move::Up || *input == Move::Down {
                    self.can_move(new_pos.shift(&Move::Right), input)
                        && self.can_move(new_pos, input)
                } else {
                    self.can_move(new_pos, input)
                }
            }
            Some(Element::WideboxB) => {
                if *input == Move::Up || *input == Move::Down {
                    self.can_move(new_pos.shift(&Move::Left), input)
                        && self.can_move(new_pos, input)
                } else {
                    self.can_move(new_pos, input)
                }
            }
            Some(Element::Wall) => false,
            Some(Element::Player) => panic!("Who pushes player"),
            None => true,
        }
    }

    fn from_vec_str(s: Vec<&str>) -> Result<Self, ()> {
        let height = s.len();
        let width = s[0].len();

        let mut player = Pos::default();
        let mut elements: Vec<Vec<Option<Element>>> = vec![vec![None; width]; height];

        s.iter().enumerate().for_each(|(y, line)| {
            line.char_indices().for_each(|(x, char)| {
                elements[y][x] = match char {
                    '#' => Some(Element::Wall),
                    'O' => Some(Element::Box),
                    '.' => None,
                    '@' => {
                        player = Pos(y, x);
                        Some(Element::Player)
                    }
                    _ => panic!("Not expected char: {}", char),
                };
            })
        });

        Ok(Self { player, elements })
    }
}

#[derive(Clone)]
enum Element {
    Wall,
    Box,
    Player,
    WideboxA,
    WideboxB,
}

#[derive(Debug, Eq, PartialEq)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}
impl Move {
    fn from_char(s: char) -> Result<Self, ()> {
        let res = match s {
            '<' => Self::Left,
            'v' => Self::Down,
            '^' => Self::Up,
            '>' => Self::Right,
            _ => return Err(()),
        };
        Ok(res)
    }
}

fn parse_input(input: String) -> (Level, Vec<Move>) {
    let moves: Vec<_> = input
        .lines()
        .skip_while(|line| line.starts_with("#"))
        .skip(1)
        .flat_map(|line| line.chars().map(Move::from_char))
        .flatten()
        .collect();

    let level: Vec<&str> = input
        .lines()
        .take_while(|line| line.starts_with("#"))
        .collect();

    let level = Level::from_vec_str(level).unwrap();
    (level, moves)
}

fn task1(input: String) -> String {
    let (mut level, moves) = parse_input(input);

    for m in moves {
        level.player_move(&m);
        //level.draw();
        //thread::sleep(Duration::from_millis(1));
    }
    level.gps_sum().to_string()
}

fn task2(input: String) -> String {
    let (mut level, moves) = parse_input(input);
    level.make_wide();

    for m in moves {
        level.player_move(&m);
        //level.draw();
        //thread::sleep(Duration::from_millis(1));
    }

    level.gps_sum().to_string()
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, task1, task2};

    const INPUT1: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn parse_test() {
        parse_input(INPUT1.to_string());
    }

    #[test]
    fn test_task1() {
        task1(INPUT1.to_string());
    }

    #[test]
    fn test_task2() {
        let res = task2(INPUT1.to_string());
        println!("res: {}", res)
    }
}
