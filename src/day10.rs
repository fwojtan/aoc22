use crate::Solution;
use std::fmt::Write;

#[derive(Clone, Debug)]
pub struct Day10;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Op {
    Addx(i32),
    NoOp,
}

#[derive(Clone, Debug)]
pub struct TickState {
    #[allow(dead_code)]
    active_op: Op,
    register_val: i32,
}

impl Solution for Day10 {
    type ParsedInput = Vec<TickState>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // You can leave this as-is if you want to handle the input differently for each part.
        // Alternatively, you can parse the input into two completely separate structs and pass
        // them through together in a tuple.
        let mut program_state = Vec::new();
        let mut register_val = 1;
        for line in input_lines.lines() {
            let mut split = line.split_whitespace();
            match split.next().unwrap() {
                "noop" => program_state.push(TickState {
                    active_op: Op::NoOp,
                    register_val,
                }),
                "addx" => {
                    let op_val = split.next().unwrap().parse::<i32>().unwrap();
                    for _ in 0..2 {
                        program_state.push(TickState {
                            active_op: Op::Addx(op_val),
                            register_val,
                        });
                    }
                    register_val += op_val;
                }
                _ => panic!("Unknown op"),
            };
        }
        program_state
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        // for (i, state) in input.iter().enumerate() {
        //     println!(
        //         "Cycle: {}, Operation: {:?}, Value: {}",
        //         i, state.active_op, state.register_val
        //     );
        // }
        let mut signal_snapshots = vec![];
        for i in 0..6 {
            let idx = 20 + i * 40;
            signal_snapshots.push(input[idx - 1].register_val * (idx as i32))
        }
        signal_snapshots.iter().sum::<i32>().to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        let mut screen = vec!["."; 240];

        for (i, (tick_state, pixel)) in input.iter().zip(screen.iter_mut()).enumerate() {
            let sprite_coord = tick_state.register_val;
            let draw_coord = i % 40;
            // println!(
            //     "Cycle: {}, Draw Coord: {}, Sprite: {}-{}, Instr: {:?}",
            //     i,
            //     draw_coord,
            //     sprite_coord - 1,
            //     sprite_coord + 1,
            //     tick_state.active_op
            // );
            if (draw_coord as i32) <= sprite_coord + 1 && (draw_coord as i32) >= sprite_coord - 1 {
                *pixel = "#";
            } else {
                *pixel = ".";
            }
        }

        let mut result = "\n".to_string();
        for i in 0..6 {
            let line = &screen[i * 40..(i + 1) * 40];
            writeln!(result, "{}", line.join("")).unwrap();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day10_part1_case1() {
        assert_eq!(
            Day10::solve_part_one(
                r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
            ),
            "13140".to_string()
        )
    }

    #[test]
    fn check_day10_part2_case1() {
        assert_eq!(Day10::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day10_both_case1() {
        assert_eq!(Day10::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
