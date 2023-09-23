use std::collections::HashSet;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day06;

impl Solution for Day06 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // You can leave this as-is if you want to handle the input differently for each part.
        // Alternatively, you can parse the input into two completely separate structs and pass
        // them through together in a tuple.
        input_lines.to_string()
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        find_unique(4, input)
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        find_unique(14, input)
    }
}

fn find_unique(num: usize, input: &mut String) -> String {
    let mut set = HashSet::new();
    (num + input
        .as_bytes()
        .windows(num)
        .position(|w| {
            let mut dupe = false;
            for i in w {
                if set.contains(i) {
                    dupe = true;
                }
                set.insert(i);
            }
            set.clear();
            !dupe
        })
        .unwrap())
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day06_part1_case1() {
        assert_eq!(Day06::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day06_part2_case1() {
        assert_eq!(Day06::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day06_both_case1() {
        assert_eq!(Day06::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
