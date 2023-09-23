use std::{cmp::max, collections::HashSet};

use nalgebra::{vector, Vector2};

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day09;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MoveInstruction {
    Up,
    Down,
    Left,
    Right,
}

impl MoveInstruction {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "U" => Some(Self::Up),
            "D" => Some(Self::Down),
            "L" => Some(Self::Left),
            "R" => Some(Self::Right),
            _ => None,
        }
    }

    fn to_vector(&self) -> Vector2<i32> {
        match self {
            Self::Up => vector![0, 1],
            Self::Down => vector![0, -1],
            Self::Left => vector![-1, 0],
            Self::Right => vector![1, 0],
        }
    }
}

impl Solution for Day09 {
    type ParsedInput = Vec<MoveInstruction>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // You can leave this as-is if you want to handle the input differently for each part.
        // Alternatively, you can parse the input into two completely separate structs and pass
        // them through together in a tuple.
        let mut instruction_list = Vec::new();
        for line in input_lines.lines() {
            let mut split = line.split_whitespace();
            let instruction = MoveInstruction::from_str(split.next().unwrap()).unwrap();
            let distance = split.next().unwrap().parse::<i32>().unwrap();
            for _i in 0..distance {
                instruction_list.push(instruction.clone());
            }
        }
        instruction_list
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        let mut visited_locations: HashSet<Vector2<i32>> = HashSet::new();
        let mut knots: Vec<Vector2<i32>> = vec![vector![0, 0]; 2];
        visited_locations.insert(knots[1].clone());

        for instruction in input.iter() {
            knots[0] += instruction.to_vector();
            // println!("Moved Head H: {:?}, T: {:?}", head_pos, tail_pos);
            if !touching(&knots[1], &knots[0]) {
                move_to_catch(&mut knots, 1);
                // println!("Moved Tail H: {:?}, T: {:?}", head_pos, tail_pos);
            }
            visited_locations.insert(knots[1].clone());
        }

        visited_locations.len().to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        let mut visited_locations: HashSet<Vector2<i32>> = HashSet::new();
        let mut knots: Vec<Vector2<i32>> = vec![vector![0, 0]; 10];
        visited_locations.insert(knots[9].clone());

        for instruction in input.iter() {
            knots[0] += instruction.to_vector();
            for i in 1..10 {
                if !touching(&knots[i], &knots[i - 1]) {
                    move_to_catch(&mut knots, i);
                }
            }
            visited_locations.insert(knots[9].clone());
        }

        visited_locations.len().to_string()
    }
}

fn touching(vec1: &Vector2<i32>, vec2: &Vector2<i32>) -> bool {
    ((vec1.x - 1)..(vec1.x + 2)).contains(&vec2.x) && ((vec1.y - 1)..(vec1.y + 2)).contains(&vec2.y)
}

fn move_to_catch(knots: &mut Vec<Vector2<i32>>, idx: usize) {
    let vec2 = knots[idx - 1];
    let x_move = (vec2.x - knots[idx].x) / max((vec2.x - knots[idx].x).abs(), 1);
    let y_move = (vec2.y - knots[idx].y) / max((vec2.y - knots[idx].y).abs(), 1);
    knots[idx].x += x_move;
    knots[idx].y += y_move;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day09_part1_case1() {
        assert_eq!(
            Day09::solve_part_one(
                r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            ),
            "13".to_string()
        )
    }

    #[test]
    fn check_day09_part2_case1() {
        assert_eq!(Day09::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day09_both_case1() {
        assert_eq!(Day09::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
