// Example simple puzzle to test you've installed correctly.
// This will only be run if you specify to run day 0 specifically.  Running all days will skip this script.

// This (fictional) puzzle input consists of lines containing 2 numbers each.
// For Part 1, we're asked to find the sum of all the numbers in the list.
// For Part 2, we're asked to find the square of the difference between the two numbers in each line,
// then return the sum of those.

// When run with `cargo run 0`, the calling code in main.rs will load the input in the file inputs/0
// and pass that through as the input to the Day00 solution here as a single &str.
// This solution (as with all DayXX templates in this repo) returns two Strings, which will be printed
// out to terminal following the labels "Part 1:" and "Part 2: respectively".
// You can also run with `--bench` to benchmark the different parts of the solution.

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day00;

impl Solution for Day00 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.to_string()
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        format!("{}", input.lines().map(sum_numbers_in_line).sum::<i32>())
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        format!(
            "{}",
            input.lines().map(square_difference_in_line).sum::<i32>()
        )
    }
}

fn sum_numbers_in_line(line: &str) -> i32 {
    line.split(", ")
        .map(|number| number.parse::<i32>().expect("Couldn't parse"))
        .sum::<i32>()
}

fn square_difference_in_line(line: &str) -> i32 {
    let numbers = line
        .split(", ")
        .map(|number| number.parse::<i32>().expect("Couldn't parse"))
        .collect::<Vec<i32>>();
    assert_eq!(numbers.len(), 2);
    (numbers.first().unwrap() - numbers.get(1).unwrap()).pow(2)
}

// The template per-day files also come with template UTs.  Most Advent of Code puzzles
// have one or more examples during the puzzle page which can be useful to run your code
// against.

// There are examples here to test just the output for the first or second part of the
// the puzzle, or both together.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day00_part1_case1() {
        assert_eq!(
            Day00::solve_part_one(
                "1, 2
4, 3"
            ),
            "10".to_string()
        )
    }

    #[test]
    fn check_day00_part2_case1() {
        assert_eq!(
            Day00::solve_part_two(
                "1, 2
4, 3"
            ),
            "2".to_string()
        )
    }

    #[test]
    fn check_day00_both_case1() {
        assert_eq!(
            Day00::solve(
                "1, 2
40, 30",
                false
            ),
            ("73".to_string(), "101".to_string())
        )
    }
}
