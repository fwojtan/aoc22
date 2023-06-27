use std::collections::HashSet;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day03;

impl Solution for Day03 {
    type ParsedInput = Vec<(Vec<u8>, HashSet<u8>)>;

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

    fn part_two(_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
}

fn to_priority(bytes: u8) -> u8 {
    if bytes >= 97 {
        bytes - 96
    } else {
        bytes - 38
    }
}

fn find_matching_item(rucksack: &(Vec<u8>, HashSet<u8>)) -> u64 {
    for item in &rucksack.0 {
        if rucksack.1.contains(&item) {
            return *item as u64;
        }
    }
    0
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
