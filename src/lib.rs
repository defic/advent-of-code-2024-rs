use std::{
    env,
    io::{self, BufRead},
};

pub fn get_input() -> Vec<String> {
    let stdin = io::stdin();
    let input_lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();
    input_lines
}

pub fn task_argument(task1: impl Fn(), task2: impl Fn()) {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first() {
        Some(val) if val == "1" => task1(),
        Some(val) if val == "2" => task2(),
        Some(_) => eprintln!("Invalid argument. Only 1 or 2 are accepted"),
        None => eprintln!("Provide argument 1 or 2"),
    }
}