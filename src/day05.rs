use std::collections::HashMap;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day05;

struct CrateMove {
    quantity: usize,
    orig: usize,
    dest: usize,
}

pub struct Puzzle {
    instructions: Vec<CrateMove>,
    stacks: CrateStacks,
}

pub struct CrateStacks {
    stacks: HashMap<usize, Vec<char>>,
}

impl CrateStacks {
    fn _single_move(&mut self, from: usize, to: usize) {
        let from_stack = self.stacks.get_mut(&from).unwrap();
        if let Some(value) = from_stack.pop() {
            let to_stack = self.stacks.get_mut(&to).unwrap();
            to_stack.push(value);
        }
    }

    fn move_crates(&mut self, instruction: &CrateMove) {
        for _ in 0..instruction.quantity {
            self._single_move(instruction.orig, instruction.dest);
        }
    }
}

impl Puzzle {
    fn execute_instructions(&mut self) {
        for instruction in &self.instructions {
            self.stacks.move_crates(&instruction);
        }
    }

    fn top_crates(&self) -> String {
        let mut crates = String::new();
        for i in 1..10 {
            let stack = self.stacks.stacks.get(&i).unwrap();
            if let Some(top) = stack.last() {
                crates.push(*top);
            }
        }
        crates
    }

    fn _print_crates(&self) {
        for i in 1..10 {
            let stack = self.stacks.stacks.get(&i).unwrap();
            println!("{}: {:?}", i, stack);
        }
    }
}

impl Solution for Day05 {
    type ParsedInput = Puzzle;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let reg = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let input_lines = input_lines.lines();
        let stacks_input: Vec<&str> = input_lines.clone().take(8).collect();
        let mut stacks = HashMap::new();
        for i in 1..10 {
            stacks.insert(i, vec![]);
        }
        for layer in stacks_input.iter().rev() {
            let layer = layer.chars().collect::<Vec<_>>();
            for i in 0..9 {
                let idx = 4 * i + 1;
                let value = layer[idx];
                if !value.is_whitespace() {
                    let stack = stacks.get_mut(&(i + 1)).unwrap();
                    stack.push(value)
                }
            }
        }
        let instructions = input_lines
            .skip(10)
            .map(|line| parse_crate_move(line, &reg))
            .collect::<Vec<_>>();
        let stacks = CrateStacks { stacks };
        Puzzle {
            instructions,
            stacks,
        }
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        input.execute_instructions();
        input.top_crates()
    }

    fn part_two(_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
}

fn parse_crate_move(instruction: &str, reg: &regex::Regex) -> CrateMove {
    let caps = reg.captures(instruction).unwrap();
    let mut caps = caps.iter();
    let quantity = caps
        .nth(1)
        .unwrap()
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    let from = caps
        .next()
        .unwrap()
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    let to = caps
        .next()
        .unwrap()
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    CrateMove {
        quantity: quantity,
        orig: from,
        dest: to,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(Day05::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day05_part2_case1() {
        assert_eq!(Day05::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day05_both_case1() {
        assert_eq!(
            Day05::solve(
                r"                    
                                        
                                        
                                                        
                                                        
                    [D]    
                    [N] [C]    
                    [Z] [M] [P]
                    1   2   3 
                    
                    move 1 from 2 to 1
                    move 3 from 1 to 3
                    move 2 from 2 to 1
                    move 1 from 1 to 2",
                false
            ),
            ("CMZ".to_string(), "0".to_string())
        )
    }
}
