use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day01;

impl Solution for Day01 {
    type ParsedInput = Vec<Vec<u32>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // You can leave this as-is if you want to handle the input differently for each part.
        // Alternatively, you can parse the input into two completely separate structs and pass
        // them through together in a tuple.
        let mut input = vec![];
        let mut elf = vec![];

        for line in input_lines.lines() {
            if line.is_empty() {
                input.push(elf);
                elf = vec![];
            } else {
                elf.push(line.parse::<u32>().unwrap())
            }
        }
        input
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        input
            .iter()
            .map(|elf| elf.iter().sum::<u32>())
            .max()
            .unwrap()
            .to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        let mut results = vec![0, 0, 0];
        let elves = input
            .iter()
            .map(|elf| elf.iter().sum::<u32>())
            .collect::<Vec<u32>>();
        for elf in elves {
            if elf > results[0] {
                println!("{} is bigger than {}", elf, results[0]);
                results.remove(0);
                results.push(elf);
                results.sort();
                println!("Results: {:?}", results);
            }
        }
        results.iter().sum::<u32>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_part1_case1() {
        assert_eq!(Day01::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(Day01::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day01_both_case1() {
        assert_eq!(Day01::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
