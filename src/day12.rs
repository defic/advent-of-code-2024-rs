use std::{
    collections::{btree_map::Keys, HashMap},
    time::Instant,
};

use advent_of_code_2024::task_argument_with_input;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let time = Instant::now();
    let res = task_argument_with_input("inputs/12.txt", task1, task2);
    println!("Res: {}, Took: {:?}", res, time.elapsed());
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct RegionId(char, usize, usize); //first square coordinate is id

struct Region {
    id: RegionId,
    areas: Vec<(usize, usize)>,
}

struct RegionMap {
    regions: Vec<Vec<Option<RegionId>>>,
}

impl RegionMap {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            regions: vec![vec![None; width]; height],
        }
    }

    fn replace(&mut self, boss: RegionId, scrub: RegionId) {
        self.regions.iter_mut().for_each(|line| {
            line.iter_mut()
                .filter(|x| **x == Some(scrub))
                .for_each(|e| *e = Some(boss));
        });
    }

    fn add(&mut self, y: usize, x: usize, id: RegionId) {
        self.regions[y][x] = Some(id);
    }

    fn get_region_char(&self, char: &char, y: i32, x: i32) -> Option<RegionId> {
        if y < 0 || x < 0 {
            return None;
        }
        let res = self.regions[y as usize][x as usize]?;

        if res.0 != *char {
            return None;
        }
        Some(res)
    }
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

fn task1(input: String) -> String {
    let input = parse_input(input);

    let (region_map, plant_regions) = construct_regions(&input);

    let mut price = 0;

    for (id, cells) in plant_regions {
        let perimeter = calculate_perimeter(&cells);
        let area = cells.len();
        price += perimeter * area;
    }

    price.to_string()
}

fn calculate_perimeter(region: &[(usize, usize)]) -> usize {
    region
        .iter()
        .map(|e| {
            4 - region
                .iter()
                .filter(|k| e != *k && e.0.abs_diff(k.0) + e.1.abs_diff(k.1) == 1)
                .count()
        })
        .sum()
}

fn construct_regions(map: &[Vec<char>]) -> (RegionMap, HashMap<RegionId, Vec<(usize, usize)>>) {
    let mut region_map = RegionMap::new(map.len(), map.first().unwrap().len());
    let mut plant_regions: HashMap<RegionId, Vec<(usize, usize)>> = HashMap::new();

    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            let up_y = y as i32 - 1;
            let up = region_map.get_region_char(char, up_y, x as i32);
            let left_x = x as i32 - 1;
            let left = region_map.get_region_char(char, y as i32, left_x);

            match (up, left) {
                (None, None) => {
                    let new = RegionId(*char, y, x);
                    region_map.add(y, x, new);
                    plant_regions.insert(new, vec![(y, x)]);
                }
                (Some(region_id), None) | (None, Some(region_id)) => {
                    region_map.add(y, x, region_id);
                    let vec = plant_regions.get_mut(&region_id).unwrap();
                    vec.push((y, x));
                }
                (Some(boss), Some(scrub)) => {
                    if boss != scrub {
                        region_map.replace(boss, scrub);
                        let list = plant_regions.remove(&scrub).unwrap();
                        let bosslist = plant_regions.get_mut(&boss).unwrap();
                        bosslist.extend(list);
                    }
                    region_map.add(y, x, boss);
                    plant_regions.get_mut(&boss).unwrap().push((y, x));
                }
            }
        }
    }

    (region_map, plant_regions)
}

fn task2(input: String) -> String {
    let input = parse_input(input);

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::task1;

    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_task1() {
        task1(INPUT.to_string());
    }
}
