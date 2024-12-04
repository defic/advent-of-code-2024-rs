use std::{fs::File, io::Read};

use advent_of_code_2024::{get_input, task_argument};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit0, digit1},
    combinator::{complete, map_res, not, peek},
    multi::{many0, many1, many_till, separated_list1},
    number,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

fn main() {
    task_argument(task1, task2);
}

fn task1() {
    let mut f = File::open("inputs/input_day3_task1.txt").unwrap();
    let mut string = String::new();
    f.read_to_string(&mut string).unwrap();
    let sum = parse_all(&string);
    println!("Sum: {}", sum);
}

// Parse a single number
fn parse_a_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(input)
}

fn multiplication(input: &str) -> IResult<&str, i32> {
    use nom::character::complete::char;

    let (input, _) = tag("mul(")(input)?;
    let (input, n1) = parse_a_number(input)?;
    let (input, _) = char(',')(input)?;
    let (input, n2) = parse_a_number(input)?;
    let (input, _) = char(')')(input)?;

    Ok((input, n1 * n2))
}

//stop if mul( or don't()
fn is_stop_sequence(input: &str) -> IResult<&str, ()> {
    let _ = peek(alt((tag("mul("), tag("don't()"))))(input)?;
    Ok((input, ()))
}

fn skip_chars(input: &str) -> IResult<&str, ()> {
    let (input, (_, _)) = many_till(nom::character::complete::anychar, is_stop_sequence)(input)?;
    Ok((input, ()))
}

pub fn parse_all(input: &str) -> i32 {
    let mut total = 0;
    let mut remaining = input;
    let mut enabled = true;

    while !remaining.is_empty() {
        if enabled {
            // Stops if: mul & don't()
            if let Ok((new_input, _)) = skip_chars(remaining) {
                remaining = new_input;
            }

            if let Ok((_, _)) = peek(tag::<_, _, ()>("don't"))(remaining) {
                enabled = false;
                continue;
            }

            if let Ok((new_input, result)) = multiplication(remaining) {
                total += result;
                remaining = new_input;
            } else if !remaining.is_empty() {
                remaining = &remaining[1..];
            }
        } else if let Ok((new_input, _)) = skip_until_do(remaining) {
            remaining = new_input;
            enabled = true;
        }
    }
    total
}

fn skip_until_do(input: &str) -> IResult<&str, ()> {
    let (input, (_, _)) = many_till(nom::character::complete::anychar, peek(tag("do()")))(input)?;
    Ok((input, ()))
}

fn task2() {
    //Accidentally edited task1 code to solve task2
}

#[cfg(test)]
mod tests {

    use crate::parse_all;

    #[test]
    fn simple() {
        let res = parse_all("asd mul(3,3), mul(5,5x jeps mul(2,2)");
        println!("res: {:?}", res);
    }
}
