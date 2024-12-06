use std::slice::Iter;

use advent_of_code_2024::task_argument;

fn main() {
    task_argument(task1, task2);
}

fn get_input() -> (Vec<Vec<i32>>, Vec<(i32, i32)>) {
    let mut rules: Vec<String> = std::fs::read_to_string("inputs/day5.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    let spacer_index = rules
        .iter()
        .enumerate()
        .find(|e| !e.1.contains("|"))
        .unwrap()
        .0;

    let mut updates = rules.split_off(spacer_index);
    updates.remove(0); // empty line between rules and updates

    let rules: Vec<_> = rules
        .into_iter()
        .map(|input| {
            let mut iterator = input.split("|");
            (
                iterator.next().unwrap().parse::<i32>().unwrap(),
                iterator.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    let updates: Vec<_> = updates
        .into_iter()
        .map(|input| {
            input
                .split(",")
                .map(|val| val.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    (updates, rules)
}

fn task1() {
    let (updates, rules) = get_input();
    let res = solve1(rules, updates);
    println!("{}", res);
}

fn solve1(rules: Vec<(i32, i32)>, updates: Vec<Vec<i32>>) -> i32 {
    updates
        .iter()
        .filter(|update| check_update(update, &rules))
        .map(|update| update.get(update.len() / 2).unwrap())
        .sum()
}

fn check_update(update: &[i32], rules: &[(i32, i32)]) -> bool {
    update.iter().enumerate().all(|(index, item)| {
        let to_check = &update[index + 1..];
        to_check
            .iter()
            .all(|item2| !invalid_order(*item, *item2, rules))
    })
}

fn invalid_order(a: i32, b: i32, rules: &[(i32, i32)]) -> bool {
    rules.iter().any(|rule| {
        rule.1 == a && rule.0 == b //the elements are in wrong places
    })
}

fn task2() {
    let (updates, rules) = get_input();
    let res = solve2(rules, updates);
    println!("{}", res);
}

fn solve2(rules: Vec<(i32, i32)>, mut updates: Vec<Vec<i32>>) -> i32 {
    updates
        .iter_mut()
        .map(|update| {
            let fixed = fix_update(update, &rules);
            (fixed, update)
        })
        .filter(|(fixed, _)| *fixed)
        .map(|(_, item)| item.get(item.len() / 2).unwrap())
        .sum()
}

fn fix_update(update: &mut [i32], rules: &[(i32, i32)]) -> bool {
    let mut fixed = false;
    for index in 0..(update.len()) {
        while let Some((index2, _item2)) = update[index + 1..]
            .iter()
            .enumerate()
            .find(|(_, item2)| invalid_order(update[index], **item2, rules))
        {
            fixed = true;
            let actual_index2 = index + 1 + index2;
            update.swap(index, actual_index2);
        }
    }
    fixed
}

#[cfg(test)]
mod tests {
    use crate::{check_update, fix_update, solve1, solve2};

    #[test]
    fn test_task1() {
        let (rules, updates) = get_data();

        updates.clone().into_iter().for_each(|update| {
            let res1 = check_update(&update, &rules);
            println!("res {}", res1);
        });

        let score = solve1(rules.clone(), updates);
        println!("score: {}", score);
    }

    #[test]
    fn test_task2() {
        let (rules, updates) = get_data();
        let score = solve2(rules, updates);
        println!("score: {}", score);
    }

    #[test]
    fn fix2() {
        let (rules, mut updates) = get_data();
        updates.iter_mut().for_each(|item| {
            let fixed = fix_update(item, &rules);
            println!("Fixed: {:?} {}", item, fixed)
        });
    }

    fn get_data() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
        let rules = [
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ]
        .to_vec();

        let updates = [
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ]
        .to_vec();
        (rules, updates)
    }
}
