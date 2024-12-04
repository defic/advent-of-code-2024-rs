use advent_of_code_2024::{get_input, task_argument};

fn main() {
    task_argument(task1, task2);
}

fn task1() {
    let lines: Vec<String> = std::fs::read_to_string("inputs/input_day4.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    let res = solve1(lines);
    println!("res: {}", res);
}

fn solve1(input_lines: Vec<String>) -> i32 {
    let input_lines: Vec<_> = input_lines
        .into_iter()
        .map(|string| string.chars().collect::<Vec<char>>())
        .collect();

    let mut total_hits = 0;
    for (y, line) in input_lines.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 'X' {
                total_hits += check_xmas(y, x, &input_lines);
            }
        }
    }
    total_hits
}

fn check_xmas(y: usize, x: usize, input_lines: &Vec<Vec<char>>) -> i32 {
    //count hits

    let directions: Vec<(i32, i32)> = vec![
        (1, 0),   //N
        (1, 1),   //NE
        (0, 1),   //E
        (-1, 1),  //SE
        (-1, 0),  //S
        (-1, -1), //SW
        (0, -1),  //W
        (1, -1),  //NW
    ];

    let mut hits = 0;
    for dir in directions {
        if let Ok(()) = check_direction(y as i32, x as i32, dir, input_lines) {
            hits += 1;
        }
    }
    hits
}

fn check_direction(
    y: i32,
    x: i32,
    direction: (i32, i32),
    input_lines: &Vec<Vec<char>>,
) -> Result<(), ()> {
    let mut index = 0;
    while let Ok(char) = get_char_in_coord(
        (y + direction.0 * index, x + direction.1 * index),
        input_lines,
    ) {
        let hit = match index {
            0 => char == 'X',
            1 => char == 'M',
            2 => char == 'A',
            3 => char == 'S',
            _ => true,
        };
        if !hit {
            return Err(());
        }
        if index == 3 {
            return Ok(());
        }
        index += 1;
    }
    Err(())
}

fn get_char_in_coord(coord: (i32, i32), input_lines: &Vec<Vec<char>>) -> Result<char, ()> {
    if coord.0 < 0 && coord.1 < 0 {
        return Err(());
    }

    let coord = (coord.0 as usize, coord.1 as usize);
    let char = input_lines.get(coord.0).ok_or(())?.get(coord.1).ok_or(())?;
    Ok(*char)
}

fn task2() {
    let lines: Vec<String> = std::fs::read_to_string("inputs/input_day4.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    let res = solve2(lines);
    println!("res: {}", res);
}

fn solve2(input_lines: Vec<String>) -> i32 {
    let input_lines: Vec<_> = input_lines
        .into_iter()
        .map(|string| string.chars().collect::<Vec<char>>())
        .collect();

    let mut total_hits = 0;
    for (y, line) in input_lines.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 'A' {
                total_hits += check_mas(y, x, &input_lines);
            }
        }
    }
    total_hits
}

fn is_match(
    center: (usize, usize),
    relative_coords: Vec<(i32, i32)>,
    input_lines: &Vec<Vec<char>>,
) -> Result<(), ()> {
    let mas = ['M', 'A', 'S'];
    for (index, coord) in relative_coords.iter().enumerate() {
        let coord = (center.0 as i32 + coord.0, center.1 as i32 + coord.1);
        let hit = get_char_in_coord(coord, input_lines)?;
        if hit != mas[index] {
            return Err(());
        }
    }
    Ok(())
}

fn check_mas(y: usize, x: usize, input_lines: &Vec<Vec<char>>) -> i32 {
    //either:
    let uphill: Vec<(i32, i32)> = vec![(-1, -1), (0, 0), (1, 1)];
    let mut reverse = uphill.clone();
    reverse.reverse();
    let uphill = vec![uphill, reverse];

    let uphill_results: Vec<_> = uphill
        .into_iter()
        .map(|d| is_match((y, x), d, input_lines))
        .collect();

    let uphill_ok = uphill_results.iter().any(|result| result.is_ok());

    //either
    let downhill: Vec<(i32, i32)> = vec![(-1, 1), (0, 0), (1, -1)];
    let mut reverse = downhill.clone();
    reverse.reverse();
    let downhill = vec![downhill, reverse];

    let downhill_results: Vec<_> = downhill
        .into_iter()
        .map(|d| is_match((y, x), d, input_lines))
        .collect();

    let downhill_ok = downhill_results.iter().any(|result| result.is_ok());

    if uphill_ok && downhill_ok {
        return 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::{solve1, solve2};

    #[test]
    fn test_task1() {
        let input = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];

        let val = solve1(input);
        println!("Res: {}", val)
    }

    #[test]
    fn test_task2() {
        let input = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];

        let val = solve2(input);
        println!("Res: {}", val)
    }
}
