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

    fn part_one(_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part one
        0.to_string()
    }

    fn part_two(_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
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
