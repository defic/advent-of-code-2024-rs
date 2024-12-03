use std::env;

use advent_of_code_2024::{get_input, task_argument};

fn main() {
    task_argument(task1, task2);
}

fn task1() {
    let input_lines = get_input();

    let number_vecs: Vec<_> = input_lines
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let safe_count = number_vecs.iter().filter(|x| is_safe(x)).count();
    println!("Safe count: {}", safe_count);
}

fn is_safe(numbers: &[i32]) -> bool {
    let increasing = numbers[0] < numbers[1];
    let safe = numbers.windows(2).all(|elements| {
        let window_increasing = elements[0] < elements[1];
        let diff = (elements[0] - elements[1]).abs();
        let correct_diff = diff > 0 && diff < 4;
        increasing == window_increasing && correct_diff
    });
    safe
}

fn task2() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_safe_increasing() {
        assert!(is_safe(&[1, 3, 4, 6, 9]));
        assert!(!is_safe(&[1, 3, 3, 6, 9]));
        assert!(!is_safe(&[9, 5, 4, 3, 2, 1]));
        assert!(is_safe(&[9, 6, 4, 3, 2, 1]));
    }
}
