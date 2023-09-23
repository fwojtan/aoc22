use std::collections::HashSet;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day03;

impl Solution for Day03 {
    type ParsedInput = Vec<(HashSet<u8>, HashSet<u8>)>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut input = vec![];
        for line in input_lines.lines() {
            let splitline = line.split_at(line.len() / 2);
            input.push((
                splitline.0.bytes().map(to_priority).collect(),
                splitline.1.bytes().map(to_priority).collect(),
            ))
        }
        input
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        input
            .iter()
            .map(find_matching_item)
            .sum::<u64>()
            .to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        input
            .chunks(3)
            .map(find_group_item)
            .sum::<u64>()
            .to_string()
    }
}

fn to_priority(bytes: u8) -> u8 {
    if bytes >= 97 {
        bytes - 96
    } else {
        bytes - 38
    }
}

fn find_matching_item(rucksack: &(HashSet<u8>, HashSet<u8>)) -> u64 {
    for item in &rucksack.0 {
        if rucksack.1.contains(&item) {
            return *item as u64;
        }
    }
    0
}

fn find_group_item(rucksacks: &[(HashSet<u8>, HashSet<u8>)]) -> u64 {
    let mut set_one = HashSet::<u8>::from_iter(rucksacks[0].0.clone().into_iter());
    set_one.extend(rucksacks[0].1.iter()); // this could be refactored out given I split this up in parsing for part one convenience...
    let mut set_two = HashSet::<u8>::from_iter(rucksacks[1].0.clone().into_iter());
    set_two.extend(rucksacks[1].1.iter());
    let intersection = set_one.intersection(&set_two);
    let mut intersected_set = HashSet::new();
    intersected_set.extend(intersection.into_iter());
    let mut set_three = HashSet::<u8>::from_iter(rucksacks[2].0.clone().into_iter());
    set_three.extend(rucksacks[2].1.iter());
    *intersected_set.intersection(&set_three).next().unwrap() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(Day03::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(Day03::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day03_both_case1() {
        assert_eq!(Day03::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
