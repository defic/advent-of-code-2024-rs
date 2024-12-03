use advent_of_code_2024::{get_input, task_argument};

fn main() {
    task_argument(task1, task2);
}

fn number_vecs(input_lines: Vec<String>) -> Vec<Vec<i32>> {
    input_lines
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn task1() {
    let input_lines = get_input();
    let number_vecs = number_vecs(input_lines);
    let safe_count = number_vecs.iter().filter(|x| is_safe(x)).count();
    println!("Safe count: {}", safe_count);
}

fn is_safe(numbers: &[i32]) -> bool {
    let increasing = numbers[0] < numbers[1];
    numbers
        .windows(2)
        .all(|elements| safe_window(elements, increasing))
}

fn safe_window(elements: &[i32], increasing: bool) -> bool {
    let window_increasing = elements[0] < elements[1];
    let diff = (elements[0] - elements[1]).abs();
    let correct_diff = diff > 0 && diff < 4;
    increasing == window_increasing && correct_diff
}

fn task2() {
    let input_lines = get_input();
    let number_vecs = number_vecs(input_lines);
    let safe_count = number_vecs.iter().filter(|x| report_dampener(x)).count();
    println!("Safe count: {}", safe_count);
}

fn report_dampener(numbers: &[i32]) -> bool {
    let mut reverse = numbers.to_vec();
    reverse.reverse();

    can_damp_problems(numbers) || can_damp_problems(&reverse)
}

fn can_damp_problems(numbers: &[i32]) -> bool {
    let Some((a, b)) = get_problem_indices(numbers) else {
        return true; // no problems
    };
    let mut without_a = numbers.to_vec();
    without_a.remove(a);
    let mut without_b = numbers.to_vec();
    without_b.remove(b);
    get_problem_indices(&without_a).is_none() || get_problem_indices(&without_b).is_none()
}

fn get_problem_indices(numbers: &[i32]) -> Option<(usize, usize)> {
    for (index, elem) in numbers.iter().enumerate() {
        if let Some(next) = numbers.get(index + 1) {
            let diff = (elem - next).abs();
            if !(1..=3).contains(&diff) || elem > next {
                return Some((index, index + 1));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_safe() {
        assert!(is_safe(&[1, 3, 4, 6, 9]));
        assert!(!is_safe(&[1, 3, 3, 6, 9]));
        assert!(!is_safe(&[9, 5, 4, 3, 2, 1]));
        assert!(is_safe(&[9, 6, 4, 3, 2, 1]));
    }

    #[test]
    fn test_dampener() {
        assert!(report_dampener(&[7, 6, 4, 2, 1]));
        assert!(!report_dampener(&[1, 2, 7, 8, 9]));
        assert!(!report_dampener(&[9, 7, 6, 2, 1]));
        assert!(report_dampener(&[1, 3, 2, 4, 5]));
        assert!(report_dampener(&[8, 6, 4, 4, 1]));
        assert!(report_dampener(&[1, 3, 6, 7, 9]));

        // should pass since we can just remove 15
        assert!(report_dampener(&[1, 3, 6, 9, 15]));
    }
}
