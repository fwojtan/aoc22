use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day02;

impl Solution for Day02 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // You can leave this as-is if you want to handle the input differently for each part.
        // Alternatively, you can parse the input into two completely separate structs and pass
        // them through together in a tuple.
        input_lines.to_string()
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        input.lines().map(score_round).sum::<u32>().to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        input.lines().map(score_by_outcome).sum::<u32>().to_string()
    }
}

fn score_round(round: &str) -> u32 {
    match round {
        "A X" => 4, // 1 + 3
        "B X" => 1, // 1 + 0
        "C X" => 7, // 1 + 6
        "A Y" => 8, // 2 + 6
        "B Y" => 5, // 2 + 3
        "C Y" => 2, // 2 + 0
        "A Z" => 3, // 3 + 0
        "B Z" => 9, // 3 + 6
        "C Z" => 6, // 3 + 3
        _ => 0,
    }
}

// A beats C
// C beats B
// B beats A

fn score_by_outcome(round: &str) -> u32 {
    match round {
        "A X" => 3, // lose -> C -> 0 + 3
        "B X" => 1, // lose -> A -> 0 + 1
        "C X" => 2, // lose -> B -> 0 + 2
        "A Y" => 4, // draw -> A -> 3 + 1
        "B Y" => 5, // draw -> B -> 3 + 2
        "C Y" => 6, // draw -> C -> 3 + 3
        "A Z" => 8, // win  -> B -> 6 + 2
        "B Z" => 9, // win  -> C -> 6 + 3
        "C Z" => 7, // win  -> A -> 6 + 1
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day02_part1_case1() {
        let input = r"A Y
B X
C Z";
        assert_eq!(Day02::solve_part_one(input), "15".to_string())
    }

    #[test]
    fn check_day02_part2_case1() {
        assert_eq!(Day02::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day02_both_case1() {
        let input = r"A Y
B X
C Z";
        assert_eq!(
            Day02::solve(input, false),
            ("15".to_string(), "0".to_string())
        )
    }
}
