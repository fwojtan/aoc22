use regex::Regex;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day04;

pub struct Elf {
    upper: u32,
    lower: u32,
}
pub struct Pair {
    a: Elf,
    b: Elf,
}

impl Pair {
    fn range_contains_range(&self) -> bool {
        (self.a.upper <= self.b.upper && self.a.lower >= self.b.lower)
            || (self.b.upper <= self.a.upper && self.b.lower >= self.a.lower)
    }

    fn ranges_overlap(&self) -> bool {
        (self.a.upper <= self.b.upper && self.a.upper >= self.b.lower)
            || (self.a.lower <= self.b.upper && self.a.lower >= self.b.lower)
            || (self.a.lower < self.b.lower && self.a.upper > self.b.upper)
            || (self.a.lower > self.b.lower && self.a.upper < self.b.upper)
    }
}

impl Solution for Day04 {
    type ParsedInput = Vec<Pair>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut input = vec![];
        for line in input_lines.lines() {
            let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
            let caps = re
                .captures(line)
                .unwrap()
                .iter()
                .skip(1)
                .map(|hit| hit.unwrap().as_str().parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let elf_a = Elf {
                lower: caps[0],
                upper: caps[1],
            };
            let elf_b = Elf {
                lower: caps[2],
                upper: caps[3],
            };
            let pair = Pair { a: elf_a, b: elf_b };
            input.push(pair);
        }
        input
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        let mut count = 0;
        for pair in input {
            if pair.range_contains_range() {
                count += 1;
            }
        }
        count.to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        let mut count = 0;
        for pair in input {
            if pair.ranges_overlap() {
                count += 1;
            }
        }
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day04_part1_case1() {
        assert_eq!(Day04::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day04_part2_case1() {
        assert_eq!(Day04::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day04_both_case1() {
        let input = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(
            Day04::solve(input, false),
            ("2".to_string(), "4".to_string())
        )
    }
}
